package symbol

type SymbolKind int

const (
	VARIABLE SymbolKind = iota
	FUNCTION
	PARAMETER
	STRUCT_FIELD
	STRUCT
	CONSTANT
	MODULE
)

func (k SymbolKind) String() string {
	switch k {
	case VARIABLE:
		return "variable"
	case FUNCTION:
		return "function"
	case PARAMETER:
		return "parameter"
	case STRUCT_FIELD:
		return "struct_field"
	case STRUCT:
		return "struct"
	case CONSTANT:
		return "constant"
	case MODULE:
		return "module"
	default:
		return "unknown"
	}
}

type Symbol struct {
	Name        string
	Kind        SymbolKind
	NestedScope *Scope
}
