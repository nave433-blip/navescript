// navescript/internal/vm/vm.go (Updated - Input Access)
package vm

import (
	"fmt"
	"github.com/navescript/nvs/internal/compiler"
	"github.com/navescript/nvs/internal/vm/frame"
)

const StackSize = 2048

type VM struct {
	constants    []interface{}
	instructions []byte
	stack        []interface{}
	sp           int
	globals      []interface{}
	frames       []*frame.Frame
	frameIndex   int
	returnValue  interface{}
	inputData    interface{} // New field for input from host (e.g., AST from Go)
}

func New(bytecode *compiler.Bytecode) *VM {
	return &VM{
		instructions: bytecode.Instructions,
		constants:    bytecode.Constants,
		stack:        make([]interface{}, StackSize),
		sp:           0,
		globals:      make([]interface{}, 1024),
		frames:       []*frame.Frame{frame.New(bytecode.Instructions, 0)},
		frameIndex:   1,
	}
}

func (vm *VM) currentFrame() *frame.Frame {
	return vm.frames[vm.frameIndex-1]
}

func (vm *VM) SetInput(data interface{}) {
	vm.inputData = data
}

// GetInput allows Navescript code running in the VM to retrieve the host-provided input.
func (vm *VM) GetInput() interface{} {
	return vm.inputData
}

func (vm *VM) Run() error {
	for ip := 0; ip < len(vm.currentFrame().Instructions); ip++ {
		op := compiler.Opcode(vm.currentFrame().Instructions[ip])
		switch op {
		case compiler.OpConstant:
			ip++
			idx := int(vm.currentFrame().Instructions[ip])
			vm.push(vm.constants[idx])
		case compiler.OpPop:
			vm.pop()
		case compiler.OpAdd:
			right := vm.pop().(int)
			left := vm.pop().(int)
			vm.push(left + right)
		case compiler.OpJump:
			ip++
			pos := int(vm.currentFrame().Instructions[ip])
			ip = pos - 1
		case compiler.OpJumpNotTruthy:
			ip++
			pos := int(vm.currentFrame().Instructions[ip])
			condition := vm.pop().(bool) // Assuming boolean
			if !condition {
				ip = pos - 1
			}
		case compiler.OpSetGlobal:
			ip++
			idx := int(vm.currentFrame().Instructions[ip])
			vm.globals[idx] = vm.pop()
		case compiler.OpGetGlobal:
			ip++
			idx := int(vm.currentFrame().Instructions[ip])
			vm.push(vm.globals[idx])
		case compiler.OpSetLocal:
			ip++
			idx := int(vm.currentFrame().Instructions[ip])
			vm.currentFrame().LocalVariables[idx] = vm.pop()
		case compiler.OpGetLocal:
			ip++
			idx := int(vm.currentFrame().Instructions[ip])
			vm.push(vm.currentFrame().LocalVariables[idx])
		case compiler.OpCall:
			ip++
			numArgs := int(vm.currentFrame().Instructions[ip])
			fmt.Printf("VM: Conceptual call with %d args
", numArgs)
		case compiler.OpReturnValue:
			vm.returnValue = vm.pop() // Capture return value
			return nil // Exit VM or return to previous frame
		}
	}
	return nil
}

func (vm *VM) push(obj interface{}) {
	if vm.sp >= StackSize {
		panic("stack overflow")
	}
	vm.stack[vm.sp] = obj
	vm.sp++
}

func (vm *VM) pop() interface{} {
	if vm.sp <= 0 {
		panic("stack underflow")
	}
	vm.sp--
	obj := vm.stack[vm.sp]
	return obj
}

func (vm *VM) GetReturnValue() interface{} {
	return vm.returnValue
}
