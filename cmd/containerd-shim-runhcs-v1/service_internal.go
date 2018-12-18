package main

import (
	"context"

	"github.com/containerd/containerd/errdefs"
	"github.com/containerd/containerd/runtime/v2/task"
	google_protobuf1 "github.com/gogo/protobuf/types"
)

// TaskTarget is an inderection between the container you want to create and the
// target you would like to create it in. There are three classes of targets:
//
// 1. The Windows host. This target can be used to create V1 Argon (on RS1, RS3,
// RS4 hosts), V1 Xenon (on RS1, RS3, RS4 hosts), and V2 Argon (on RS5+ hosts).
//
// 2. The Windows VM. This target can be used to create V1 Argons (on RS1, RS3,
// RS5 guests), and V2 Argons (on RS5+ guests).
//
// 3. The Linux VM. This target can be used to create Linux Containers on RS5+
// hosts.
type TaskTarget interface {
	CreateContainer() error
	GetContainer(id string) error
}

type Container interface {
	State() (*task.StateResponse, error)
	Start() (*task.StartResponse, error)
	Delete() (*task.DeleteResponse, error)
}

func (s *service) stateInternal(ctx context.Context, req *task.StateRequest) (*task.StateResponse, error) {
	return nil, errdefs.ErrNotImplemented
}

func (s *service) createInternal(ctx context.Context, req *task.CreateTaskRequest) (*task.CreateTaskResponse, error) {
	return nil, errdefs.ErrNotImplemented
}

func (s *service) startInternal(ctx context.Context, req *task.StartRequest) (*task.StartResponse, error) {
	return nil, errdefs.ErrNotImplemented
}

func (s *service) deleteInternal(ctx context.Context, req *task.DeleteRequest) (*task.DeleteResponse, error) {
	return nil, errdefs.ErrNotImplemented
}

func (s *service) pidsInternal(ctx context.Context, req *task.PidsRequest) (*task.PidsResponse, error) {
	return nil, errdefs.ErrNotImplemented
}

func (s *service) pauseInternal(ctx context.Context, req *task.PauseRequest) (*google_protobuf1.Empty, error) {
	return nil, errdefs.ErrNotImplemented
}

func (s *service) resumeInternal(ctx context.Context, req *task.ResumeRequest) (*google_protobuf1.Empty, error) {
	return nil, errdefs.ErrNotImplemented
}

func (s *service) checkpointInternal(ctx context.Context, req *task.CheckpointTaskRequest) (*google_protobuf1.Empty, error) {
	return nil, errdefs.ErrNotImplemented
}

func (s *service) killInternal(ctx context.Context, req *task.KillRequest) (*google_protobuf1.Empty, error) {
	return nil, errdefs.ErrNotImplemented
}

func (s *service) execInternal(ctx context.Context, req *task.ExecProcessRequest) (*google_protobuf1.Empty, error) {
	return nil, errdefs.ErrNotImplemented
}

func (s *service) resizePtyInternal(ctx context.Context, req *task.ResizePtyRequest) (*google_protobuf1.Empty, error) {
	return nil, errdefs.ErrNotImplemented
}

func (s *service) closeIOInternal(ctx context.Context, req *task.CloseIORequest) (*google_protobuf1.Empty, error) {
	return nil, errdefs.ErrNotImplemented
}

func (s *service) updateInternal(ctx context.Context, req *task.UpdateTaskRequest) (*google_protobuf1.Empty, error) {
	return nil, errdefs.ErrNotImplemented
}

func (s *service) waitInternal(ctx context.Context, req *task.WaitRequest) (*task.WaitResponse, error) {
	return nil, errdefs.ErrNotImplemented
}

func (s *service) statsInternal(ctx context.Context, req *task.StatsRequest) (*task.StatsResponse, error) {
	return nil, errdefs.ErrNotImplemented
}

func (s *service) connectInternal(ctx context.Context, req *task.ConnectRequest) (*task.ConnectResponse, error) {
	return nil, errdefs.ErrNotImplemented
}

func (s *service) shutdownInternal(ctx context.Context, req *task.ShutdownRequest) (*google_protobuf1.Empty, error) {
	return nil, errdefs.ErrNotImplemented
}
