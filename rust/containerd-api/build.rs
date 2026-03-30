// Copyright (c) Microsoft Corporation. All rights reserved.

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let proto_root = "proto".to_string();
    let api_base = "proto/github.com/containerd/containerd/api";

    tonic_prost_build::configure()
        .build_server(true)
        .build_client(false)
        .compile_protos(
            &[
                &format!("{api_base}/types/mount.proto"),
                &format!("{api_base}/types/platform.proto"),
                &format!("{api_base}/types/metrics.proto"),
                &format!("{api_base}/types/task/task.proto"),
                &format!("{api_base}/runtime/task/v3/shim.proto"),
                &format!("{api_base}/runtime/sandbox/v1/sandbox.proto"),
            ],
            &[&proto_root],
        )?;

    Ok(())
}
