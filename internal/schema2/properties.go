/*
 * HCS API
 *
 * No description provided (generated by Swagger Codegen https://github.com/swagger-api/swagger-codegen)
 *
 * API version: 2.1
 * Generated by: Swagger Codegen (https://github.com/swagger-api/swagger-codegen.git)
 */

package hcsschema

type Properties struct {

	Id string `json:"Id,omitempty"`

	SystemType string `json:"SystemType,omitempty"`

	RuntimeOsType string `json:"RuntimeOsType,omitempty"`

	Name string `json:"Name,omitempty"`

	Owner string `json:"Owner,omitempty"`

	ObRoot string `json:"ObRoot,omitempty"`

	RuntimeId string `json:"RuntimeId,omitempty"`

	RuntimeTemplateId string `json:"RuntimeTemplateId,omitempty"`

	State string `json:"State,omitempty"`

	Stopped bool `json:"Stopped,omitempty"`

	ExitType string `json:"ExitType,omitempty"`

	Memory *MemoryInformationForVm `json:"Memory,omitempty"`

	ContainerReportedMemory *ContainerMemoryInformation `json:"ContainerReportedMemory,omitempty"`

	CpuGroupId string `json:"CpuGroupId,omitempty"`

	Statistics *Statistics `json:"Statistics,omitempty"`

	ProcessList []ProcessDetails `json:"ProcessList,omitempty"`

	TerminateOnLastHandleClosed bool `json:"TerminateOnLastHandleClosed,omitempty"`

	SystemGUID string `json:"SystemGUID,omitempty"`

	HostingSystemId string `json:"HostingSystemId,omitempty"`

	SharedMemoryRegionInfo []SharedMemoryRegionInfo `json:"SharedMemoryRegionInfo,omitempty"`

	GuestConnectionInfo *GuestConnectionInfo `json:"GuestConnectionInfo,omitempty"`

	Silo *SiloProperties `json:"Silo,omitempty"`

	CosIndex int32 `json:"CosIndex,omitempty"`

	Rmid int32 `json:"Rmid,omitempty"`

	CacheStats *CacheQueryStatsResponse `json:"CacheStats,omitempty"`
}