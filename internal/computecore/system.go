package computecore

type ComputeSystem struct {
	sys hcsSystem
	id  string
}

func CreateComputeSystem(id, configuration string) (*ComputeSystem, error) {
	op := hcsCreateOperation(nil, callback)
	defer func() {
		hcsCloseOperation(op)
	}()

	var system hcsSystem
	err := hcsCreateComputeSystem(id, configuration, op, nil, &system)
	if err != nil {
		return nil, err
	}

}

func OpenComputeSystem(id string, requestedAccess uint32) (*ComputeSystem, error) {
	var system hcsSystem
	err := hcsOpenComputeSystem(id, requestedAccess, &system)
	if err != nil {
		return nil, err
	}
	return &ComputeSystem{
		sys: system,
		id:  id,
	}, nil
}

func (cs *ComputeSystem) Close() error {
	return hcsCloseComputeSystem(cs.sys)
}

func (cs *ComputeSystem) Start(options string) error {
	op := hcsCreateOperation(nil, callback)
	defer func() {
		hcsCloseOperation(op)
	}()

	err := hcsStartComputeSystem(cs.sys, op, options)
	if err != nil {
		return err
	}
	err = hcsWaitForOperationResult(op, 0, &resp)
	if err != nil {
		return err
	}

}
