package object

// Environment
type Environment struct {
	Store  map[string]Object
	Consts map[string]bool
	outer  *Environment
}

func NewEnclosedEnvironment(outer *Environment) *Environment {
	env := NewEnvironment()
	env.outer = outer

	return env
}

func NewEnvironment() *Environment {
	s := make(map[string]Object)
	c := make(map[string]bool)
	return &Environment{Store: s, Consts: c}
}

func (e *Environment) Get(name string) (Object, bool) {
	obj, ok := e.Store[name]
	if !ok && e.outer != nil {
		obj, ok = e.outer.Get(name)
	}
	return obj, ok
}

func (e *Environment) Set(name string, val Object) Object {
	e.Store[name] = val
	return val
}

func (e *Environment) SetConst(name string, val Object) Object {
	e.Store[name] = val
	e.Consts[name] = true
	return val
}

func (e *Environment) Update(name string, val Object) (Object, bool) {
	if isConst, ok := e.Consts[name]; ok && isConst {
		return nil, false // Cannot update a constant
	}

	_, ok := e.Store[name]
	if ok {
		e.Store[name] = val
		return val, true
	}
	if e.outer != nil {
		return e.outer.Update(name, val)
	}
	return nil, false
}

func (e *Environment) GetAt(distance int, name string) (Object, bool) {
	ancestor := e.ancestor(distance)
	if ancestor == nil {
		return nil, false
	}
	obj, ok := ancestor.Store[name]
	return obj, ok
}

func (e *Environment) UpdateAt(distance int, name string, val Object) bool {
	ancestor := e.ancestor(distance)
	if ancestor == nil {
		return false
	}
	if isConst, ok := ancestor.Consts[name]; ok && isConst {
		return false
	}
	ancestor.Store[name] = val
	return true
}

func (e *Environment) ancestor(distance int) *Environment {
	curr := e
	for i := 0; i < distance && curr != nil; i++ {
		curr = curr.outer
	}
	return curr
}
