package polyglot

import (
	"fmt"
	"os/exec"
)

type PolyglotBridge struct{}

func (b *PolyglotBridge) CallExternal(language string, code string) (string, error) {
	switch language {
	case "python":
		return b.runPython(code)
	case "js":
		return b.runJS(code)
	default:
		return "", fmt.Errorf("unsupported language: %s", language)
	}
}

func (b *PolyglotBridge) runPython(code string) (string, error) {
	cmd := exec.Command("python3", "-c", code)
	out, err := cmd.CombinedOutput()
	return string(out), err
}

func (b *PolyglotBridge) runJS(code string) (string, error) {
	cmd := exec.Command("node", "-e", code)
	out, err := cmd.CombinedOutput()
	return string(out), err
}
