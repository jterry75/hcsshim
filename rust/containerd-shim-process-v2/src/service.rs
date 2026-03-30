// Copyright (c) Microsoft Corporation. All rights reserved.

use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::{Mutex, Notify};
use tonic::{Request, Response, Status};
use tracing::{error, info, warn};

use computecore::HcsSystemHandle;
use computecore::schema::containers::ContainerSettings;
use containerd_api::runtime::sandbox::v1::sandbox_server::Sandbox;
use containerd_api::runtime::sandbox::v1::*;
use containerd_api::task::v3::task_server::Task;
use containerd_api::task::v3::*;

use crate::oci;

// ---------------------------------------------------------------------------
// Internal state types
// ---------------------------------------------------------------------------

/// Tracks the lifecycle state of a sandbox (pod).
#[derive(Debug)]
enum SandboxStatus {
    Created,
    Running,
    Stopped { exit_status: u32 },
}

/// State for a single pod sandbox (there is at most one per shim instance).
#[derive(Debug)]
struct SandboxState {
    id: String,
    status: SandboxStatus,
    bundle_path: String,
    system: Option<HcsSystemHandle>,
}

/// Tracks the lifecycle state of a task (container).
#[derive(Debug)]
enum TaskStatus {
    Created,
    Running { pid: u32 },
    Stopped { pid: u32, exit_status: u32 },
}

/// State for a single task / container.
#[derive(Debug)]
struct TaskState {
    id: String,
    bundle_path: String,
    status: TaskStatus,
    /// The HCS container configuration document. Kept around so it can be
    /// updated (e.g. by `update` to merge new resource limits or mounts)
    /// and re-serialized.
    #[allow(unused)] // TODO: Remove once `update` is implemented.
    document: ContainerSettings,
    system: Option<HcsSystemHandle>,
}

// ---------------------------------------------------------------------------
// ShimService
// ---------------------------------------------------------------------------

/// Unified service implementation for process-isolated Windows containers.
///
/// Implements both the containerd Task v3 and Sandbox v1 gRPC services.
///
/// **State model:**
/// - `sandbox`: `Option<SandboxState>` — at most one sandbox per shim.
/// - `tasks`: `HashMap<String, TaskState>` — one entry per container.
pub struct ShimService {
    id: String,
    namespace: String,
    shutdown_signal: Arc<Notify>,
    sandbox: Mutex<Option<SandboxState>>,
    tasks: Mutex<HashMap<String, TaskState>>,
}

impl ShimService {
    pub fn new(id: String, namespace: String) -> Self {
        Self {
            id,
            namespace,
            shutdown_signal: Arc::new(Notify::new()),
            sandbox: Mutex::new(None),
            tasks: Mutex::new(HashMap::new()),
        }
    }

    /// Signal the service to shut down gracefully.
    pub fn signal_shutdown(&self) {
        info!(id = %self.id, namespace = %self.namespace, "signaling shutdown");
        self.shutdown_signal.notify_waiters();
    }

    /// Wait until a shutdown signal is received.
    #[allow(unused)]
    pub async fn wait_for_shutdown(&self) {
        self.shutdown_signal.notified().await;
    }
}

// ---------------------------------------------------------------------------
// Task v3 implementation
// ---------------------------------------------------------------------------

#[tonic::async_trait]
impl Task for ShimService {
    async fn state(
        &self,
        request: Request<StateRequest>,
    ) -> Result<Response<StateResponse>, Status> {
        let req = request.into_inner();
        let tasks = self.tasks.lock().await;
        let task = tasks
            .get(&req.id)
            .ok_or_else(|| Status::not_found(format!("task {} not found", req.id)))?;

        let (status, pid, exit_status) = match &task.status {
            TaskStatus::Created => (containerd_api::v1::types::Status::Created.into(), 0, 0),
            TaskStatus::Running { pid } => {
                (containerd_api::v1::types::Status::Running.into(), *pid, 0)
            }
            TaskStatus::Stopped { pid, exit_status } => (
                containerd_api::v1::types::Status::Stopped.into(),
                *pid,
                *exit_status,
            ),
        };

        Ok(Response::new(StateResponse {
            id: task.id.clone(),
            bundle: task.bundle_path.clone(),
            pid,
            status,
            exit_status,
            ..Default::default()
        }))
    }

    async fn create(
        &self,
        request: Request<CreateTaskRequest>,
    ) -> Result<Response<CreateTaskResponse>, Status> {
        let req = request.into_inner();
        info!(id = %req.id, bundle = %req.bundle, "create task");

        // Load the OCI spec from the bundle.
        let spec = oci_spec::runtime::Spec::load(&req.bundle).map_err(|e| {
            error!(error = %e, "failed to load OCI spec");
            Status::invalid_argument(format!("failed to load OCI spec: {e}"))
        })?;

        // Build the HCS document from the OCI spec.
        let document = oci::create_container_document(&req.id, &spec).map_err(|e| {
            error!(error = %e, "failed to create container document");
            Status::internal(format!("failed to create container document: {e}"))
        })?;

        // Serialize the document for the HCS call.
        let doc_json = serde_json::to_string(&document).map_err(|e| {
            error!(error = %e, "failed to serialize container document");
            Status::internal(format!("failed to serialize container document: {e}"))
        })?;

        // Create the compute system via HCS.
        let system = computecore::create_compute_system(&req.id, &doc_json)
            .await
            .map_err(|e| {
                error!(error = %e, "HcsCreateComputeSystem failed");
                Status::internal(format!("HcsCreateComputeSystem failed: {e}"))
            })?;

        let task_state = TaskState {
            id: req.id.clone(),
            bundle_path: req.bundle.clone(),
            status: TaskStatus::Created,
            document,
            system: Some(system),
        };

        let mut tasks = self.tasks.lock().await;
        if tasks.contains_key(&req.id) {
            return Err(Status::already_exists(format!(
                "task {} already exists",
                req.id
            )));
        }
        tasks.insert(req.id.clone(), task_state);

        Ok(Response::new(CreateTaskResponse { pid: 0 }))
    }

    async fn start(
        &self,
        request: Request<StartRequest>,
    ) -> Result<Response<StartResponse>, Status> {
        let req = request.into_inner();
        info!(id = %req.id, "start task");

        let mut tasks = self.tasks.lock().await;
        let task = tasks
            .get_mut(&req.id)
            .ok_or_else(|| Status::not_found(format!("task {} not found", req.id)))?;

        if !matches!(task.status, TaskStatus::Created) {
            return Err(Status::failed_precondition(format!(
                "task {} is not in created state",
                req.id
            )));
        }

        // Start the compute system.
        let system = task
            .system
            .as_ref()
            .ok_or_else(|| Status::internal("task has no compute system handle"))?;

        computecore::start_compute_system(system, None)
            .await
            .map_err(|e| {
                error!(error = %e, "HcsStartComputeSystem failed");
                Status::internal(format!("HcsStartComputeSystem failed: {e}"))
            })?;

        // TODO: Get actual PID from the compute system init process.
        let pid = 0u32;
        task.status = TaskStatus::Running { pid };

        Ok(Response::new(StartResponse { pid }))
    }

    async fn delete(
        &self,
        request: Request<DeleteRequest>,
    ) -> Result<Response<DeleteResponse>, Status> {
        let req = request.into_inner();
        info!(id = %req.id, "delete task");

        let mut tasks = self.tasks.lock().await;
        let task = tasks
            .remove(&req.id)
            .ok_or_else(|| Status::not_found(format!("task {} not found", req.id)))?;

        let (pid, exit_status) = match &task.status {
            TaskStatus::Stopped { pid, exit_status } => (*pid, *exit_status),
            _ => (0, 0),
        };

        // The HcsSystemHandle is dropped here, which calls HcsCloseComputeSystem.

        Ok(Response::new(DeleteResponse {
            pid,
            exit_status,
            ..Default::default()
        }))
    }

    async fn pids(&self, request: Request<PidsRequest>) -> Result<Response<PidsResponse>, Status> {
        let req = request.into_inner();
        let tasks = self.tasks.lock().await;
        let task = tasks
            .get(&req.id)
            .ok_or_else(|| Status::not_found(format!("task {} not found", req.id)))?;

        let pid = match &task.status {
            TaskStatus::Running { pid } => *pid,
            TaskStatus::Stopped { pid, .. } => *pid,
            TaskStatus::Created => 0,
        };

        Ok(Response::new(PidsResponse {
            processes: vec![containerd_api::v1::types::ProcessInfo {
                pid,
                ..Default::default()
            }],
        }))
    }

    async fn pause(&self, _request: Request<PauseRequest>) -> Result<Response<()>, Status> {
        Err(Status::unimplemented(
            "pause is not supported for process-isolated containers",
        ))
    }

    async fn resume(&self, _request: Request<ResumeRequest>) -> Result<Response<()>, Status> {
        Err(Status::unimplemented(
            "resume is not supported for process-isolated containers",
        ))
    }

    async fn checkpoint(
        &self,
        _request: Request<CheckpointTaskRequest>,
    ) -> Result<Response<()>, Status> {
        Err(Status::unimplemented(
            "checkpoint is not supported for process-isolated containers",
        ))
    }

    async fn kill(&self, request: Request<KillRequest>) -> Result<Response<()>, Status> {
        let req = request.into_inner();
        info!(id = %req.id, signal = req.signal, "kill task");

        let mut tasks = self.tasks.lock().await;
        let task = tasks
            .get_mut(&req.id)
            .ok_or_else(|| Status::not_found(format!("task {} not found", req.id)))?;

        let system = task
            .system
            .as_ref()
            .ok_or_else(|| Status::internal("task has no compute system handle"))?;

        // For Windows process-isolated containers, SIGTERM (15) → graceful
        // shutdown, SIGKILL (9) → terminate.
        if req.signal == 9 {
            computecore::terminate_compute_system(system, None)
                .await
                .map_err(|e| {
                    warn!(error = %e, "HcsTerminateComputeSystem failed");
                    Status::internal(format!("HcsTerminateComputeSystem failed: {e}"))
                })?;
        } else {
            computecore::shutdown_compute_system(system, None)
                .await
                .map_err(|e| {
                    warn!(error = %e, "HcsShutDownComputeSystem failed, falling back to terminate");
                    // Fall through — caller can retry with SIGKILL.
                    Status::internal(format!("HcsShutDownComputeSystem failed: {e}"))
                })?;
        }

        let pid = match &task.status {
            TaskStatus::Running { pid } => *pid,
            _ => 0,
        };
        task.status = TaskStatus::Stopped {
            pid,
            exit_status: 0,
        };

        Ok(Response::new(()))
    }

    async fn exec(&self, _request: Request<ExecProcessRequest>) -> Result<Response<()>, Status> {
        // TODO: Implement exec by calling HcsCreateProcess on the task's system.
        Err(Status::unimplemented("exec"))
    }

    async fn resize_pty(
        &self,
        _request: Request<ResizePtyRequest>,
    ) -> Result<Response<()>, Status> {
        Err(Status::unimplemented("resize_pty"))
    }

    async fn close_io(&self, _request: Request<CloseIoRequest>) -> Result<Response<()>, Status> {
        // TODO: Close stdin pipe for the process.
        Err(Status::unimplemented("close_io"))
    }

    async fn update(&self, _request: Request<UpdateTaskRequest>) -> Result<Response<()>, Status> {
        // TODO: HcsModifyComputeSystem for resource updates.
        Err(Status::unimplemented("update"))
    }

    async fn wait(&self, request: Request<WaitRequest>) -> Result<Response<WaitResponse>, Status> {
        let req = request.into_inner();

        // Get the system handle while holding the lock briefly.
        let raw = {
            let tasks = self.tasks.lock().await;
            let task = tasks
                .get(&req.id)
                .ok_or_else(|| Status::not_found(format!("task {} not found", req.id)))?;
            match &task.status {
                TaskStatus::Stopped { exit_status, .. } => {
                    // Already stopped — return immediately.
                    return Ok(Response::new(WaitResponse {
                        exit_status: *exit_status,
                        ..Default::default()
                    }));
                }
                _ => task
                    .system
                    .as_ref()
                    .map(|s| s.as_raw())
                    .ok_or_else(|| Status::internal("task has no compute system handle"))?,
            }
        };

        // Wait outside the lock so other RPCs can proceed.
        // Reconstruct a temporary non-owning reference for the wait call.
        // SAFETY: We only read the raw isize; actual close happens via TaskState's Drop.
        let temp_handle = unsafe {
            // Create a temporary handle that we must NOT drop (it would close the real handle).
            std::mem::ManuallyDrop::new(computecore::HcsSystemHandle::from_raw(raw))
        };

        let result = computecore::wait_for_compute_system_exit(&temp_handle, u32::MAX).await;

        // Update state after wait completes.
        let exit_status = match result {
            Ok(_) => 0u32,
            Err(ref e) => {
                warn!(error = %e, "wait_for_compute_system_exit returned error");
                1u32
            }
        };

        {
            let mut tasks = self.tasks.lock().await;
            if let Some(task) = tasks.get_mut(&req.id) {
                let pid = match &task.status {
                    TaskStatus::Running { pid } => *pid,
                    _ => 0,
                };
                task.status = TaskStatus::Stopped { pid, exit_status };
            }
        }

        Ok(Response::new(WaitResponse {
            exit_status,
            ..Default::default()
        }))
    }

    async fn stats(
        &self,
        request: Request<StatsRequest>,
    ) -> Result<Response<StatsResponse>, Status> {
        let req = request.into_inner();
        let tasks = self.tasks.lock().await;
        let task = tasks
            .get(&req.id)
            .ok_or_else(|| Status::not_found(format!("task {} not found", req.id)))?;

        let system = task
            .system
            .as_ref()
            .ok_or_else(|| Status::internal("task has no compute system handle"))?;

        let _props = computecore::get_compute_system_properties(system, None)
            .await
            .map_err(|e| Status::internal(format!("failed to get properties: {e}")))?;

        // TODO: Parse the properties JSON and convert to StatsResponse.
        Err(Status::unimplemented("stats response parsing"))
    }

    async fn connect(
        &self,
        _request: Request<ConnectRequest>,
    ) -> Result<Response<ConnectResponse>, Status> {
        let pid = std::process::id();
        Ok(Response::new(ConnectResponse {
            shim_pid: pid,
            task_pid: 0,
            version: env!("CARGO_PKG_VERSION").to_string(),
        }))
    }

    async fn shutdown(&self, _request: Request<ShutdownRequest>) -> Result<Response<()>, Status> {
        info!("shutdown RPC received");
        self.signal_shutdown();
        Ok(Response::new(()))
    }
}

// ---------------------------------------------------------------------------
// Sandbox v1 implementation
// ---------------------------------------------------------------------------

#[tonic::async_trait]
impl Sandbox for ShimService {
    async fn create_sandbox(
        &self,
        request: Request<CreateSandboxRequest>,
    ) -> Result<Response<CreateSandboxResponse>, Status> {
        let req = request.into_inner();
        info!(sandbox_id = %req.sandbox_id, bundle = %req.bundle_path, "create sandbox");

        let mut sandbox = self.sandbox.lock().await;
        if sandbox.is_some() {
            return Err(Status::already_exists("sandbox already exists"));
        }

        *sandbox = Some(SandboxState {
            id: req.sandbox_id,
            status: SandboxStatus::Created,
            bundle_path: req.bundle_path,
            system: None,
        });

        Ok(Response::new(CreateSandboxResponse {}))
    }

    async fn start_sandbox(
        &self,
        request: Request<StartSandboxRequest>,
    ) -> Result<Response<StartSandboxResponse>, Status> {
        let req = request.into_inner();
        info!(sandbox_id = %req.sandbox_id, "start sandbox");

        let mut sandbox = self.sandbox.lock().await;
        let state = sandbox
            .as_mut()
            .ok_or_else(|| Status::not_found("sandbox not created"))?;

        if state.id != req.sandbox_id {
            return Err(Status::not_found(format!(
                "sandbox {} not found",
                req.sandbox_id
            )));
        }

        if !matches!(state.status, SandboxStatus::Created) {
            return Err(Status::failed_precondition(
                "sandbox is not in created state",
            ));
        }

        // For process-isolated, the sandbox itself is lightweight — it's
        // really just a bookkeeping entity. The actual compute systems are
        // created per-task. Mark as running.
        state.status = SandboxStatus::Running;

        Ok(Response::new(StartSandboxResponse {
            pid: std::process::id(),
            created_at: None,
        }))
    }

    async fn platform(
        &self,
        _request: Request<PlatformRequest>,
    ) -> Result<Response<PlatformResponse>, Status> {
        Ok(Response::new(PlatformResponse {
            platform: Some(containerd_api::types::Platform {
                os: "windows".to_string(),
                architecture: std::env::consts::ARCH.to_string(),
                variant: String::new(),
                os_version: String::new(),
            }),
        }))
    }

    async fn stop_sandbox(
        &self,
        request: Request<StopSandboxRequest>,
    ) -> Result<Response<StopSandboxResponse>, Status> {
        let req = request.into_inner();
        info!(sandbox_id = %req.sandbox_id, "stop sandbox");

        let mut sandbox = self.sandbox.lock().await;
        let state = sandbox
            .as_mut()
            .ok_or_else(|| Status::not_found("sandbox not created"))?;

        // Terminate any sandbox-level compute system if present.
        if let Some(system) = state.system.as_ref() {
            let _ = computecore::terminate_compute_system(system, None).await;
        }

        state.status = SandboxStatus::Stopped { exit_status: 0 };

        Ok(Response::new(StopSandboxResponse {}))
    }

    async fn wait_sandbox(
        &self,
        _request: Request<WaitSandboxRequest>,
    ) -> Result<Response<WaitSandboxResponse>, Status> {
        let sandbox = self.sandbox.lock().await;
        let state = sandbox
            .as_ref()
            .ok_or_else(|| Status::not_found("sandbox not created"))?;

        let exit_status = match &state.status {
            SandboxStatus::Stopped { exit_status } => *exit_status,
            _ => 0,
        };

        Ok(Response::new(WaitSandboxResponse {
            exit_status,
            ..Default::default()
        }))
    }

    async fn sandbox_status(
        &self,
        request: Request<SandboxStatusRequest>,
    ) -> Result<Response<SandboxStatusResponse>, Status> {
        let req = request.into_inner();
        let sandbox = self.sandbox.lock().await;
        let state = sandbox
            .as_ref()
            .ok_or_else(|| Status::not_found("sandbox not created"))?;

        let status_str = match &state.status {
            SandboxStatus::Created => "created",
            SandboxStatus::Running => "ready",
            SandboxStatus::Stopped { .. } => "stopped",
        };

        Ok(Response::new(SandboxStatusResponse {
            sandbox_id: req.sandbox_id,
            pid: std::process::id(),
            state: status_str.to_string(),
            info: Default::default(),
            created_at: None,
            exited_at: None,
            extra: None,
        }))
    }

    async fn ping_sandbox(
        &self,
        _request: Request<PingRequest>,
    ) -> Result<Response<PingResponse>, Status> {
        Ok(Response::new(PingResponse {}))
    }

    async fn shutdown_sandbox(
        &self,
        _request: Request<ShutdownSandboxRequest>,
    ) -> Result<Response<ShutdownSandboxResponse>, Status> {
        info!("sandbox shutdown RPC received");
        self.signal_shutdown();
        Ok(Response::new(ShutdownSandboxResponse {}))
    }

    async fn sandbox_metrics(
        &self,
        _request: Request<SandboxMetricsRequest>,
    ) -> Result<Response<SandboxMetricsResponse>, Status> {
        Err(Status::unimplemented("sandbox_metrics"))
    }
}
