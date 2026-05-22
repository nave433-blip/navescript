package debugger

type Breakpoint struct {
	ID   int
	File string
	Line int
}

type StackFrame struct {
	FunctionName string
	File         string
	Line         int
	Variables    map[string]interface{}
}

type Debugger struct {
	Breakpoints []Breakpoint
	Running     bool
	Paused      bool
}

func New() *Debugger {
	return &Debugger{
		Breakpoints: []Breakpoint{},
		Running:     false,
		Paused:      false,
	}
}

func (d *Debugger) SetBreakpoint(file string, line int) {
	d.Breakpoints = append(d.Breakpoints, Breakpoint{
		ID:   len(d.Breakpoints) + 1,
		File: file,
		Line: line,
	})
}
