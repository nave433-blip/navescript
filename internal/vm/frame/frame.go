package frame

type Frame struct {
	IP             int
	Instructions   []byte
	BasePointer    int
	LocalVariables []interface{}
}

func New(instructions []byte, basePointer int) *Frame {
	return &Frame{
		IP:             -1,
		Instructions:   instructions,
		BasePointer:    basePointer,
		LocalVariables: make([]interface{}, 256),
	}
}
