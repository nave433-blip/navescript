package stdlib

import (
    "io/ioutil"
    "os"
    "path/filepath"
)

var FSModule = map[string]interface{}{
    "read_file":  readFile,
    "write_file": writeFile,
    "list_dir":   listDirectory,
    "exists":     fileExists,
}

func readFile(path string) (string, error) {
    data, err := ioutil.ReadFile(path)
    return string(data), err
}

func writeFile(path, content string) error {
    return ioutil.WriteFile(path, []byte(content), 0644)
}

func listDirectory(path string) ([]string, error) {
    entries, err := ioutil.ReadDir(path)
    if err != nil {
        return nil, err
    }
    names := make([]string, len(entries))
    for i, e := range entries {
        names[i] = e.Name()
    }
    return names, nil
}

func fileExists(path string) bool {
    _, err := os.Stat(path)
    return !os.IsNotExist(err)
}
