package stdlib

import "crypto/sha256"
import "encoding/hex"

var CryptoModule = map[string]interface{}{
	"sha256": func(data string) string {
		hash := sha256.Sum256([]byte(data))
		return hex.EncodeToString(hash[:])
	},
}
