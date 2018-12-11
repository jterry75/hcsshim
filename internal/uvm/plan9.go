package uvm

import (
	"fmt"

	"github.com/Microsoft/hcsshim/internal/guestrequest"
	"github.com/Microsoft/hcsshim/internal/logfields"
	"github.com/Microsoft/hcsshim/internal/requesttype"
	"github.com/Microsoft/hcsshim/internal/schema2"
	"github.com/sirupsen/logrus"
)

// AddPlan9 adds a Plan9 share to a utility VM. Each Plan9 share is ref-counted and
// only added if it isn't already.
func (uvm *UtilityVM) AddPlan9(hostPath string, uvmPath string, readOnly bool) (err error) {
	fields := logrus.Fields{
		logfields.UVMID: uvm.id,
		"host-path":     hostPath,
		"uvm-path":      uvmPath,
		"readOnly":      readOnly,
	}
	logrus.WithFields(fields).Debug("uvm::AddPlan9 - Begin Operation")
	defer func() {
		if err != nil {
			fields[logrus.ErrorKey] = err
			logrus.WithFields(fields).Error("uvm::AddPlan9 - End Operation - Error")
		} else {
			logrus.WithFields(fields).Debug("uvm::AddPlan9 - End Operation - Success")
		}
	}()

	if uvm.operatingSystem != "linux" {
		return errNotSupported
	}
	if uvmPath == "" {
		return fmt.Errorf("uvmPath must be passed to AddPlan9")
	}

	uvm.m.Lock()
	defer uvm.m.Unlock()
	if uvm.plan9Shares == nil {
		uvm.plan9Shares = make(map[string]*plan9Info)
	}
	if _, ok := uvm.plan9Shares[hostPath]; !ok {
		uvm.plan9Counter++

		modification := &hcsschema.ModifySettingRequest{
			RequestType: requesttype.Add,
			Settings: hcsschema.Plan9Share{
				Name: fmt.Sprintf("%d", uvm.plan9Counter),
				Path: hostPath,
				Port: int32(uvm.plan9Counter), // TODO: Temporary. Will all use a single port (9999)
			},
			ResourcePath: fmt.Sprintf("VirtualMachine/Devices/Plan9/Shares"),
			GuestRequest: guestrequest.GuestRequest{
				ResourceType: guestrequest.ResourceTypeMappedDirectory,
				RequestType:  requesttype.Add,
				Settings: guestrequest.LCOWMappedDirectory{
					MountPath: uvmPath,
					Port:      int32(uvm.plan9Counter), // TODO: Temporary. Will all use a single port (9999)
					ReadOnly:  readOnly,
				},
			},
		}

		if err := uvm.Modify(modification); err != nil {
			return err
		}
		uvm.plan9Shares[hostPath] = &plan9Info{
			refCount:  1,
			uvmPath:   uvmPath,
			idCounter: uvm.plan9Counter,
			port:      int32(uvm.plan9Counter), // TODO: Temporary. Will all use a single port (9999)
		}
	} else {
		uvm.plan9Shares[hostPath].refCount++
	}
	fields["refCount"] = uvm.plan9Shares[hostPath].refCount
	return nil
}

// RemovePlan9 removes a Plan9 share from a utility VM. Each Plan9 share is ref-counted
// and only actually removed when the ref-count drops to zero.
func (uvm *UtilityVM) RemovePlan9(hostPath string) (err error) {
	fields := logrus.Fields{
		logfields.UVMID: uvm.id,
		"host-path":     hostPath,
	}
	logrus.WithFields(fields).Debug("uvm::RemovePlan9 - Begin Operation")
	defer func() {
		if err != nil {
			fields[logrus.ErrorKey] = err
			logrus.WithFields(fields).Error("uvm::RemovePlan9 - End Operation - Error")
		} else {
			logrus.WithFields(fields).Debug("uvm::RemovePlan9 - End Operation - Success")
		}
	}()

	if uvm.operatingSystem != "linux" {
		return errNotSupported
	}

	uvm.m.Lock()
	defer uvm.m.Unlock()
	share, ok := uvm.plan9Shares[hostPath]
	if !ok {
		return ErrNotAttached
	}

	share.refCount--
	fields["refCount"] = share.refCount
	if share.refCount > 0 {
		return nil
	}
	modification := &hcsschema.ModifySettingRequest{
		RequestType: requesttype.Remove,
		Settings: hcsschema.Plan9Share{
			Name: fmt.Sprintf("%d", share.idCounter),
			Port: share.port,
		},
		ResourcePath: fmt.Sprintf("VirtualMachine/Devices/Plan9/Shares"),
		GuestRequest: guestrequest.GuestRequest{
			ResourceType: guestrequest.ResourceTypeMappedDirectory,
			RequestType:  requesttype.Remove,
			Settings: guestrequest.LCOWMappedDirectory{
				MountPath: share.uvmPath,
				Port:      share.port,
			},
		},
	}
	if err := uvm.Modify(modification); err != nil {
		return fmt.Errorf("failed to remove plan9 share %s from %s: %+v: %s", hostPath, uvm.id, modification, err)
	}
	delete(uvm.plan9Shares, hostPath)
	return nil
}
