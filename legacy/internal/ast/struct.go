package ast

import (
	"strings"

	"github.com/walonCode/code-lang/internal/token"
)

// struct
type StructStatement struct {
	Token token.Token
	Name *Identifier
	Fields map[string]Expression
}

func (ss *StructStatement) statementNode() {}
func (ss *StructStatement) TokenLiteral() string {
    return ss.Token.Literal
}
func (ss *StructStatement) String() string {
    var out strings.Builder
    out.WriteString("struct ")
    out.WriteString(ss.Name.String())
    out.WriteString(" { ")
    for k, v := range ss.Fields {
        out.WriteString(k)
        out.WriteString(": ")
        out.WriteString(v.String())
        out.WriteString(", ")
    }
    out.WriteString(" }")
    return out.String()
}
func(ss *StructStatement)Line()int { return ss.Token.Line}
func (ss *StructStatement)Column()int { return ss.Token.Column}


type StructLiteral struct {
	Token token.Token
	Name *Identifier
	Fields map[string]Expression
}
func (sl *StructLiteral) expressionNode() {}
func (sl *StructLiteral) TokenLiteral() string {
    return sl.Token.Literal
}
func (sl *StructLiteral) String() string {
    var out strings.Builder
    out.WriteString(sl.Name.String())
    out.WriteString(" { ")
    for k, v := range sl.Fields {
        out.WriteString(k)
        out.WriteString(": ")
        out.WriteString(v.String())
        out.WriteString(", ")
    }
    out.WriteString(" }")
    return out.String()
}
func(ss *StructLiteral)Line()int { return ss.Token.Line}
func (ss *StructLiteral)Column()int { return ss.Token.Column}