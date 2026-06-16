package ast

import (
	"bytes"

	"github.com/walonCode/code-lang/internal/token"
)

type ImportStatement struct {
	Token token.Token
	Path string
}
func (i *ImportStatement) statementNode()      {}
func (i *ImportStatement) TokenLiteral() string { return i.Token.Literal }
func (i *ImportStatement) String() string {
	var out bytes.Buffer
	
	out.WriteString("import")
	out.WriteString(" ")
	out.WriteString(i.Path)
	
	return out.String()
}
func (i *ImportStatement) Line() int   { return i.Token.Line }
func (i *ImportStatement) Column() int { return i.Token.Column }