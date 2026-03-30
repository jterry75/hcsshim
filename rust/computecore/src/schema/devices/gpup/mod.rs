// Config.Devices.Gpup

use serde::{Deserialize, Serialize};

use super::Device;

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GpuPartition {
    #[serde(flatten)]
    pub base: Device,

    #[serde(rename = "InstanceGuid")]
    pub channel_instance_guid: String,

    #[serde(default, rename = "PoolId", skip_serializing_if = "Option::is_none")]
    pub pool_id: Option<String>,

    #[serde(
        default,
        rename = "MinPartitionVRAM",
        skip_serializing_if = "Option::is_none"
    )]
    pub min_partition_vram: Option<u64>,

    #[serde(
        default,
        rename = "MaxPartitionVRAM",
        skip_serializing_if = "Option::is_none"
    )]
    pub max_partition_vram: Option<u64>,

    #[serde(
        default,
        rename = "OptimalPartitionVRAM",
        skip_serializing_if = "Option::is_none"
    )]
    pub optimal_partition_vram: Option<u64>,

    #[serde(
        default,
        rename = "MinPartitionEncode",
        skip_serializing_if = "Option::is_none"
    )]
    pub min_partition_encode: Option<u64>,

    #[serde(
        default,
        rename = "MaxPartitionEncode",
        skip_serializing_if = "Option::is_none"
    )]
    pub max_partition_encode: Option<u64>,

    #[serde(
        default,
        rename = "OptimalPartitionEncode",
        skip_serializing_if = "Option::is_none"
    )]
    pub optimal_partition_encode: Option<u64>,

    #[serde(
        default,
        rename = "MinPartitionDecode",
        skip_serializing_if = "Option::is_none"
    )]
    pub min_partition_decode: Option<u64>,

    #[serde(
        default,
        rename = "MaxPartitionDecode",
        skip_serializing_if = "Option::is_none"
    )]
    pub max_partition_decode: Option<u64>,

    #[serde(
        default,
        rename = "OptimalPartitionDecode",
        skip_serializing_if = "Option::is_none"
    )]
    pub optimal_partition_decode: Option<u64>,

    #[serde(
        default,
        rename = "MinPartitionCompute",
        skip_serializing_if = "Option::is_none"
    )]
    pub min_partition_compute: Option<u64>,

    #[serde(
        default,
        rename = "MaxPartitionCompute",
        skip_serializing_if = "Option::is_none"
    )]
    pub max_partition_compute: Option<u64>,

    #[serde(
        default,
        rename = "OptimalPartitionCompute",
        skip_serializing_if = "Option::is_none"
    )]
    pub optimal_partition_compute: Option<u64>,

    #[serde(
        default,
        rename = "HostResource",
        skip_serializing_if = "Option::is_none"
    )]
    pub host_resource: Option<String>,

    #[serde(
        default,
        rename = "NumaAwarePlacement",
        skip_serializing_if = "Option::is_none"
    )]
    pub numa_aware_placement: Option<bool>,

    #[serde(
        default,
        rename = "Preallocation",
        skip_serializing_if = "Option::is_none"
    )]
    pub preallocation: Option<serde_json::Value>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GpuPartitionTelemetry {
    #[serde(
        default,
        rename = "ChannelInstanceGuid",
        skip_serializing_if = "Option::is_none"
    )]
    pub channel_instance_guid: Option<String>,

    #[serde(default, rename = "PoolId", skip_serializing_if = "Option::is_none")]
    pub pool_id: Option<String>,

    #[serde(
        default,
        rename = "MinPartitionVRAM",
        skip_serializing_if = "Option::is_none"
    )]
    pub min_partition_vram: Option<u64>,

    #[serde(
        default,
        rename = "MaxPartitionVRAM",
        skip_serializing_if = "Option::is_none"
    )]
    pub max_partition_vram: Option<u64>,

    #[serde(
        default,
        rename = "OptimalPartitionVRAM",
        skip_serializing_if = "Option::is_none"
    )]
    pub optimal_partition_vram: Option<u64>,

    #[serde(
        default,
        rename = "MinPartitionEncode",
        skip_serializing_if = "Option::is_none"
    )]
    pub min_partition_encode: Option<u64>,

    #[serde(
        default,
        rename = "MaxPartitionEncode",
        skip_serializing_if = "Option::is_none"
    )]
    pub max_partition_encode: Option<u64>,

    #[serde(
        default,
        rename = "OptimalPartitionEncode",
        skip_serializing_if = "Option::is_none"
    )]
    pub optimal_partition_encode: Option<u64>,

    #[serde(
        default,
        rename = "MinPartitionDecode",
        skip_serializing_if = "Option::is_none"
    )]
    pub min_partition_decode: Option<u64>,

    #[serde(
        default,
        rename = "MaxPartitionDecode",
        skip_serializing_if = "Option::is_none"
    )]
    pub max_partition_decode: Option<u64>,

    #[serde(
        default,
        rename = "OptimalPartitionDecode",
        skip_serializing_if = "Option::is_none"
    )]
    pub optimal_partition_decode: Option<u64>,

    #[serde(
        default,
        rename = "MinPartitionCompute",
        skip_serializing_if = "Option::is_none"
    )]
    pub min_partition_compute: Option<u64>,

    #[serde(
        default,
        rename = "MaxPartitionCompute",
        skip_serializing_if = "Option::is_none"
    )]
    pub max_partition_compute: Option<u64>,

    #[serde(
        default,
        rename = "OptimalPartitionCompute",
        skip_serializing_if = "Option::is_none"
    )]
    pub optimal_partition_compute: Option<u64>,

    #[serde(
        default,
        rename = "HostResource",
        skip_serializing_if = "Option::is_none"
    )]
    pub host_resource: Option<String>,

    #[serde(
        default,
        rename = "NumaAwarePlacement",
        skip_serializing_if = "Option::is_none"
    )]
    pub numa_aware_placement: Option<bool>,
}
