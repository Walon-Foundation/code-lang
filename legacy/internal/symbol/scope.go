package symbol

type Scope struct {
	Name    string
	Parent  *Scope
	Symbols map[string]*Symbol
}

func NewScope(name string, parent *Scope) *Scope {
	return &Scope{
		Name:    name,
		Parent:  parent,
		Symbols: make(map[string]*Symbol),
	}
}

func (s *Scope) Define(sym *Symbol) {
	s.Symbols[sym.Name] = sym
}

func (s *Scope) Resolve(name string) *Symbol {
	if sym, ok := s.Symbols[name]; ok {
		return sym
	}
	if s.Parent != nil {
		return s.Parent.Resolve(name)
	}
	return nil
}

func (s *Scope) ResolveWithDistance(name string) (*Symbol, int) {
	distance := 0
	curr := s
	for curr != nil {
		if sym, ok := curr.Symbols[name]; ok {
			return sym, distance
		}
		curr = curr.Parent
		distance++
	}
	return nil, -1
}
