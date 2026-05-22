package go

// NaveScript Go Binding
import (
    "C"
)

type Runtime struct {
    handle unsafe.Pointer
}

func NewRuntime() *Runtime {
    return &Runtime{handle: C.nvs_create_runtime()}
}

func (r *Runtime) Run(code string) {
    cCode := C.CString(code)
    C.nvs_run(r.handle, cCode)
    C.free(unsafe.Pointer(cCode))
}
