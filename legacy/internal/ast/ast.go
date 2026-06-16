package ast

import (
	"bytes"
	"strings"

	"github.com/walonCode/code-lang/internal/token"
)

type Node interface {
	TokenLiteral() string
	String() string
	Line() int
	Column() int
}

type Statement interface {
	Node
	statementNode()
}

type Expression interface {
	Node // TokenLiteral and String()
	expressionNode()
}

type Program struct {
	Statements []Statement
}

type LetStatement struct {
	Token token.Token
	Name  *Identifier
	Value Expression
}

// method on the let statement
func (ls *LetStatement) statementNode()       {}
func (ls *LetStatement) TokenLiteral() string { return ls.Token.Literal }
func (ls *LetStatement) String() string {
	var out bytes.Buffer

	out.WriteString(ls.TokenLiteral() + " ")
	out.WriteString(ls.Name.String())
	out.WriteString(" = ")

	if ls.Value != nil {
		out.WriteString(ls.Value.String())
	}

	out.WriteString(";")

	return out.String()
}
func (ls *LetStatement) Line() int   { return ls.Token.Line }
func (ls *LetStatement) Column() int { return ls.Token.Column }

type ConstStatement struct {
	Token token.Token
	Name  *Identifier
	Value Expression
}

func (cs *ConstStatement) statementNode()       {}
func (cs *ConstStatement) TokenLiteral() string { return cs.Token.Literal }
func (cs *ConstStatement) String() string {
	var out bytes.Buffer

	out.WriteString(cs.TokenLiteral() + " ")
	out.WriteString(cs.Name.String())
	out.WriteString(" = ")

	if cs.Value != nil {
		out.WriteString(cs.Value.String())
	}

	out.WriteString(";")

	return out.String()
}
func (cs *ConstStatement) Line() int   { return cs.Token.Line }
func (cs *ConstStatement) Column() int { return cs.Token.Column }

type ReturnStatement struct {
	Token       token.Token
	ReturnValue Expression
}

// methods on the return statement
func (i *ReturnStatement) statementNode()       {}
func (i *ReturnStatement) TokenLiteral() string { return i.Token.Literal }
func (i *ReturnStatement) String() string {
	var out bytes.Buffer
	out.WriteString(i.TokenLiteral() + " ")
	if i.ReturnValue != nil {
		out.WriteString(i.ReturnValue.String())
	}
	out.WriteString(";")
	return out.String()
}
func (i *ReturnStatement) Line() int   { return i.Token.Line }
func (i *ReturnStatement) Column() int { return i.Token.Column }

type PrefixExpression struct {
	Token    token.Token
	Operator string
	Right    Expression
}

// methods on prefix expression
func (i *PrefixExpression) expressionNode()      {}
func (i *PrefixExpression) TokenLiteral() string { return i.Token.Literal }
func (i *PrefixExpression) String() string {
	var out bytes.Buffer

	out.WriteString("(")
	out.WriteString(i.Operator)
	out.WriteString(i.Right.String())
	out.WriteString(")")

	return out.String()
}
func (i *PrefixExpression) Line() int   { return i.Token.Line }
func (i *PrefixExpression) Column() int { return i.Token.Column }

type InfixExpression struct {
	Token    token.Token
	Left     Expression
	Right    Expression
	Operator string
}

// method on infix expression
func (i *InfixExpression) expressionNode()      {}
func (i *InfixExpression) TokenLiteral() string { return i.Token.Literal }
func (i *InfixExpression) String() string {
	var out bytes.Buffer

	out.WriteString("(")
	out.WriteString(i.Left.String())
	out.WriteString(" " + i.Operator + " ")
	out.WriteString(i.Right.String())
	out.WriteString(")")

	return out.String()
}
func (i *InfixExpression) Line() int   { return i.Token.Line }
func (i *InfixExpression) Column() int { return i.Token.Column }

type UpdateExpression struct {
	Token    token.Token
	Operator string
	Target   Expression
	Prefix   bool
}

func (u *UpdateExpression) expressionNode()      {}
func (u *UpdateExpression) TokenLiteral() string { return u.Token.Literal }
func (u *UpdateExpression) String() string {
	var out bytes.Buffer
	if u.Prefix {
		out.WriteString("(")
		out.WriteString(u.Operator)
		out.WriteString(u.Target.String())
		out.WriteString(")")
		return out.String()
	}

	out.WriteString("(")
	out.WriteString(u.Target.String())
	out.WriteString(u.Operator)
	out.WriteString(")")
	return out.String()
}
func (u *UpdateExpression) Line() int   { return u.Token.Line }
func (u *UpdateExpression) Column() int { return u.Token.Column }

type Identifier struct {
	Token token.Token
	Value string
}

// methods on the Identifier
func (i *Identifier) expressionNode()      {}
func (i *Identifier) TokenLiteral() string { return i.Token.Literal }
func (i *Identifier) String() string       { return i.Value }
func (i *Identifier) Line() int            { return i.Token.Line }
func (i *Identifier) Column() int          { return i.Token.Column }

type IntegerLiteral struct {
	Token token.Token
	Value int64
}

// methods on Interger Literal
func (i *IntegerLiteral) expressionNode()      {}
func (i *IntegerLiteral) TokenLiteral() string { return i.Token.Literal }
func (i *IntegerLiteral) String() string       { return i.Token.Literal }
func (i *IntegerLiteral) Line() int            { return i.Token.Line }
func (i *IntegerLiteral) Column() int          { return i.Token.Column }

type ExpressionStatement struct {
	Token      token.Token
	Expression Expression
}

// method on the expression statement
func (i *ExpressionStatement) statementNode()       {}
func (i *ExpressionStatement) TokenLiteral() string { return i.Token.Literal }
func (i *ExpressionStatement) String() string {
	if i.Expression != nil {
		return i.Expression.String()
	}
	return ""
}
func (i *ExpressionStatement) Line() int   { return i.Token.Line }
func (i *ExpressionStatement) Column() int { return i.Token.Column }

func (p *Program) TokenLiteral() string {
	if len(p.Statements) > 0 {
		return p.Statements[0].TokenLiteral()
	} else {
		return ""
	}
}

func (p *Program) String() string {
	var out bytes.Buffer

	for _, s := range p.Statements {
		out.WriteString(s.String())
	}

	return out.String()
}
func (p *Program) Line() int {
	if len(p.Statements) > 0 {
		return p.Statements[0].Line()
	}
	return 0
}
func (p *Program) Column() int { return p.Statements[0].Column() }

// Boolean
type Boolean struct {
	Token token.Token
	Value bool
}

func (b *Boolean) expressionNode()      {}
func (b *Boolean) TokenLiteral() string { return b.Token.Literal }
func (b *Boolean) String() string       { return b.Token.Literal }
func (b *Boolean) Line() int            { return b.Token.Line }
func (b *Boolean) Column() int          { return b.Token.Column }

type BlockStatement struct {
	Token      token.Token
	Statements []Statement
}

// method on the block statement
func (bs *BlockStatement) statementNode()       {}
func (bs *BlockStatement) TokenLiteral() string { return bs.Token.Literal }
func (bs *BlockStatement) String() string {
	var out bytes.Buffer
	for _, s := range bs.Statements {
		out.WriteString(s.String())
	}

	return out.String()
}
func (bs *BlockStatement) Line() int   { return bs.Token.Line }
func (bs *BlockStatement) Column() int { return bs.Token.Column }

// call expression
type CallExpression struct {
	Token     token.Token
	Function  Expression
	Arguments []Expression
}

// methods on the call expression
func (ce *CallExpression) expressionNode()      {}
func (ce *CallExpression) TokenLiteral() string { return ce.Token.Literal }
func (ce *CallExpression) String() string {
	var out bytes.Buffer
	args := []string{}
	for _, a := range ce.Arguments {
		args = append(args, a.String())
	}
	out.WriteString(ce.Function.String())
	out.WriteString("(")
	out.WriteString(strings.Join(args, ", "))
	out.WriteString(")")
	return out.String()
}
func (ce *CallExpression) Line() int   { return ce.Token.Line }
func (ce *CallExpression) Column() int { return ce.Token.Column }

// strings
type StringLiteral struct {
	Token token.Token
	Value string
}

// method on string
func (sl *StringLiteral) expressionNode()      {}
func (sl *StringLiteral) TokenLiteral() string { return sl.Token.Literal }
func (sl *StringLiteral) String() string       { return sl.Token.Literal }
func (sl *StringLiteral) Line() int            { return sl.Token.Line }
func (sl *StringLiteral) Column() int          { return sl.Token.Column }

// char
type CharLiteral struct {
	Token token.Token
	Value rune
}

func (sl *CharLiteral) expressionNode()      {}
func (sl *CharLiteral) TokenLiteral() string { return sl.Token.Literal }
func (sl *CharLiteral) String() string       { return sl.Token.Literal }
func (sl *CharLiteral) Line() int            { return sl.Token.Line }
func (sl *CharLiteral) Column() int          { return sl.Token.Column }

// float
type FloatLiteral struct {
	Token token.Token
	Value float64
}

func (sl *FloatLiteral) expressionNode()      {}
func (sl *FloatLiteral) TokenLiteral() string { return sl.Token.Literal }
func (sl *FloatLiteral) String() string       { return sl.Token.Literal }
func (sl *FloatLiteral) Line() int            { return sl.Token.Line }
func (sl *FloatLiteral) Column() int          { return sl.Token.Column }

type MemberExpression struct {
	Token    token.Token
	Object   Expression
	Property *Identifier
}

func (m *MemberExpression) expressionNode()      {}
func (m *MemberExpression) TokenLiteral() string { return m.Token.Literal }
func (m *MemberExpression) String() string {
	var out bytes.Buffer

	out.WriteString(m.Object.String())
	out.WriteString(".")
	out.WriteString(m.Property.String())

	return out.String()
}
func (m *MemberExpression) Line() int   { return m.Token.Line }
func (m *MemberExpression) Column() int { return m.Token.Column }

type BreakStatement struct {
	Token token.Token
}

func (bs *BreakStatement) statementNode()       {}
func (bs *BreakStatement) TokenLiteral() string { return bs.Token.Literal }
func (bs *BreakStatement) String() string       { return "break;" }
func (bs *BreakStatement) Line() int            { return bs.Token.Line }
func (bs *BreakStatement) Column() int          { return bs.Token.Column }

type ContinueStatement struct {
	Token token.Token
}

func (cs *ContinueStatement) statementNode()       {}
func (cs *ContinueStatement) TokenLiteral() string { return cs.Token.Literal }
func (cs *ContinueStatement) String() string       { return "continue;" }
func (cs *ContinueStatement) Line() int            { return cs.Token.Line }
func (cs *ContinueStatement) Column() int          { return cs.Token.Column }
