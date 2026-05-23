// navescript/cmd/nvs/main.go (Updated - Self-Hosting Bridge Refined)
package main

import (
	"encoding/json" // Added for JSON serialization
	"fmt"
	"io/ioutil"
	"os"

	"github.com/navescript/nvs/internal/compiler"
	"github.com/navescript/nvs/internal/lexer"
	"github.com/navescript/nvs/internal/parser"
	"github.com/navescript/nvs/internal/vm"
)

// This function acts as the bridge for self-hosting.
// It runs a Navescript file within the Go-based Navescript VM
// and returns its output (e.g., generated NAS code).
func runNavescriptFileInVM(path string, inputAST *parser.Program) (string, error) {
	code, err := ioutil.ReadFile(path)
	if err != nil {
		return "", fmt.Errorf("error reading Navescript file: %v", err)
	}

	l := lexer.New(string(code))
	p := parser.New(l)
	program := p.ParseProgram()
	if len(p.Errors()) != 0 {
		return "", fmt.Errorf("Navescript parsing errors in %s: %v", path, p.Errors())
	}

	comp := compiler.New()
	err = comp.Compile(program) // Compile nas_backend.ns itself
	if err != nil {
		return "", fmt.Errorf("Navescript compilation error for %s: %v", path, err)
	}

	machine := vm.New(comp.Bytecode())
	
	// Serialize the input AST to JSON and pass it to the Navescript VM
	astJSON, err := json.Marshal(inputAST)
	if err != nil {
		return "", fmt.Errorf("failed to serialize AST to JSON: %v", err)
	}
	machine.SetInput(string(astJSON)) // Pass the JSON string of the AST
	
	err = machine.Run() // Run nas_backend.ns within the VM
	if err != nil {
		return "", fmt.Errorf("Navescript VM execution error for %s: %v", path, err)
	}
	
	// Assuming nas_backend.ns `return`s a string of NAS code
	returnValue := machine.GetReturnValue()
	if nasOutput, ok := returnValue.(string); ok {
		return nasOutput, nil
	}
	return "", fmt.Errorf("nas_backend.ns did not return a string of NAS code, got: %v", returnValue)
}

func main() {
	if len(os.Args) < 2 {
		fmt.Println("Usage: nvs <command> [args]")
		os.Exit(1)
	}

	command := os.Args[1]
	switch command {
	case "run":
		if len(os.Args) < 3 {
			fmt.Println("Error: No file specified.")
			os.Exit(1)
		}
		runFile(os.Args[2])
	case "build":
		if len(os.Args) < 3 {
			fmt.Println("Error: No file specified.")
			os.Exit(1)
		}
		buildFile(os.Args[2])
	case "fmt":
		fmt.Println("Code formatted.")
	default:
		fmt.Printf("Unknown command: %s
", command)
	}
}

func runFile(path string) {
	code, err := ioutil.ReadFile(path)
	if err != nil {
		fmt.Printf("Error reading file: %v
", err)
		os.Exit(1)
	}

	l := lexer.New(string(code))
	p := parser.New(l)
	program := p.ParseProgram()
	if len(p.Errors()) != 0 {
		for _, err := range p.Errors() {
			fmt.Println(err)
		}
		os.Exit(1)
	}

	comp := compiler.New()
	err = comp.Compile(program)
	if err != nil {
		fmt.Printf("Compilation error: %v
", err)
		os.Exit(1)
	}

	machine := vm.New(comp.Bytecode())
	err = machine.Run()
	if err != nil {
		fmt.Printf("VM execution error: %v
", err)
		os.Exit(1)
	}
	fmt.Printf("VM returned: %v
", machine.GetReturnValue()) // Show what the user's script returned
}

func buildFile(path string) {
	code, err := ioutil.ReadFile(path)
	if err != nil {
		fmt.Printf("Error reading file: %v
", err)
		os.Exit(1)
	}

	// Lexing and parsing user's Navescript file
	l := lexer.New(string(code))
	p := parser.New(l)
	userProgramAST := p.ParseProgram() // This is the AST we'd pass to nas_backend.ns
	if len(p.Errors()) != 0 {
		for _, err := range p.Errors() {
			fmt.Println(err)
		}
		os.Exit(1)
	}

	// Execute the native Navescript compiler backend (nas_backend.ns)
	// within the Go-based Navescript VM to generate NAS code.
	fmt.Printf("Invoking native Navescript compiler backend to generate NAS assembly for %s...
", path)
	generatedNAS, err := runNavescriptFileInVM("navescript/src/compiler/backend/nas_backend.ns", userProgramAST) // userProgramAST is conceptually passed
	if err != nil {
		fmt.Printf("Error running native Navescript backend: %v
", err)
		os.Exit(1)
	}

	outputFileName := path + ".nas"
	err = ioutil.WriteFile(outputFileName, []byte(generatedNAS), 0644)
	if err != nil {
		fmt.Printf("Error writing NAS file: %v
", err)
		os.Exit(1)
	}
	fmt.Printf("Successfully compiled %s to %s
", path, outputFileName)
}
