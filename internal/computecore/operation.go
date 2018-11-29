package computecore

import (
	"reflect"
)

type Operation struct {
	op hcsOperation

	opctx     interface{}
	haveOpctx bool
}

func NewOperation(context interface{}) *Operation {
	var ctx uintptr
	if context != nil {
		ctx = reflect.ValueOf(context).UnsafeAddr()
	}
	op := hcsCreateOperation(ctx, callback)
	return &Operation{
		op: op,
	}
}

func (o *Operation) Close() {
	hcsCloseOperation(o.op)
}

func (o *Operation) Context() interface{} {
	if !o.haveOpctx {
		ctx := hcsGetOperationContext(o.op)
		o.opctx = ctx.(interface{})
		o.haveOpctx = true
	}
	return o.opctx
}

func (o *Operation) SetContext(ctx interface{}) error {
	err := hcsSetOperationContext(o.op, ctx)
	if err != nil {
		return err
	}
	o.opctx = ctx
	o.haveOpctx = true
}

func (o *Operation) ComputeSystem() *ComputeSystem {

}

func (o *Operation) Process() *Process {

}

func (o *Operation) Type() OperationType {

}

func (o *Operation) ID() uint64 {

}

func (o *Operation) Result() (string, error) {

}

func (o *Operation) ResultAndProcessInfo() (*Process, string, error) {

}

func (o *Operation Wait(timeout time.Duration) (string, error) {
}

func (o *Operation) WaitForResultAndProcessInfo(timeout time.Duration) {

}

func (o *Operation) SetCallback() {

}

func (o *Operation) Cancel() {
	
}