package lexer

import (
	"strings"

	"github.com/walonCode/code-lang/internal/token"
)

type Lexer struct {
	input        string
	position     int
	readPosition int
	ch           byte
	line         int
	column       int
}

// methods on the lexer
func (l *Lexer) readChar() {
	//check if we reach the end of the input
	if l.readPosition >= len(l.input) {
		//we set ch to 0 call ASCII 0 is NULL
		l.ch = 0
		l.column++
	} else {
		//if not we set the ch to current position
		l.ch = l.input[l.readPosition]
		if l.ch == '\n' {
			l.line++
			l.column = 0
		} else {
			l.column++
		}
	}
	//set position to the current position of ch
	l.position = l.readPosition
	//increament the read position by 1
	l.readPosition++
}

func (l *Lexer) NextToken() token.Token {
	var tok token.Token

	l.skipWhiteSpace()

	currentLine := l.line
	currentColumn := l.column

	switch l.ch {
	case '=':
		if l.peakChar() == '=' {
			ch := l.ch
			l.readChar()
			tok = token.Token{Type: token.EQ, Literal: string(ch) + string(l.ch), Line: currentLine, Column: currentColumn}
		} else {
			tok = newToken(token.ASSIGN, l.ch, currentLine, currentColumn)
		}
	case '(':
		tok = newToken(token.LPAREN, l.ch, currentLine, currentColumn)
	case ')':
		tok = newToken(token.RPAREN, l.ch, currentLine, currentColumn)
	case '+':
		if l.peakChar() == '+' {
			ch := l.ch
			l.readChar()
			tok = token.Token{Type: token.INC, Literal: string(ch) + string(l.ch), Line: currentLine, Column: currentColumn}
		} else if l.peakChar() == '=' {
			ch := l.ch
			l.readChar()
			tok = token.Token{Type: token.ADD_ASSIGN, Literal: string(ch) + string(l.ch), Line: currentLine, Column: currentColumn}
		} else {
			tok = newToken(token.PLUS, l.ch, currentLine, currentColumn)
		}
	case '{':
		tok = newToken(token.LBRACE, l.ch, currentLine, currentColumn)
	case '}':
		tok = newToken(token.RBRACE, l.ch, currentLine, currentColumn)
	case '-':
		if l.peakChar() == '-' {
			ch := l.ch
			l.readChar()
			tok = token.Token{Type: token.DEC, Literal: string(ch) + string(l.ch), Column: currentColumn, Line: currentLine}
		} else if l.peakChar() == '=' {
			ch := l.ch
			l.readChar()
			tok = token.Token{Type: token.SUB_ASSIGN, Literal: string(ch) + string(l.ch), Column: currentColumn, Line: currentLine}
		} else {
			tok = newToken(token.MINUS, l.ch, currentLine, currentColumn)
		}
	case '[':
		tok = newToken(token.LBRACKET, l.ch, currentLine, currentColumn)
	case ']':
		tok = newToken(token.RBRACKET, l.ch, currentLine, currentColumn)
	case '#':
		l.skipSingleLineComment()
		return l.NextToken()
	case '!':
		if l.peakChar() == '=' {
			ch := l.ch
			l.readChar()
			tok = token.Token{Type: token.NOT_EQ, Literal: string(ch) + string(l.ch), Line: currentLine, Column: currentColumn}
		} else {
			tok = newToken(token.BANG, l.ch, currentLine, currentColumn)
		}
	case '/':
		if l.peakChar() == '/' {
			ch := l.ch
			l.readChar()
			tok = token.Token{Type: token.FLOOR, Literal: string(ch) + string(l.ch), Line: currentLine, Column: currentColumn}
		} else if l.peakChar() == '*' {
			l.readChar()
			l.readChar()
			l.skipMultiLneComment()
			return l.NextToken()
		} else if l.peakChar() == '=' {
			ch := l.ch
			l.readChar()
			tok = token.Token{Type: token.QUO_ASSIGN, Literal: string(ch) + string(l.ch), Line: currentLine, Column: currentColumn}
		} else {
			tok = newToken(token.SLASH, l.ch, currentLine, currentColumn)
		}
	case '*':
		if l.peakChar() == '*' {
			ch := l.ch
			l.readChar()
			tok = token.Token{Type: token.SQUARE, Literal: string(ch) + string(l.ch), Line: currentLine, Column: currentColumn}
		} else if l.peakChar() == '=' {
			ch := l.ch
			l.readChar()
			tok = token.Token{Type: token.MUL_ASSIGN, Literal: string(ch) + string(l.ch), Column: currentColumn, Line: currentLine}
		} else {
			tok = newToken(token.ASTERISK, l.ch, currentLine, currentColumn)
		}
	case '<':
		if l.peakChar() == '=' {
			ch := l.ch
			l.readChar()
			tok = token.Token{Type: token.LESS_THAN_EQUAL, Literal: string(ch) + string(l.ch), Line: currentLine, Column: currentColumn}
		} else {
			tok = newToken(token.LT, l.ch, currentLine, currentColumn)
		}
	case '>':
		if l.peakChar() == '=' {
			ch := l.ch
			l.readChar()
			tok = token.Token{Type: token.GREATER_THAN_EQUAL, Literal: string(ch) + string(l.ch), Line: currentLine, Column: currentColumn}
		} else {
			tok = newToken(token.GT, l.ch, currentLine, currentColumn)
		}
	case ';':
		tok = newToken(token.SEMICOLON, l.ch, currentLine, currentColumn)
	case '%':
		if l.peakChar() == '=' {
			ch := l.ch
			l.readChar()
			tok = token.Token{Type: token.REM_ASSIGN, Literal: string(ch) + string(l.ch), Line: currentLine, Column: currentColumn}
		} else {
			tok = newToken(token.REM, l.ch, currentLine, currentColumn)
		}
	case ',':
		tok = newToken(token.COMMA, l.ch, currentLine, currentColumn)
	case '|':
		if l.peakChar() == '|' {
			ch := l.ch
			l.readChar()
			tok = token.Token{Type: token.OR, Literal: string(ch) + string(l.ch), Line: currentLine, Column: currentColumn}
		}
	case '&':
		if l.peakChar() == '&' {
			ch := l.ch
			l.readChar()
			tok = token.Token{Type: token.AND, Literal: string(ch) + string(l.ch), Line: currentLine, Column: currentColumn}
		}
	case '"':
		tok.Type = token.STRING
		tok.Literal = l.readString()
		tok.Line = currentLine
		tok.Column = currentColumn
	case '\'':
		tok.Type = token.CHAR
		tok.Literal = l.readCharType()
		tok.Line = currentLine
		tok.Column = currentColumn
	case '.':
		if isDigit(l.peakChar()) {
			tok.Type = token.FLOAT
			tok.Literal = l.readFloat()
			tok.Line = currentLine
			tok.Column = currentColumn
		} else {
			tok = newToken(token.DOT, l.ch, currentLine, currentColumn)
		}
	case ':':
		tok = newToken(token.COLON, l.ch, currentLine, currentColumn)
	case 0:
		tok.Literal = ""
		tok.Type = token.EOF
		tok.Line = currentLine
		tok.Column = currentColumn
	default:
		if isLetter(l.ch) {
			tok.Literal = l.readIndentifier()
			tok.Type = token.LookUpIdent(tok.Literal)
			tok.Line = currentLine
			tok.Column = currentColumn
			return tok
		} else if isDigit(l.ch) {
			tok.Literal = l.readNumber()
			if strings.Contains(tok.Literal, ".") {
				tok.Type = token.FLOAT
			} else {
				tok.Type = token.INT
			}
			tok.Line = currentLine
			tok.Column = currentColumn
			return tok
		} else {
			tok = newToken(token.ILLEGAL, l.ch, currentLine, currentColumn)
		}
	}

	l.readChar()
	return tok
}

func (l *Lexer) skipSingleLineComment() {
	for l.ch != '\n' && l.ch != 0 {
		l.readChar()
	}
}

func (l *Lexer) skipMultiLneComment() {
	for {
		if l.ch == 0 {
			break
		}

		if l.ch == '*' && l.peakChar() == '/' {
			l.readChar()
			l.readChar()
			break
		}

		l.readChar()
	}
}

func (l *Lexer) readCharType() string {
	l.readChar()
	if l.ch == 0 || l.ch == '\'' {
		return token.ILLEGAL
	}

	value := l.ch
	l.readChar()

	if l.ch != '\'' {
		l.readChar()
		return token.ILLEGAL
	}
	return string(value)
}

func (l *Lexer) readString() string {
	position := l.position + 1
	for {
		l.readChar()
		if l.ch == '"' || l.ch == 0 {
			break
		}
	}
	return l.input[position:l.position]
}

func (l *Lexer) readIndentifier() string {
	position := l.position
	for isLetter(l.ch) || isDigit(l.ch) {
		l.readChar()
	}
	return l.input[position:l.position]
}

func (l *Lexer) readNumber() string {
	position := l.position
	for isDigit(l.ch) {
		l.readChar()
	}

	if l.ch == '.' {
		l.readChar()
		for isDigit(l.ch) {
			l.readChar()
		}
	}

	return l.input[position:l.position]
}

func (l *Lexer) readFloat() string {
	position := l.position
	l.readChar()
	for isDigit(l.ch) {
		l.readChar()
	}

	return l.input[position:l.position]
}

func (l *Lexer) skipWhiteSpace() {
	for l.ch == ' ' || l.ch == '\t' || l.ch == '\n' || l.ch == '\r' {
		l.readChar()
	}
}

func (l *Lexer) peakChar() byte {
	if l.readPosition >= len(l.input) {
		return 0
	} else {
		return l.input[l.readPosition]
	}
}

// helpers
func newToken(tokenType token.TokenType, ch byte, line, column int) token.Token {
	return token.Token{Type: tokenType, Literal: string(ch), Line: line, Column: column}
}

func isLetter(ch byte) bool {
	return 'a' <= ch && ch <= 'z' || 'A' <= ch && ch <= 'Z' || ch == '_'
}

func isDigit(ch byte) bool {
	return '0' <= ch && ch <= '9'
}

func New(input string) *Lexer {
	l := &Lexer{input: input, line: 1, column: 0}
	l.readChar()
	return l
}
