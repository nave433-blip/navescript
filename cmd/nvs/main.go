package main

import (
	"fmt"
	"os"
	"github.com/navescript/nvs/internal/parser"
	"github.com/navescript/nvs/internal/lexer"
)

func main() {
	if len(os.Args) < 2 {
		fmt.Println("Usage: nvs <file.ns>")
		return
	}

	code, err := os.ReadFile(os.Args[1])
	if err != nil {
		panic(err)
	}

	l := lexer.New(string(code))
	p := parser.New(l)
	ast := p.ParseProgram()

	fmt.Printf("Navescript Engine v0.1.0\nParsed AST nodes: %d\n", len(ast.Statements))
}
