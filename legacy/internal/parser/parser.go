package parser

import (
	"fmt"
	"strconv"

	"github.com/walonCode/code-lang/internal/ast"
	"github.com/walonCode/code-lang/internal/lexer"
	"github.com/walonCode/code-lang/internal/token"
)

const (
	_ int = iota
	LOWEST
	ASSIGN // =
	OR
	AND
	EQUALS      // ==
	LESSGREATER // > or <
	SUM         // +
	PRODUCT     // *
	PREFIX      // -x or !x
	POSTFIX     // x++ or x--
	CALL        //myfunction(x)
	INDEX
	MEMBER
)

var precendeces = map[token.TokenType]int{
	token.EQ:                 EQUALS,
	token.ASSIGN:             ASSIGN,
	token.NOT_EQ:             EQUALS,
	token.LT:                 LESSGREATER,
	token.GT:                 LESSGREATER,
	token.GREATER_THAN_EQUAL: LESSGREATER,
	token.LESS_THAN_EQUAL:    LESSGREATER,
	token.PLUS:               SUM,
	token.MINUS:              SUM,
	token.ADD_ASSIGN:         SUM,
	token.SUB_ASSIGN:         SUM,
	token.SLASH:              PRODUCT,
	token.ASTERISK:           PRODUCT,
	token.MUL_ASSIGN:         PRODUCT,
	token.REM_ASSIGN:         PRODUCT,
	token.QUO_ASSIGN:         PRODUCT,
	token.FLOOR:              PRODUCT,
	token.REM:                PRODUCT,
	token.SQUARE:             PRODUCT,
	token.INC:                POSTFIX,
	token.DEC:                POSTFIX,
	token.LPAREN:             CALL,
	token.LBRACKET:           INDEX,
	token.DOT:                MEMBER,
	token.AND:                AND,
	token.OR:                 OR,
}

func (p *Parser) peekPredences() int {
	if p, ok := precendeces[p.peekToken.Type]; ok {
		return p
	}

	return LOWEST
}

func (p *Parser) curPrecendence() int {
	if p, ok := precendeces[p.curToken.Type]; ok {
		return p
	}

	return LOWEST
}

// Pratt Parser
type (
	prefixParseFn func() ast.Expression
	infixParseFn  func(ast.Expression) ast.Expression
)

type Parser struct {
	l *lexer.Lexer

	errors []string

	curToken  token.Token
	peekToken token.Token

	prefixParseFns map[token.TokenType]prefixParseFn
	infixParseFns  map[token.TokenType]infixParseFn
}

func (p *Parser) registerPrefix(tokenType token.TokenType, fn prefixParseFn) {
	p.prefixParseFns[tokenType] = fn
}

func (p *Parser) registerInfix(tokenType token.TokenType, fn infixParseFn) {
	p.infixParseFns[tokenType] = fn
}

func (p *Parser) nextToken() {
	p.curToken = p.peekToken
	p.peekToken = p.l.NextToken()
}

func (p *Parser) parseStatement() ast.Statement {
	switch p.curToken.Type {
	case token.LET:
		return p.parseLetStatement()
	case token.RETURN:
		return p.parseReturnStatement()
	case token.IMPORT:
		return p.parseImportStatement()
	case token.STRUCT:
		return p.parseStructStatement()
	case token.BREAK:
		return p.parseBreakStatement()
	case token.CONTINUE:
		return p.parseContinueStatement()
	case token.CONST:
		return p.parseConstStatement()
	default:
		return p.parseExpressionStatement()
	}
}

func (p *Parser) parseBreakStatement() *ast.BreakStatement {
	stmt := &ast.BreakStatement{Token: p.curToken}
	p.expectPeek(token.SEMICOLON)
	return stmt
}

func (p *Parser) parseContinueStatement() *ast.ContinueStatement {
	stmt := &ast.ContinueStatement{Token: p.curToken}
	p.expectPeek(token.SEMICOLON)
	return stmt
}

func (p *Parser) parseStructStatement() *ast.StructStatement {
	stmt := &ast.StructStatement{Token: p.curToken}

	if !p.expectPeek(token.IDENT) {
		return nil
	}

	stmt.Name = &ast.Identifier{
		Token: p.curToken,
		Value: p.curToken.Literal,
	}

	if !p.expectPeek(token.LBRACE) {
		return nil
	}

	stmt.Fields = make(map[string]ast.Expression)

	for !p.peekTokenIs(token.RBRACE) {
		p.nextToken()
		key := p.curToken.Literal

		if !p.expectPeek(token.COLON) {
			return nil
		}

		p.nextToken()
		value := p.parseExpression(LOWEST)

		stmt.Fields[key] = value

		if p.peekTokenIs(token.COMMA) {
			p.nextToken()
		}
	}

	if !p.expectPeek(token.RBRACE) {
		return nil
	}

	if !p.expectPeek(token.SEMICOLON) {
		return nil
	}

	return stmt
}

func (p *Parser) parseImportStatement() *ast.ImportStatement {
	exp := &ast.ImportStatement{Token: p.curToken}

	if !p.expectPeek(token.STRING) {
		return nil
	}

	exp.Path = p.curToken.Literal

	if !p.expectPeek(token.SEMICOLON) {
		return nil
	}

	return exp
}

func (p *Parser) parseExpressionStatement() *ast.ExpressionStatement {
	// defer untrace(trace("parseExpressionStatement"))
	stmt := &ast.ExpressionStatement{Token: p.curToken}
	stmt.Expression = p.parseExpression(LOWEST)

	if !p.expectPeek(token.SEMICOLON) {
		return nil
	}

	return stmt
}

func (p *Parser) parseFloatLiteral() ast.Expression {
	exp := &ast.FloatLiteral{Token: p.curToken}

	value, err := strconv.ParseFloat(p.curToken.Literal, 64)
	if err != nil {
		msg := fmt.Sprintf("could not parse %q as float", p.curToken.Literal)
		p.errors = append(p.errors, msg)
		return nil
	}

	exp.Value = value
	return exp
}

func (p *Parser) parseIntegerLiteral() ast.Expression {
	// defer untrace(trace("parseIntegerLiteral"))
	il := &ast.IntegerLiteral{Token: p.curToken}

	value, err := strconv.Atoi(p.curToken.Literal)
	if err != nil {
		msg := fmt.Sprintf("could not parse %q as integer", p.curToken.Literal)
		p.errors = append(p.errors, msg)
		return nil
	}

	il.Value = int64(value)

	return il
}

func (p *Parser) noPrefixParseError(t token.TokenType) {
	msg := fmt.Sprintf("[Line %d, Column %d]no prefix parse function for %s found", p.curToken.Line, p.curToken.Column, t)
	p.errors = append(p.errors, msg)
}

func (p *Parser) parseExpression(predence int) ast.Expression {
	// defer untrace(trace("parseExpression"))
	prefix := p.prefixParseFns[p.curToken.Type]
	if prefix == nil {
		p.noPrefixParseError(p.curToken.Type)
		return nil
	}

	leftExp := prefix()

	if p.peekTokenIs(token.LBRACE) {
		return p.parseStructLiteral(leftExp)
	}

	for !p.peekTokenIs(token.SEMICOLON) && predence < p.peekPredences() {
		infix := p.infixParseFns[p.peekToken.Type]
		if infix == nil {
			return leftExp
		}

		p.nextToken()

		leftExp = infix(leftExp)
	}

	return leftExp
}

func (p *Parser) parseStructLiteral(name ast.Expression) ast.Expression {
	ident, ok := name.(*ast.Identifier)
	if !ok {
		return nil
	}

	lit := &ast.StructLiteral{
		Token:  p.peekToken,
		Name:   ident,
		Fields: make(map[string]ast.Expression),
	}

	p.expectPeek(token.LBRACE)

	for !p.peekTokenIs(token.RBRACE) {
		p.nextToken()
		key := p.curToken.Literal

		p.expectPeek(token.COLON)
		p.nextToken()

		value := p.parseExpression(LOWEST)
		lit.Fields[key] = value

		if p.peekTokenIs(token.COMMA) {
			p.nextToken()
		}
	}

	p.expectPeek(token.RBRACE)
	return lit
}

func (p *Parser) parseReturnStatement() *ast.ReturnStatement {
	stmt := &ast.ReturnStatement{Token: p.curToken}

	p.nextToken()

	stmt.ReturnValue = p.parseExpression(LOWEST)

	if !p.expectPeek(token.SEMICOLON) {
		return nil
	}

	return stmt
}

func (p *Parser) parseConstStatement() *ast.ConstStatement {
	stmt := &ast.ConstStatement{Token: p.curToken}
	if !p.expectPeek(token.IDENT) {
		return nil
	}

	stmt.Name = &ast.Identifier{Token: p.curToken, Value: p.curToken.Literal}

	if !p.expectPeek(token.ASSIGN) {
		return nil
	}

	p.nextToken()

	stmt.Value = p.parseExpression(LOWEST)

	if !p.expectPeek(token.SEMICOLON) {
		return nil
	}

	return stmt
}

func (p *Parser) parseLetStatement() *ast.LetStatement {
	stmt := &ast.LetStatement{Token: p.curToken}
	if !p.expectPeek(token.IDENT) {
		return nil
	}

	stmt.Name = &ast.Identifier{Token: p.curToken, Value: p.curToken.Literal}

	if !p.expectPeek(token.ASSIGN) {
		return nil
	}

	p.nextToken()

	stmt.Value = p.parseExpression(LOWEST)

	if !p.expectPeek(token.SEMICOLON) {
		return nil
	}

	return stmt
}

func (p *Parser) curTokenIs(t token.TokenType) bool {
	return p.curToken.Type == t
}

func (p *Parser) peekTokenIs(t token.TokenType) bool {
	return p.peekToken.Type == t
}

func (p *Parser) expectPeek(t token.TokenType) bool {
	if p.peekTokenIs(t) {
		p.nextToken()
		return true
	} else {
		p.peekError(t)
		return false
	}
}

func (p *Parser) ParsePrograme() *ast.Program {
	program := &ast.Program{}
	program.Statements = []ast.Statement{}
	for p.curToken.Type != token.EOF {
		stmt := p.parseStatement()
		if stmt != nil {
			program.Statements = append(program.Statements, stmt)
		}
		p.nextToken()
	}

	return program
}

func (p *Parser) Errors() []string {
	return p.errors
}

func (p *Parser) peekError(t token.TokenType) {
	msg := fmt.Sprintf("[Line %d, Column %d]expect next token to be %s, got %s instead",
		p.curToken.Line, p.curToken.Column,
		t, p.peekToken.Type,
	)
	p.errors = append(p.errors, msg)
}

func New(l *lexer.Lexer) *Parser {
	p := &Parser{
		l:      l,
		errors: []string{},
	}

	p.nextToken()
	p.nextToken()

	//prefix
	p.prefixParseFns = make(map[token.TokenType]prefixParseFn)
	p.registerPrefix(token.IDENT, p.parseIdentifier)
	p.registerPrefix(token.INT, p.parseIntegerLiteral)
	p.registerPrefix(token.BANG, p.parsePrefixExpression)
	p.registerPrefix(token.MINUS, p.parsePrefixExpression)
	p.registerPrefix(token.INC, p.parseUpdatePrefixExpression)
	p.registerPrefix(token.DEC, p.parseUpdatePrefixExpression)
	p.registerPrefix(token.FALSE, p.parseBoolean)
	p.registerPrefix(token.TRUE, p.parseBoolean)
	p.registerPrefix(token.LPAREN, p.parseGroupedExpression)
	p.registerPrefix(token.IF, p.parseIfExpression)
	p.registerPrefix(token.FUNCTION, p.parseFunctionLiteral)
	p.registerPrefix(token.STRING, p.parseStringLiteral)
	p.registerPrefix(token.CHAR, p.parseCharLiteral)
	p.registerPrefix(token.FLOAT, p.parseFloatLiteral)
	p.registerPrefix(token.LBRACKET, p.parseArrayLiteral)
	p.registerPrefix(token.LBRACE, p.parseHashLiteral)
	p.registerPrefix(token.FOR, p.parseForExpression)
	p.registerPrefix(token.WHILE, p.parseWhileExpression)

	//infix
	p.infixParseFns = make(map[token.TokenType]infixParseFn)
	p.registerInfix(token.PLUS, p.parseInfixExpression)
	p.registerInfix(token.MINUS, p.parseInfixExpression)
	p.registerInfix(token.SLASH, p.parseInfixExpression)
	p.registerInfix(token.ASTERISK, p.parseInfixExpression)
	p.registerInfix(token.REM, p.parseInfixExpression)
	p.registerInfix(token.SQUARE, p.parseInfixExpression)
	p.registerInfix(token.FLOOR, p.parseInfixExpression)
	p.registerInfix(token.EQ, p.parseInfixExpression)
	p.registerInfix(token.NOT_EQ, p.parseInfixExpression)
	p.registerInfix(token.LT, p.parseInfixExpression)
	p.registerInfix(token.GT, p.parseInfixExpression)
	p.registerInfix(token.AND, p.parseInfixExpression)
	p.registerInfix(token.OR, p.parseInfixExpression)
	p.registerInfix(token.GREATER_THAN_EQUAL, p.parseInfixExpression)
	p.registerInfix(token.LESS_THAN_EQUAL, p.parseInfixExpression)
	p.registerInfix(token.REM_ASSIGN, p.parseInfixExpression)
	p.registerInfix(token.QUO_ASSIGN, p.parseInfixExpression)
	p.registerInfix(token.ADD_ASSIGN, p.parseInfixExpression)
	p.registerInfix(token.SUB_ASSIGN, p.parseInfixExpression)
	p.registerInfix(token.MUL_ASSIGN, p.parseInfixExpression)
	p.registerInfix(token.LPAREN, p.parseCallExpression)
	p.registerInfix(token.LBRACKET, p.parseIndexExpression)
	p.registerInfix(token.ASSIGN, p.parseInfixExpression)
	p.registerInfix(token.DOT, p.parseMemberExpression)
	p.registerInfix(token.INC, p.parseUpdatePostfixExpression)
	p.registerInfix(token.DEC, p.parseUpdatePostfixExpression)
	return p
}

func (p *Parser) parseMemberExpression(left ast.Expression) ast.Expression {
	exp := &ast.MemberExpression{
		Token:  p.curToken,
		Object: left,
	}

	if !p.expectPeek(token.IDENT) {
		return nil
	}

	exp.Property = &ast.Identifier{
		Token: p.curToken,
		Value: p.curToken.Literal,
	}

	return exp
}

func (p *Parser) parseWhileExpression() ast.Expression {
	exp := &ast.WhileExpression{Token: p.curToken}

	if !p.expectPeek(token.LPAREN) {
		return nil
	}

	p.nextToken()

	exp.Condition = p.parseExpression(LOWEST)

	if !p.expectPeek(token.RPAREN) {
		return nil
	}

	if !p.expectPeek(token.LBRACE) {
		return nil
	}

	exp.Body = p.parseBlockStatement()

	return exp
}

func (p *Parser) parseForExpression() ast.Expression {
	exp := &ast.ForExpression{Token: p.curToken}

	if !p.expectPeek(token.LPAREN) {
		return nil
	}

	p.nextToken() // curToken is now Init or ;

	// Init
	if p.curTokenIs(token.SEMICOLON) {
		exp.Init = nil
	} else {
		exp.Init = p.parseStatement()
	}

	// parseStatement for Init (Let/Expression) will have consumed its semicolon.
	// CurToken is now ';'.
	p.nextToken() // Move to Condition

	// Condition
	if p.curTokenIs(token.SEMICOLON) {
		exp.Condition = nil
	} else {
		exp.Condition = p.parseExpression(LOWEST)
		if !p.expectPeek(token.SEMICOLON) {
			return nil
		}
	}

	p.nextToken() // Move past Condition's ; to Post

	// Post
	if p.curTokenIs(token.RPAREN) {
		exp.Post = nil
	} else {
		// Post in for loop usually hasn't got a semicolon.
		// parseStatement requires one, so we parse expression and wrap.
		postExp := p.parseExpression(LOWEST)
		if postExp != nil {
			exp.Post = &ast.ExpressionStatement{Token: p.curToken, Expression: postExp}
		}
	}

	if !p.expectPeek(token.RPAREN) {
		return nil
	}

	if !p.expectPeek(token.LBRACE) {
		return nil
	}

	exp.Body = p.parseBlockStatement()

	return exp
}

func (p *Parser) parseHashLiteral() ast.Expression {
	hash := &ast.HashLiteral{Token: p.curToken}
	hash.Pairs = make(map[ast.Expression]ast.Expression)

	for !p.peekTokenIs(token.RBRACE) {
		p.nextToken()
		key := p.parseExpression(LOWEST)

		if !p.expectPeek(token.COLON) {
			return nil
		}

		p.nextToken()
		value := p.parseExpression(LOWEST)

		hash.Pairs[key] = value

		if !p.peekTokenIs(token.RBRACE) && !p.expectPeek(token.COMMA) {
			return nil
		}
	}

	if !p.expectPeek(token.RBRACE) {
		return nil
	}

	return hash
}

func (p *Parser) parseIndexExpression(left ast.Expression) ast.Expression {
	exp := &ast.IndexExpression{Token: p.curToken, Left: left}
	p.nextToken()

	exp.Index = p.parseExpression(LOWEST)

	if !p.expectPeek(token.RBRACKET) {
		return nil
	}

	return exp
}

func (p *Parser) parseArrayLiteral() ast.Expression {
	array := &ast.ArrayLiteral{Token: p.curToken}
	array.Elements = p.parseExpressionList(token.RBRACKET)

	return array
}

func (p *Parser) parseExpressionList(end token.TokenType) []ast.Expression {
	list := []ast.Expression{}

	if p.peekTokenIs(end) {
		p.nextToken()
		return list
	}

	p.nextToken()
	list = append(list, p.parseExpression(LOWEST))

	for p.peekTokenIs(token.COMMA) {
		p.nextToken()
		p.nextToken()
		list = append(list, p.parseExpression(LOWEST))
	}

	if !p.expectPeek(end) {
		return nil
	}

	return list
}

func (p *Parser) parseStringLiteral() ast.Expression {
	return &ast.StringLiteral{Token: p.curToken, Value: p.curToken.Literal}
}

func (p *Parser) parseCharLiteral() ast.Expression {
	return &ast.CharLiteral{Token: p.curToken, Value: rune(p.curToken.Literal[0])}
}

func (p *Parser) parseCallExpression(function ast.Expression) ast.Expression {
	exp := &ast.CallExpression{Token: p.curToken, Function: function}
	exp.Arguments = p.parseExpressionList(token.RPAREN)
	return exp
}

func (p *Parser) parseFunctionLiteral() ast.Expression {
	exp := &ast.FunctionLiteral{Token: p.curToken}
	if !p.expectPeek(token.LPAREN) {
		return nil
	}

	exp.Parameters = p.parseFunctionParameters()

	if !p.expectPeek(token.LBRACE) {
		return nil
	}

	exp.Body = *p.parseBlockStatement()

	return exp
}

func (p *Parser) parseFunctionParameters() []*ast.Identifier {
	idens := []*ast.Identifier{}

	if p.peekTokenIs(token.RPAREN) {
		p.nextToken()
		return idens
	}

	p.nextToken()

	iden := &ast.Identifier{Token: p.curToken, Value: p.curToken.Literal}
	idens = append(idens, iden)

	for p.peekTokenIs(token.COMMA) {
		p.nextToken()
		p.nextToken()
		iden := &ast.Identifier{Token: p.curToken, Value: p.curToken.Literal}
		idens = append(idens, iden)
	}

	if !p.expectPeek(token.RPAREN) {
		return nil
	}

	return idens
}

func (p *Parser) parseIfExpression() ast.Expression {
	exp := &ast.IfExpression{Token: p.curToken}

	if !p.expectPeek(token.LPAREN) {
		return nil
	}

	p.nextToken()
	exp.Condition = p.parseExpression(LOWEST)

	if !p.expectPeek(token.RPAREN) {
		return nil
	}

	if !p.expectPeek(token.LBRACE) {
		return nil
	}

	exp.Consequence = p.parseBlockStatement()

	for p.peekTokenIs(token.ELSE_IF) {
		p.nextToken()

		if !p.expectPeek(token.LPAREN) {
			return nil
		}

		p.nextToken()

		condition := p.parseExpression(LOWEST)

		if !p.expectPeek(token.RPAREN) {
			return nil
		}
		if !p.expectPeek(token.LBRACE) {
			return nil
		}

		consequence := p.parseBlockStatement()

		exp.IfElse = append(exp.IfElse, &ast.ELSE_IF{
			Condition:   condition,
			Consequence: consequence,
		})
	}

	if p.peekTokenIs(token.ELSE) {
		p.nextToken()

		if !p.expectPeek(token.LBRACE) {
			return nil
		}

		exp.Alternative = p.parseBlockStatement()
	}

	return exp
}

func (p *Parser) parseBlockStatement() *ast.BlockStatement {
	block := &ast.BlockStatement{Token: p.curToken}
	block.Statements = []ast.Statement{}

	p.nextToken()

	for !p.curTokenIs(token.RBRACE) && !p.curTokenIs(token.EOF) {
		stmt := p.parseStatement()
		if stmt != nil {
			block.Statements = append(block.Statements, stmt)
		}
		p.nextToken()
	}

	return block
}

func (p *Parser) parseGroupedExpression() ast.Expression {
	p.nextToken()

	exp := p.parseExpression(LOWEST)

	if !p.expectPeek(token.RPAREN) {
		return nil
	}

	return exp
}

func (p *Parser) parseInfixExpression(left ast.Expression) ast.Expression {
	// defer untrace(trace("parseInfixExpression"))
	expression := &ast.InfixExpression{
		Token:    p.curToken,
		Operator: p.curToken.Literal,
		Left:     left,
	}

	predence := p.curPrecendence()
	p.nextToken()
	expression.Right = p.parseExpression(predence)

	return expression
}

func (p *Parser) parseUpdatePrefixExpression() ast.Expression {
	exp := &ast.UpdateExpression{
		Token:    p.curToken,
		Operator: p.curToken.Literal,
		Prefix:   true,
	}

	p.nextToken()
	exp.Target = p.parseExpression(PREFIX)

	return exp
}

func (p *Parser) parseUpdatePostfixExpression(left ast.Expression) ast.Expression {
	return &ast.UpdateExpression{
		Token:    p.curToken,
		Operator: p.curToken.Literal,
		Target:   left,
		Prefix:   false,
	}
}

func (p *Parser) parsePrefixExpression() ast.Expression {
	// defer untrace(trace("parsePrefixExpression"))
	expression := &ast.PrefixExpression{
		Token:    p.curToken,
		Operator: p.curToken.Literal,
	}

	p.nextToken()

	expression.Right = p.parseExpression(PREFIX)

	return expression
}

func (p *Parser) parseBoolean() ast.Expression {
	return &ast.Boolean{Token: p.curToken, Value: p.curTokenIs(token.TRUE)}
}

func (p *Parser) parseIdentifier() ast.Expression {
	return &ast.Identifier{Token: p.curToken, Value: p.curToken.Literal}
}
