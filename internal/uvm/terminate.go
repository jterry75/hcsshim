package uvm

import (
	"github.com/Microsoft/hcsshim/internal/logfields"
	"github.com/sirupsen/logrus"
)

// Terminate requests a utility VM terminate. If IsPending() on the error returned is true,
// it may not actually be shut down until Wait() succeeds.
func (uvm *UtilityVM) Terminate() error {
	fields := logrus.Fields{
		logfields.UVMID: uvm.id,
	}
	logrus.WithFields(fields).Debugf("uvm::Terminate - Begin Operation")
	defer func() {
		if err != nil {
			fields[logrus.ErrorKey] = err
			logrus.WithFields(fields).Error("uvm::Terminate - End Operation - Error")
		} else {
			logrus.WithFields(fields).Debug("uvm::Terminate - End Operation - Success")
		}
	}()

	return uvm.hcsSystem.Terminate()
}
