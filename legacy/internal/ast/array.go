package ast

import (
	"bytes"
	"strings"

	"github.com/walonCode/code-lang/internal/token"
)

// array
type ArrayLiteral struct {
	Token    token.Token
	Elements []Expression
}

// method on Array literal
func (al *ArrayLiteral) expressionNode()      {}
func (al *ArrayLiteral) TokenLiteral() string { return al.Token.Literal }
func (al *ArrayLiteral) String() string {
	var out bytes.Buffer
	elements := []string{}
	for _, el := range al.Elements {
		elements = append(elements, el.String())
	}
	out.WriteString("[")
	out.WriteString(strings.Join(elements, ", "))
	out.WriteString("]")
	return out.String()
}
func (al *ArrayLiteral) Line() int   { return al.Token.Line }
func (al *ArrayLiteral) Column() int { return al.Token.Column }

// array index expression
type IndexExpression struct {
	Token token.Token //[
	Left  Expression
	Index Expression
}

// method on array index
func (ie *IndexExpression) expressionNode()      {}
func (ie *IndexExpression) TokenLiteral() string { return ie.Token.Literal }
func (ie *IndexExpression) String() string {
	var out bytes.Buffer
	out.WriteString("(")
	out.WriteString(ie.Left.String())
	out.WriteString("[")
	out.WriteString(ie.Index.String())
	out.WriteString("])")
	return out.String()
}
func (ie *IndexExpression) Line() int   { return ie.Token.Line }
func (ie *IndexExpression) Column() int { return ie.Token.Column }