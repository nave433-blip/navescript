package stdlib

import (
	"crypto/sha256"
	"encoding/hex"
	"encoding/json"
)

var CryptoModule = map[string]interface{}{
	"sha256": sha256Hash,
}

var JSONModule = map[string]interface{}{
	"stringify": jsonStringify,
	"parse":     jsonParse,
}

func sha256Hash(data string) string {
	h := sha256.New()
	h.Write([]byte(data))
	return hex.EncodeToString(h.Sum(nil))
}

func jsonStringify(data interface{}) (string, error) {
	b, err := json.Marshal(data)
	return string(b), err
}

func jsonParse(data string) (interface{}, error) {
	var v interface{}
	err := json.Unmarshal([]byte(data), &v)
	return v, err
}
