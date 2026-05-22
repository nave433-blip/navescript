package engines

import (
    "bytes"
    "fmt"
    "os"
    "os/exec"
)

type PythonEngine struct{}

func (e *PythonEngine) Name() string { return "python" }

func (e *PythonEngine) Execute(code string, args []string) (string, error) {
    // Create temp file
    tmpFile, err := os.CreateTemp("", "nvs_python_*.py")
    if err != nil {
        return "", err
    }
    defer os.Remove(tmpFile.Name())
    
    // Write code
    if _, err := tmpFile.WriteString(code); err != nil {
        return "", err
    }
    tmpFile.Close()
    
    // Execute
    cmd := exec.Command("python3", append([]string{tmpFile.Name()}, args...)...)
    var stdout, stderr bytes.Buffer
    cmd.Stdout = &stdout
    cmd.Stderr = &stderr
    
    if err := cmd.Run(); err != nil {
        return "", fmt.Errorf("python error: %s\n%s", err, stderr.String())
    }
    
    return stdout.String(), nil
}
