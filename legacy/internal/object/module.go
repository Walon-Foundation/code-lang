package object

import "strings"

import "fmt"

type Module struct {
	Members map[string]Object
}

func (m *Module) Type() ObjectType { return MODULE_OBJ }
func (m *Module) Inspect() string {
	var s strings.Builder
	s.WriteString("Module{")
	for k, v := range m.Members {
		fmt.Fprintf(&s, "%s: %s, ", k, v.Inspect())
	}
	s.WriteString("}")
	return s.String()
}
