package ast

import (
	"bytes"

	"github.com/walonCode/code-lang/internal/token"
)

type WhileExpression struct {
	Token token.Token
	Condition Expression
	Body *BlockStatement
}
func (w *WhileExpression) expressionNode()      {}
func (w *WhileExpression) TokenLiteral() string { return w.Token.Literal }
func (w *WhileExpression) String() string {
	var out bytes.Buffer
	
	out.WriteString("while")
	out.WriteString(" ")
	out.WriteString(w.Condition.String())
	out.WriteString(" ")
	out.WriteString(w.Body.String())
	
	return out.String()
}
func (w *WhileExpression) Line() int   { return w.Token.Line }
func (w *WhileExpression) Column() int { return w.Token.Column }