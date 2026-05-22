package ffi

import "fmt"

// ForeignFunction represents a binding to an external engine
type ForeignFunction struct {
	Language string
	Name     string
	Pointer  uintptr
}

// Registry manages external bindings
type Registry struct {
	Bindings map[string]*ForeignFunction
}

func NewRegistry() *Registry {
	return &Registry{Bindings: make(map[string]*ForeignFunction)}
}

func (r *Registry) Register(lang, name string, ptr uintptr) {
	r.Bindings[name] = &ForeignFunction{Language: lang, Name: name, Pointer: ptr}
	fmt.Printf("Registered %s binding for %s\n", lang, name)
}
