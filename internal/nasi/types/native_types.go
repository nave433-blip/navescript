package types

// This is the Native Navescript implementation of the type system,
// replacing reliance on external Go type reflection.

type TypeKind int

const (
	KindBasic TypeKind = iota
	KindUnion
	KindGeneric
	KindTuple
)

type NativeType struct {
	Kind TypeKind
	Name string
	// Internal representation for Navescript runtime
	Details map[string]interface{}
}

func NewNativeBasicType(name string) *NativeType {
	return &NativeType{Kind: KindBasic, Name: name}
}

func (n *NativeType) String() string {
	return n.Name
}
