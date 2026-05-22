package types

// Type represents the base type system interface
type Type interface {
	Name() string
	Equals(other Type) bool
}

// BasicType implements Type for simple types
type BasicType struct {
	TypeName string
}

func (b *BasicType) Name() string     { return b.TypeName }
func (b *BasicType) Equals(other Type) bool {
	return b.Name() == other.Name()
}

// UnionType handles T | U
type UnionType struct {
	Types []Type
}

func (u *UnionType) Name() string { return "Union" }
func (u *UnionType) Equals(other Type) bool { return false } // Simplified for now

// GenericType handles T<U>
type GenericType struct {
	Base     Type
	TypeVars []Type
}

func (g *GenericType) Name() string { return g.Base.Name() + "<...>" }
func (g *GenericType) Equals(other Type) bool { return false }
