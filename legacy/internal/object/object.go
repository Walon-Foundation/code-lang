package object

import (
	"bytes"
	"fmt"
	"hash/fnv"
	"strings"
	"time"

	"github.com/walonCode/code-lang/internal/ast"
)

type BuiltinFunction func(node *ast.CallExpression, args ...Object) Object
type ObjectType string

// constant for each object type
const (
	INTEGER_OBJ      = "INTEGER"
	BOOLEAN_OBJ      = "BOOLEAN"
	NULL_OBJ         = "NULL"
	RETURN_VALUE_OBJ = "RETURN_VALUE"
	ERROR_OBJ        = "ERROR"
	FUNCTION_OBJ     = "FUNCTION"
	STRING_OBJ       = "STRING"
	CHAR_OBJ         = "CHAR"
	FLOAT_OBJ        = "FLOAT"
	BUILTIN_OBJ      = "BUILTIN"
	ARRAY_OBJ        = "ARRAY"
	HASH_OBJ         = "HASH"
	MODULE_OBJ       = "MODULE"
	SERVER_OBJ       = "SERVER"
	TIME_OBJ         = "TIME"
	STRUCT_TYPE      = "STRUCT"
	STRUCT_INSTANCE  = "STRUCT"
	BREAK_OBJ        = "BREAK"
	CONTINUE_OBJ     = "CONTINUE"
)

// this allows us only to have on Bolean object and Null object
var (
	NULL  = &Null{}
	TRUE  = &Boolean{Value: true}
	FALSE = &Boolean{Value: false}
)

type Hashable interface {
	HashKey() HashKey
}

type Object interface {
	Type() ObjectType
	Inspect() string
}

// integer obj
type Integer struct {
	Value int64
}

func (i *Integer) Inspect() string  { return fmt.Sprintf("%d", i.Value) }
func (i *Integer) Type() ObjectType { return INTEGER_OBJ }

// bool obj
type Boolean struct {
	Value bool
}

func (i *Boolean) Inspect() string  { return fmt.Sprintf("%t", i.Value) }
func (i *Boolean) Type() ObjectType { return BOOLEAN_OBJ }

// null obj
type Null struct{}

func (i *Null) Inspect() string  { return "null" }
func (i *Null) Type() ObjectType { return NULL_OBJ }

// return value obj
type ReturnValue struct {
	Value Object
}

func (rv *ReturnValue) Type() ObjectType { return RETURN_VALUE_OBJ }
func (rv *ReturnValue) Inspect() string  { return rv.Value.Inspect() }

// error object
type Error struct {
	Message string
	Line    int
	Column  int
}

func (e *Error) Type() ObjectType { return ERROR_OBJ }
func (e *Error) Inspect() string {
	return fmt.Sprintf("[Line %d, Column %d] ERROR: %s", e.Line, e.Column, e.Message)
}

func NewError(line, col int, format string, a ...any) *Error {
	return &Error{Message: fmt.Sprintf(format, a...), Line: line, Column: col}
}

// function object
type Function struct {
	Parameters []*ast.Identifier
	Body       *ast.BlockStatement
	Env        *Environment
}

func (f *Function) Type() ObjectType { return FUNCTION_OBJ }
func (f *Function) Inspect() string {
	var out bytes.Buffer
	params := []string{}
	for _, p := range f.Parameters {
		params = append(params, p.String())
	}
	out.WriteString("fn")
	out.WriteString("(")
	out.WriteString(strings.Join(params, ", "))
	out.WriteString(") {\n")
	out.WriteString(f.Body.String())
	out.WriteString("\n}")
	return out.String()
}

// sttring obj
type String struct {
	Value string
}

func (s *String) Type() ObjectType { return STRING_OBJ }
func (s *String) Inspect() string  { return s.Value }

// char obj
type Char struct {
	Value rune
}

func (s *Char) Type() ObjectType { return CHAR_OBJ }
func (s *Char) Inspect() string  { return string(s.Value) }

// float obj
type Float struct {
	Value float64
}

func (s *Float) Type() ObjectType { return FLOAT_OBJ }
func (s *Float) Inspect() string  { return fmt.Sprintf("%f", s.Value) }

// builtin obj
type Builtin struct {
	Fn BuiltinFunction
}

func (b *Builtin) Type() ObjectType { return BUILTIN_OBJ }
func (b *Builtin) Inspect() string  { return "builtin function" }

// array obj
type Array struct {
	Elements []Object
}

func (ao *Array) Type() ObjectType { return ARRAY_OBJ }
func (ao *Array) Inspect() string {
	var out bytes.Buffer
	elements := []string{}
	for _, e := range ao.Elements {
		elements = append(elements, e.Inspect())
	}
	out.WriteString("[")
	out.WriteString(strings.Join(elements, ", "))
	out.WriteString("]")
	return out.String()
}

// hashkey object
type HashKey struct {
	Type  ObjectType
	Value uint64
}

// hash method for string int and bool
func (b *Boolean) HashKey() HashKey {
	var value uint64

	if b.Value {
		value = 1
	} else {
		value = 0
	}

	return HashKey{Type: b.Type(), Value: value}
}

func (i *Integer) HashKey() HashKey {
	return HashKey{Type: i.Type(), Value: uint64(i.Value)}
}

func (s *String) HashKey() HashKey {
	h := fnv.New64a()
	h.Write([]byte(s.Value))
	return HashKey{Type: s.Type(), Value: h.Sum64()}
}

// Hash obj
type HashPair struct {
	Key   Object
	Value Object
}

type Hash struct {
	Pairs map[HashKey]HashPair
}

func (h *Hash) Type() ObjectType { return HASH_OBJ }
func (h *Hash) Inspect() string {
	var out bytes.Buffer
	pairs := []string{}
	for _, pair := range h.Pairs {
		pairs = append(pairs, fmt.Sprintf("%s: %s",
			pair.Key.Inspect(), pair.Value.Inspect()))
	}
	out.WriteString("{")
	out.WriteString(strings.Join(pairs, ", "))
	out.WriteString("}")
	return out.String()
}

// server obj
type Server struct {
	Route     map[string]Object
	ApplyFunc func(fn Object, args []Object, node *ast.CallExpression) Object
	Members   map[string]Object
}

func (h *Server) Type() ObjectType { return SERVER_OBJ }
func (h *Server) Inspect() string {
	var out bytes.Buffer
	out.WriteString("server")
	return out.String()
}

// time object
type Time struct {
	Value time.Time
}

func (t *Time) Type() ObjectType { return TIME_OBJ }
func (t *Time) Inspect() string {
	return t.Value.Format("2006-01-02 15:04:05")
}

type StructType struct {
	Name     string
	Defaults map[string]Object
}

func (s *StructType) Type() ObjectType { return "STRUCT_TYPE" }
func (s *StructType) Inspect() string  { return "struct " + s.Name }

type StructInstance struct {
	TypeName string
	Fields   map[string]Object
}

func (s *StructInstance) Type() ObjectType { return "STRUCT_INSTANCE" }
func (s *StructInstance) Inspect() string  { return s.TypeName }

type Break struct{}

func (b *Break) Type() ObjectType { return BREAK_OBJ }
func (b *Break) Inspect() string  { return "break" }

type Continue struct{}

func (c *Continue) Type() ObjectType { return CONTINUE_OBJ }
func (c *Continue) Inspect() string  { return "continue" }
