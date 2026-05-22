package main

import (
	"os"
	"github.com/navescript/nvs/internal/lsp"
)

func main() {
	server := lsp.NewServer(os.Stdin, os.Stdout)
	server.Run()
}
