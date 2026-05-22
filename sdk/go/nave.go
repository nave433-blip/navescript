/*
    GoNvs: NaveScript-Go Bridge
*/
package gonvs

/*
#cgo LDFLAGS: -lnave
#include <nave_api.h>
*/
import "C"
import "unsafe"

type GoNvs struct {
	ctx unsafe.Pointer
}

func New() *GoNvs {
	return &GoNvs{ctx: C.nave_init()}
}

func (n *GoNvs) Run(path string) int {
	cPath := C.CString(path)
	defer C.free(unsafe.Pointer(cPath))
	return int(C.nave_run(n.ctx, cPath))
}
