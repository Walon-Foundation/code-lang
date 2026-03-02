package symbol

import (
	"fmt"

	"github.com/walonCode/code-lang/internal/ast"
)

type Builder struct {
	Global      *Scope
	Current     *Scope
	Errors      []string
	Resolutions map[ast.Node]int
}

func (b *Builder) error(line, col int, format string, args ...any) {
	b.Errors = append(b.Errors, fmt.Sprintf("[Line %d, Column %d] %s", line, col, fmt.Sprintf(format, args...)))
}

func NewBuilder() *Builder {
	global := NewScope("global", nil)
	return &Builder{
		Global:      global,
		Current:     global,
		Resolutions: make(map[ast.Node]int),
	}
}

func (b *Builder) Visit(node ast.Node) {
	if node == nil {
		return
	}
	switch n := node.(type) {
	case *ast.Program:
		for _, stmt := range n.Statements {
			b.Visit(stmt)
		}
	case ast.Statement:
		b.VisitStatement(n)
	case ast.Expression:
		b.VisitExpression(n)
	}
}

func (b *Builder) VisitStatement(stmt ast.Statement) {
	if stmt == nil {
		return
	}
	switch s := stmt.(type) {
	case *ast.LetStatement:
		if s == nil {
			return
		}
		if fn, ok := s.Value.(*ast.FunctionLiteral); ok {
			sym := b.Define(s.Name.Value, FUNCTION)
			b.EnterScope("fn")
			for _, param := range fn.Parameters {
				b.Define(param.Value, PARAMETER)
			}
			b.VisitStatement(&fn.Body)
			sym.NestedScope = b.Current
			b.ExitScope()
		} else {
			if existing := b.Current.Symbols[s.Name.Value]; existing != nil {
				if existing.Kind == CONSTANT {
					b.error(s.Name.Line(), s.Name.Column(), "cannot re-declare constant: %s", s.Name.Value)
				}
			}
			b.Define(s.Name.Value, VARIABLE)
			if s.Value != nil {
				b.VisitExpression(s.Value)
			}
		}
	case *ast.ConstStatement:
		if s == nil {
			return
		}
		if existing := b.Current.Symbols[s.Name.Value]; existing != nil {
			b.error(s.Name.Line(), s.Name.Column(), "identifier already defined: %s", s.Name.Value)
		}
		if fn, ok := s.Value.(*ast.FunctionLiteral); ok {
			sym := b.Define(s.Name.Value, CONSTANT)
			b.EnterScope("fn")
			for _, param := range fn.Parameters {
				b.Define(param.Value, PARAMETER)
			}
			b.VisitStatement(&fn.Body)
			sym.NestedScope = b.Current
			b.ExitScope()
		} else {
			b.Define(s.Name.Value, CONSTANT)
			if s.Value != nil {
				b.VisitExpression(s.Value)
			}
		}
	case *ast.ReturnStatement:
		if s == nil {
			return
		}
		if s.ReturnValue != nil {
			b.VisitExpression(s.ReturnValue)
		}
	case *ast.ExpressionStatement:
		if s == nil {
			return
		}
		if s.Expression != nil {
			b.VisitExpression(s.Expression)
		}
	case *ast.BlockStatement:
		if s == nil {
			return
		}
		b.EnterScope("block")
		for _, stmt := range s.Statements {
			b.VisitStatement(stmt)
		}
		b.ExitScope()
	case *ast.StructStatement:
		if s == nil {
			return
		}
		sym := b.Define(s.Name.Value, STRUCT)
		b.EnterScope(s.Name.Value)
		for name := range s.Fields {
			b.Define(name, STRUCT_FIELD)
		}
		sym.NestedScope = b.Current
		b.ExitScope()
	case *ast.ImportStatement:
		if s == nil {
			return
		}
		b.Define(s.Path, MODULE)
	case *ast.BreakStatement, *ast.ContinueStatement:
		// No symbols to define
	}
}

func (b *Builder) VisitExpression(expr ast.Expression) {
	if expr == nil {
		return
	}
	switch e := expr.(type) {
	case *ast.Identifier:
		if e == nil {
			return
		}
		if _, distance := b.Current.ResolveWithDistance(e.Value); distance == -1 {
			b.error(e.Line(), e.Column(), "undefined identifier: %s", e.Value)
		} else {
			b.Resolutions[e] = distance
		}
	case *ast.IntegerLiteral, *ast.Boolean, *ast.StringLiteral, *ast.FloatLiteral, *ast.CharLiteral:
		// No symbols to define
	case *ast.PrefixExpression:
		if e == nil {
			return
		}
		b.VisitExpression(e.Right)
	case *ast.UpdateExpression:
		if e == nil {
			return
		}
		if ident, ok := e.Target.(*ast.Identifier); ok {
			if sym := b.Resolve(ident.Value); sym != nil && sym.Kind == CONSTANT {
				b.error(ident.Line(), ident.Column(), "cannot reassign to const: %s", ident.Value)
			}
		}
		b.VisitExpression(e.Target)
	case *ast.InfixExpression:
		if e == nil {
			return
		}

		if isAssignmentOp(e.Operator) {
			if ident, ok := e.Left.(*ast.Identifier); ok {
				if sym := b.Resolve(ident.Value); sym != nil && sym.Kind == CONSTANT {
					b.error(ident.Line(), ident.Column(), "cannot reassign to const: %s", ident.Value)
				}
			}
		}

		b.VisitExpression(e.Left)
		b.VisitExpression(e.Right)
	case *ast.IfExpression:
		if e == nil {
			return
		}
		b.VisitExpression(e.Condition)
		b.VisitStatement(e.Consequence)
		for _, elif := range e.IfElse {
			b.VisitExpression(elif.Condition)
			b.VisitStatement(elif.Consequence)
		}
		if e.Alternative != nil {
			b.VisitStatement(e.Alternative)
		}
	case *ast.FunctionLiteral:
		if e == nil {
			return
		}
		b.EnterScope("fn")
		for _, param := range e.Parameters {
			b.Define(param.Value, PARAMETER)
		}
		b.VisitStatement(&e.Body)
		b.ExitScope()
	case *ast.CallExpression:
		if e == nil {
			return
		}
		b.VisitExpression(e.Function)
		for _, arg := range e.Arguments {
			b.VisitExpression(arg)
		}
	case *ast.MemberExpression:
		if e == nil {
			return
		}
		b.VisitExpression(e.Object)
	case *ast.ForExpression:
		if e == nil {
			return
		}
		b.EnterScope("for")
		if e.Init != nil {
			b.VisitStatement(e.Init)
		}
		if e.Condition != nil {
			b.VisitExpression(e.Condition)
		}
		if e.Post != nil {
			b.VisitStatement(e.Post)
		}
		b.VisitStatement(e.Body)
		b.ExitScope()
	case *ast.WhileExpression:
		if e == nil {
			return
		}
		b.EnterScope("while")
		b.VisitExpression(e.Condition)
		b.VisitStatement(e.Body)
		b.ExitScope()
	case *ast.ArrayLiteral:
		if e == nil {
			return
		}
		for _, el := range e.Elements {
			b.VisitExpression(el)
		}
	case *ast.IndexExpression:
		if e == nil {
			return
		}
		b.VisitExpression(e.Left)
		b.VisitExpression(e.Index)
	case *ast.HashLiteral:
		if e == nil {
			return
		}
		for k, v := range e.Pairs {
			b.VisitExpression(k)
			b.VisitExpression(v)
		}
	case *ast.StructLiteral:
		if e == nil {
			return
		}
		for _, v := range e.Fields {
			b.VisitExpression(v)
		}
	}
}

func (b *Builder) EnterScope(name string) {
	newScope := NewScope(name, b.Current)
	b.Current = newScope
}

func (b *Builder) ExitScope() {
	if b.Current.Parent != nil {
		b.Current = b.Current.Parent
	}
}

func (b *Builder) Define(name string, kind SymbolKind) *Symbol {
	sym := &Symbol{Name: name, Kind: kind}
	b.Current.Define(sym)
	return sym
}

func (b *Builder) Resolve(name string) *Symbol {
	return b.Current.Resolve(name)
}

func isAssignmentOp(op string) bool {
	switch op {
	case "=", "+=", "-=", "*=", "/=", "%=", "**=", "//=":
		return true
	default:
		return false
	}
}
