package stdlib

import "encoding/json"

var JSONModule = map[string]interface{}{
	"marshal":   json.Marshal,
	"unmarshal": json.Unmarshal,
}
