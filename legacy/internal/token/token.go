package token

type TokenType string

type Token struct {
	Type    TokenType
	Literal string
	Line    int
	Column  int
}

const (
	ILLEGAL = "ILLEGAL"
	EOF     = "EOF"
	// Identifiers + literals
	IDENT  = "IDENT" // add, foobar, x, y, ...
	INT    = "INT"   // 1343456
	STRING = "STRING"
	FLOAT  = "FLOAT"
	CHAR   = "CHAR" // 'a'
	// Operators
	ASSIGN   = "="
	PLUS     = "+"
	MINUS    = "-"
	BANG     = "!"
	ASTERISK = "*"

	ADD_ASSIGN = "+="
	SUB_ASSIGN = "-="
	MUL_ASSIGN = "*="
	QUO_ASSIGN = "/="
	REM_ASSIGN = "%="

	REM    = "%"
	SQUARE = "**"
	FLOOR  = "//"
	INC    = "++"
	DEC    = "--"

	//logic operator
	AND = "&&"
	OR  = "||"

	SLASH              = "/"
	LT                 = "<"
	GT                 = ">"
	GREATER_THAN_EQUAL = ">="
	LESS_THAN_EQUAL    = "<="
	EQ                 = "=="
	NOT_EQ             = "!="

	// Delimiters
	COMMA     = ","
	SEMICOLON = ";"
	COLON     = ":"
	LPAREN    = "("
	RPAREN    = ")"
	LBRACE    = "{"
	RBRACE    = "}"
	LBRACKET  = "["
	RBRACKET  = "]"
	// Keywords
	FUNCTION = "FUNCTION"
	LET      = "LET"
	TRUE     = "TRUE"
	FALSE    = "FALSE"
	IF       = "IF"
	ELSE     = "ELSE"
	RETURN   = "RETURN"
	ELSE_IF  = "ELSE_IF"
	FOR      = "FOR"
	WHILE    = "WHILE"
	CONTINUE = "CONTINUE"
	BREAK    = "BREAK"
	STRUCT   = "STRUCT"
	CONST    = "CONST"

	//accessor thing
	DOT = "."

	IMPORT = "IMPORT"

	//comment
	COMMENT             = "#"
	MULTI_COMMENT_START = "/*"
	MULTI_COMMENT_END   = "*/"
)

var keywords = map[string]TokenType{
	"fn":       FUNCTION,
	"let":      LET,
	"true":     TRUE,
	"false":    FALSE,
	"if":       IF,
	"else":     ELSE,
	"elseif":   ELSE_IF,
	"for":      FOR,
	"while":    WHILE,
	"return":   RETURN,
	"break":    BREAK,
	"continue": CONTINUE,
	"import":   IMPORT,
	"struct":   STRUCT,
	"const":    CONST,
}

func LookUpIdent(ident string) TokenType {
	if tok, ok := keywords[ident]; ok {
		return tok
	}
	return IDENT
}
