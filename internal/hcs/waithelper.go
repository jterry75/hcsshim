//go:build windows

package hcs

import (
	"context"
	"time"

	"github.com/Microsoft/hcsshim/internal/computecore"
)

// withOperation creates an HCS operation, calls fn with it, waits for the result,
// and returns the result document. This keeps operation lifecycle as an internal detail.
func withOperation(ctx context.Context, fn func(op computecore.HcsOperation) error) (string, error) {
	op, err := computecore.HcsCreateOperation(ctx, 0, 0)
	if err != nil {
		return "", err
	}
	defer computecore.HcsCloseOperation(ctx, op)

	err = fn(op)
	if err != nil {
		// Any error, means the operation was not successfully even started. In
		// this case, the WaitForOperationResult will not be valid. Just close
		// the operation and return the call result.
		return "", err
	}

	return computecore.HcsWaitForOperationResult(ctx, op, 0xFFFFFFFF)
}

// withOperationTimeout is like withOperation but cancels the wait after the given
// timeout, returning ErrTimeout. A nil timeout means wait indefinitely.
func withOperationTimeout(ctx context.Context, t *time.Duration, fn func(op computecore.HcsOperation) error) (string, error) {
	op, err := computecore.HcsCreateOperation(ctx, 0, 0)
	if err != nil {
		return "", err
	}
	defer computecore.HcsCloseOperation(ctx, op)

	err = fn(op)
	if err != nil {
		// Any error, means the operation was not successfully even started. In
		// this case, the WaitForOperationResult will not be valid. Just close
		// the operation and return the call result.
		return "", err
	}

	type waitResult struct {
		doc string
		err error
	}
	done := make(chan waitResult, 1)
	go func() {
		doc, e := computecore.HcsWaitForOperationResult(ctx, op, 0xFFFFFFFF)
		done <- waitResult{doc, e}
	}()

	var timer <-chan time.Time
	if t != nil {
		tmr := time.NewTimer(*t)
		timer = tmr.C
		defer tmr.Stop()
	}

	select {
	case r := <-done:
		return r.doc, r.err
	case <-timer:
		_ = computecore.HcsCancelOperation(ctx, op)
		return "", ErrTimeout
	case <-ctx.Done():
		_ = computecore.HcsCancelOperation(ctx, op)
		return "", ctx.Err()
	}
}
