package ast

import (
	"bytes"

	"github.com/walonCode/code-lang/internal/token"
)

type ELSE_IF struct {
	Condition   Expression
	Consequence *BlockStatement
}

type IfExpression struct {
	Token       token.Token
	Condition   Expression
	Consequence *BlockStatement
	IfElse      []*ELSE_IF
	Alternative *BlockStatement
}

// methods on the if expression
func (i *IfExpression) expressionNode()      {}
func (i *IfExpression) TokenLiteral() string { return i.Token.Literal }
func (i *IfExpression) String() string {
	var out bytes.Buffer

	out.WriteString("if")
	out.WriteString(i.Condition.String())
	out.WriteString(" ")
	out.WriteString(i.Consequence.String())

	for _, v := range i.IfElse {
		out.WriteString("elseif")
		out.WriteString(v.Condition.String())
		out.WriteString("")
		out.WriteString(v.Consequence.String())
	}

	if i.Alternative != nil {
		out.WriteString("else")
		out.WriteString(i.Alternative.String())
	}

	return out.String()
}
func (i *IfExpression) Line() int   { return i.Token.Line }
func (i *IfExpression) Column() int { return i.Token.Column }