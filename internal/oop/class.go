package oop

// Class represents a Navescript class definition
type Class struct {
	Name       string
	Methods    map[string]*Method
	Fields     map[string]*Field
	SuperClass *Class
}

type Method struct {
	Name       string
	Parameters []string
	Body       interface{} // Simplified
}

type Field struct {
	Name       string
	Visibility string // public, private, protected
}

func NewClass(name string) *Class {
	return &Class{
		Name:    name,
		Methods: make(map[string]*Method),
		Fields:  make(map[string]*Field),
	}
}
