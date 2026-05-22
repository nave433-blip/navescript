package test

import (
	"testing"
	"github.com/navescript/nvs/internal/compiler"
	"github.com/navescript/nvs/internal/lexer"
	"github.com/navescript/nvs/internal/parser"
	"github.com/navescript/nvs/internal/vm"
)

func TestVariables(t *testing.T) {
	input := "let a = 1; let b = a; b;"
	
	l := lexer.New(input)
	p := parser.New(l)
	program := p.ParseProgram()
	
	comp := compiler.New()
	comp.Compile(program)
	
	machine := vm.New(comp.Bytecode())
	machine.Run()
	
	result := machine.LastPoppedStackElem()
	if result != 1 {
		t.Errorf("result is not 1, got=%d", result)
	}
}
