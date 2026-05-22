package stdlib

import (
    "os"
    "net/http"
    "encoding/json"
    "crypto/sha256"
    "encoding/hex"
    "io"
)

// FS Module expanded
func ReadFile(path string) (string, error) {
    data, err := os.ReadFile(path)
    return string(data), err
}

func WriteFile(path string, content string) error {
    return os.WriteFile(path, []byte(content), 0644)
}

// Net Module expanded
func HttpGet(url string) (string, error) {
    resp, err := http.Get(url)
    if err != nil { return "", err }
    defer resp.Body.Close()
    body, err := io.ReadAll(resp.Body)
    return string(body), err
}

// JSON Module expanded
func ToJson(v interface{}) (string, error) {
    b, err := json.Marshal(v)
    return string(b), err
}

func FromJson(data string, v interface{}) error {
    return json.Unmarshal([]byte(data), v)
}

// Crypto Module expanded
func Sha256(data string) string {
    hash := sha256.Sum256([]byte(data))
    return hex.EncodeToString(hash[:])
}
