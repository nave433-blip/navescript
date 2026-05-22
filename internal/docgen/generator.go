package docgen

import (
	"fmt"
	"os"
	"path/filepath"
)

type DocumentationGenerator struct {
	SourceDir string
}

func (dg *DocumentationGenerator) GenerateHTML() error {
	fmt.Printf("Generating documentation for %s...\n", dg.SourceDir)
	
	output := "<html><body><h1>Navescript API</h1><ul>"
	
	files, err := os.ReadDir(dg.SourceDir)
	if err != nil {
		return err
	}
	
	for _, file := range files {
		if filepath.Ext(file.Name()) == ".ns" {
			output += fmt.Sprintf("<li><a href='%s.html'>%s</a></li>", file.Name(), file.Name())
		}
	}
	output += "</ul></body></html>"
	
	return os.WriteFile("docs/index.html", []byte(output), 0644)
}
