package ffi

// NativeBridge provides the Navescript runtime interface 
// to execute polyglot code without raw pointers.

type NativeBridge struct {
	EngineName string
	// Internal Navescript handle system for foreign objects
	ObjectMap map[uint64]interface{}
}

func NewNativeBridge(engine string) *NativeBridge {
	return &NativeBridge{
		EngineName: engine,
		ObjectMap:  make(map[uint64]interface{}),
	}
}

func (b *NativeBridge) InvokeForeign(name string, args []interface{}) (interface{}, error) {
	// Systematic implementation for calling out to engine
	return nil, nil
}
