package persist

import (
	"encoding/json"
	"os"
)

type ExecutionSnapshot struct {
	IP        int
	Stack     []interface{}
	Variables map[string]interface{}
}

func SaveSnapshot(file string, snapshot ExecutionSnapshot) error {
	data, err := json.Marshal(snapshot)
	if err != nil {
		return err
	}
	return os.WriteFile(file, data, 0644)
}

func LoadSnapshot(file string) (*ExecutionSnapshot, error) {
	data, err := os.ReadFile(file)
	if err != nil {
		return nil, err
	}
	var snapshot ExecutionSnapshot
	err = json.Unmarshal(data, &snapshot)
	return &snapshot, err
}
