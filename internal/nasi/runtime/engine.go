package runtime

import (
	"fmt"
	"github.com/navescript/nvs/internal/nasi/ffi"
	"github.com/navescript/nvs/internal/nasi/oop"
	"github.com/navescript/nvs/internal/nasi/types"
)

// Runtime orchestrates all native systems
type Runtime struct {
	TypeSystem *types.NativeType
	ClassSystem map[string]*oop.NativeClass
	Bridge      *ffi.NativeBridge
}

func NewRuntime() *Runtime {
	return &Runtime{
		ClassSystem: make(map[string]*oop.NativeClass),
		Bridge:      ffi.NewNativeBridge("core"),
	}
}

func (r *Runtime) DefineClass(cls *oop.NativeClass) {
	r.ClassSystem[cls.ClassName] = cls
	fmt.Printf("Defined native class: %s\n", cls.ClassName)
}
