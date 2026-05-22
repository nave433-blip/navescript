package oop

// NativeClass defines the Navescript class model
// independent of Go's struct system.

type NativeClass struct {
	ClassName   string
	BaseClass   *NativeClass
	NativeMethods map[string]*NativeMethod
	NativeFields  map[string]*NativeField
}

type NativeMethod struct {
	Name string
	Args []string
	Body []byte // Navescript bytecode
}

type NativeField struct {
	Name       string
	Visibility int // 0: public, 1: private
}

func CreateNativeClass(name string) *NativeClass {
	return &NativeClass{
		ClassName:     name,
		NativeMethods: make(map[string]*NativeMethod),
		NativeFields:  make(map[string]*NativeField),
	}
}
