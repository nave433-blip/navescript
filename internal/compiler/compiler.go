package compiler

import (
	"fmt"
	"github.com/navescript/nvs/internal/ast"
	"github.com/navescript/nvs/internal/compiler/symbol"
)

type Opcode byte

const (
	OpConstant Opcode = iota
	OpAdd
	OpSub
	OpMul
	OpDiv
	OpPop
	OpTrue
	OpFalse
	OpJump
	OpJumpNotTruthy
	OpSetGlobal
	OpGetGlobal
	OpSetLocal
	OpGetLocal
	OpCall
	OpReturnValue
)

type Bytecode struct {
	Instructions []byte
	Constants    []interface{}
}

type Compiler struct {
	instructions []byte
	constants    []interface{}
	symbolTable  *symbol.SymbolTable
}

func New() *Compiler {
	return &Compiler{
		instructions: []byte{},
		constants:    []interface{}{},
		symbolTable:  symbol.NewSymbolTable(),
	}
}

func (c *Compiler) Compile(node ast.Node) error {
	switch n := node.(type) {
	case *ast.Program:
		for _, stmt := range n.Statements {
			if err := c.Compile(stmt); err != nil {
				return err
			}
		}
	case *ast.ExpressionStatement:
		if err := c.Compile(n.Expression); err != nil {
			return err
		}
		c.emit(OpPop)
	case *ast.IntegerLiteral:
		c.emit(OpConstant, c.addConstant(n.Value))
	case *ast.LetStatement:
		if err := c.Compile(n.Value); err != nil {
			return err
		}
		sym := c.symbolTable.Define(n.Name.Value)
		if sym.Scope == symbol.GlobalScope {
			c.emit(OpSetGlobal, sym.Index)
		} else {
			c.emit(OpSetLocal, sym.Index)
		}
	case *ast.Identifier:
		sym, ok := c.symbolTable.Resolve(n.Value)
		if !ok {
			return fmt.Errorf("undefined variable %s", n.Value)
		}
		if sym.Scope == symbol.GlobalScope {
			c.emit(OpGetGlobal, sym.Index)
		} else {
			c.emit(OpGetLocal, sym.Index)
		}
	}
	return nil
}

func (c *Compiler) emit(op Opcode, operands ...int) int {
	position := len(c.instructions)
	c.instructions = append(c.instructions, byte(op))
	for _, operand := range operands {
		c.instructions = append(c.instructions, byte(operand))
	}
	return position
}

func (c *Compiler) changeOperand(position int, operand int) {
	c.instructions[position+1] = byte(operand)
}

func (c *Compiler) addConstant(obj interface{}) int {
	c.constants = append(c.constants, obj)
	return len(c.constants) - 1
}

func (c *Compiler) Bytecode() *Bytecode {
	return &Bytecode{
		Instructions: c.instructions,
		Constants:    c.constants,
	}
}
