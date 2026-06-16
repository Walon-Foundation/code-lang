package lexer

import (
	"testing"

	"github.com/walonCode/code-lang/internal/token"
)

func TestNextToken(t *testing.T) {
	input := `let five = 5;
	let ten = 10;
	
	let add = fn(x,y){
		x + y;
	};
	
	let result = add(five, ten);
	!-*/5;
	5 < 10 > 5;
	
	if (5 < 10){
		return true;
	}else {
		return false;
	};
	10 == 10;
	10 != 9;
	10 >= 9;
	10 <= 9;
	"foobar"
	"foo bar"
	'*'
	'ab'
	'a'
	555.666
	5.6
	.555
	.5
	[1,2];
	{"foo":"bar"}
	//
	**
	%
	elseif
	for 
	while
	break
	continue
	#
	/* */
	+=
	-=
	*=
	/=
	%=
	++
	--
	user.name
	import "walon"
	struct
	||
	&&
	`

	test := []struct {
		expectedType    token.TokenType
		expectedLiteral string
	}{
		{token.LET, "let"},
		{token.IDENT, "five"},
		{token.ASSIGN, "="},
		{token.INT, "5"},
		{token.SEMICOLON, ";"},
		{token.LET, "let"},
		{token.IDENT, "ten"},
		{token.ASSIGN, "="},
		{token.INT, "10"},
		{token.SEMICOLON, ";"},
		{token.LET, "let"},
		{token.IDENT, "add"},
		{token.ASSIGN, "="},
		{token.FUNCTION, "fn"},
		{token.LPAREN, "("},
		{token.IDENT, "x"},
		{token.COMMA, ","},
		{token.IDENT, "y"},
		{token.RPAREN, ")"},
		{token.LBRACE, "{"},
		{token.IDENT, "x"},
		{token.PLUS, "+"},
		{token.IDENT, "y"},
		{token.SEMICOLON, ";"},
		{token.RBRACE, "}"},
		{token.SEMICOLON, ";"},
		{token.LET, "let"},
		{token.IDENT, "result"},
		{token.ASSIGN, "="},
		{token.IDENT, "add"},
		{token.LPAREN, "("},
		{token.IDENT, "five"},
		{token.COMMA, ","},
		{token.IDENT, "ten"},
		{token.RPAREN, ")"},
		{token.SEMICOLON, ";"},
		{token.BANG, "!"},
		{token.MINUS, "-"},
		{token.ASTERISK, "*"},
		{token.SLASH, "/"},
		{token.INT, "5"},
		{token.SEMICOLON, ";"},
		{token.INT, "5"},
		{token.LT, "<"},
		{token.INT, "10"},
		{token.GT, ">"},
		{token.INT, "5"},
		{token.SEMICOLON, ";"},
		{token.IF, "if"},
		{token.LPAREN, "("},
		{token.INT, "5"},
		{token.LT, "<"},
		{token.INT, "10"},
		{token.RPAREN, ")"},
		{token.LBRACE, "{"},
		{token.RETURN, "return"},
		{token.TRUE, "true"},
		{token.SEMICOLON, ";"},
		{token.RBRACE, "}"},
		{token.ELSE, "else"},
		{token.LBRACE, "{"},
		{token.RETURN, "return"},
		{token.FALSE, "false"},
		{token.SEMICOLON, ";"},
		{token.RBRACE, "}"},
		{token.SEMICOLON, ";"},
		{token.INT, "10"},
		{token.EQ, "=="},
		{token.INT, "10"},
		{token.SEMICOLON, ";"},
		{token.INT, "10"},
		{token.NOT_EQ, "!="},
		{token.INT, "9"},
		{token.SEMICOLON, ";"},
		{token.INT, "10"},
		{token.GREATER_THAN_EQUAL, ">="},
		{token.INT, "9"},
		{token.SEMICOLON, ";"},
		{token.INT, "10"},
		{token.LESS_THAN_EQUAL, "<="},
		{token.INT, "9"},
		{token.SEMICOLON, ";"},
		{token.STRING, "foobar"},
		{token.STRING, "foo bar"},
		{token.CHAR, "*"},
		{token.CHAR, "ILLEGAL"},
		{token.CHAR, "a"},
		{token.FLOAT, "555.666"},
		{token.FLOAT, "5.6"},
		{token.FLOAT, ".555"},
		{token.FLOAT, ".5"},
		{token.LBRACKET, "["},
		{token.INT, "1"},
		{token.COMMA, ","},
		{token.INT, "2"},
		{token.RBRACKET, "]"},
		{token.SEMICOLON, ";"},
		// {"foo":"bar"}
		{token.LBRACE, "{"},
		{token.STRING, "foo"},
		{token.COLON, ":"},
		{token.STRING, "bar"},
		{token.RBRACE, "}"},
		{token.FLOOR, "//"},
		{token.SQUARE, "**"},
		{token.REM, "%"},
		{token.ELSE_IF, "elseif"},
		{token.FOR, "for"},
		{token.WHILE, "while"},
		{token.BREAK, "break"},
		{token.CONTINUE, "continue"},
		{token.ADD_ASSIGN, "+="},
		{token.SUB_ASSIGN, "-="},
		{token.MUL_ASSIGN, "*="},
		{token.QUO_ASSIGN, "/="},
		{token.REM_ASSIGN, "%="},
		{token.INC, "++"},
		{token.DEC, "--"},
		{token.IDENT, "user"},
		{token.DOT, "."},
		{token.IDENT, "name"},
		{token.IMPORT, "import"},
		{token.STRING, "walon"},
		{token.STRUCT, "struct"},
		{token.OR, "||"},
		{token.AND, "&&"},
		{token.EOF, ""},
	}

	l := New(input)

	for i, tt := range test {
		tok := l.NextToken()
		// println(tok.Literal, tt.expectedLiteral)
		if tok.Type != tt.expectedType {
			t.Fatalf("tests[%d] - tokentype wrong. expected=%v, got=%v",
				i, tt.expectedType, tok.Type)
		}

		if tok.Literal != tt.expectedLiteral {
			t.Fatalf("tests[%d] - literal wrong. expected=%q, got=%q",
				i, tt.expectedLiteral, tok.Literal)
		}
	}
}

func TestLineAndColumn(t *testing.T) {
	input := `let x = 5;
let y = 10;
let add = fn(x, y) {
    x + y;
};

let result = add(x, y);
`

	tests := []struct {
		expectedType    token.TokenType
		expectedLiteral string
		expectedLine    int
		expectedColumn  int
	}{
		{token.LET, "let", 1, 1},
		{token.IDENT, "x", 1, 5},
		{token.ASSIGN, "=", 1, 7},
		{token.INT, "5", 1, 9},
		{token.SEMICOLON, ";", 1, 10},
		{token.LET, "let", 2, 1},
		{token.IDENT, "y", 2, 5},
		{token.ASSIGN, "=", 2, 7},
		{token.INT, "10", 2, 9},
		{token.SEMICOLON, ";", 2, 11},
		{token.LET, "let", 3, 1},
		{token.IDENT, "add", 3, 5},
		{token.ASSIGN, "=", 3, 9},
		{token.FUNCTION, "fn", 3, 11},
		{token.LPAREN, "(", 3, 13},
		{token.IDENT, "x", 3, 14},
		{token.COMMA, ",", 3, 15},
		{token.IDENT, "y", 3, 17},
		{token.RPAREN, ")", 3, 18},
		{token.LBRACE, "{", 3, 20},
		{token.IDENT, "x", 4, 5},
		{token.PLUS, "+", 4, 7},
		{token.IDENT, "y", 4, 9},
		{token.SEMICOLON, ";", 4, 10},
		{token.RBRACE, "}", 5, 1},
		{token.SEMICOLON, ";", 5, 2},
		{token.LET, "let", 7, 1},
		{token.IDENT, "result", 7, 5},
		{token.ASSIGN, "=", 7, 12},
		{token.IDENT, "add", 7, 14},
		{token.LPAREN, "(", 7, 17},
		{token.IDENT, "x", 7, 18},
		{token.COMMA, ",", 7, 19},
		{token.IDENT, "y", 7, 21},
		{token.RPAREN, ")", 7, 22},
		{token.SEMICOLON, ";", 7, 23},
		{token.EOF, "", 8, 1},
	}

	l := New(input)

	for i, tt := range tests {
		tok := l.NextToken()

		if tok.Type != tt.expectedType {
			t.Fatalf("tests[%d] - tokentype wrong. expected=%v, got=%v",
				i, tt.expectedType, tok.Type)
		}

		if tok.Literal != tt.expectedLiteral {
			t.Fatalf("tests[%d] - literal wrong. expected=%q, got=%q",
				i, tt.expectedLiteral, tok.Literal)
		}

		if tok.Line != tt.expectedLine {
			t.Fatalf("tests[%d] - line wrong. expected=%d, got=%d",
				i, tt.expectedLine, tok.Line)
		}

		if tok.Column != tt.expectedColumn {
			t.Fatalf("tests[%d] - column wrong. expected=%d, got=%d",
				i, tt.expectedColumn, tok.Column)
		}
	}
}
