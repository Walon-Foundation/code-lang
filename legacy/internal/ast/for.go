package ast

import (
	"bytes"

	"github.com/walonCode/code-lang/internal/token"
)

type ForExpression struct {
	Token     token.Token // The 'for' token
	Init      Statement   // e.g., let i = 0;
	Condition Expression  // e.g., i < 10;
	Post      Statement   // e.g., i = i + 1; (usually as an expression statement)
	Body      *BlockStatement
}

func (f *ForExpression) expressionNode()      {}
func (f *ForExpression) TokenLiteral() string { return f.Token.Literal }
func (f *ForExpression) String() string {
	var out bytes.Buffer

	out.WriteString("for")
	out.WriteString("(")
	if f.Init != nil {
		out.WriteString(f.Init.String())
	}
	out.WriteString("; ")
	if f.Condition != nil {
		out.WriteString(f.Condition.String())
	}
	out.WriteString("; ")
	if f.Post != nil {
		out.WriteString(f.Post.String())
	}
	out.WriteString(") ")
	out.WriteString(f.Body.String())

	return out.String()
}
func (f *ForExpression) Line() int   { return f.Token.Line }
func (f *ForExpression) Column() int { return f.Token.Column }