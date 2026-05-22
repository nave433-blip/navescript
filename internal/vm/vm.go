package vm

import (
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
			condition := vm.pop().(bool)
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
			frame := frame.New(vm.currentFrame().Instructions, vm.sp-numArgs)
			vm.frames = append(vm.frames, frame)
			vm.frameIndex++
		case compiler.OpReturnValue:
			returnValue := vm.pop()
			vm.frameIndex--
			vm.frames = vm.frames[:vm.frameIndex]
			vm.push(returnValue)
		}
	}
	return nil
}

func (vm *VM) push(obj interface{}) {
	vm.stack[vm.sp] = obj
	vm.sp++
}

func (vm *VM) pop() interface{} {
	obj := vm.stack[vm.sp-1]
	vm.sp--
	return obj
}
