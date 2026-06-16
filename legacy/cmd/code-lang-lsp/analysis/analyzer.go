package analysis

import (
	"fmt"
	"strings"
	"unicode/utf8"

	"github.com/walonCode/code-lang/cmd/code-lang-lsp/lsp"
	"github.com/walonCode/code-lang/internal/ast"
	"github.com/walonCode/code-lang/internal/lexer"
	"github.com/walonCode/code-lang/internal/parser"
	"github.com/walonCode/code-lang/internal/symbol"
)

type Document struct {
	URI          string
	Text         string
	Program      *ast.Program
	ParserErrors []string
	SymbolErrors []string
	Index        *Index
}

type Index struct {
	Definitions []*Definition
	References  []*Reference
	Occurrences []*Occurrence
	DefsByName  map[string][]*Definition
	RefsByDef   map[*Definition][]*Reference
	Scopes      []*ScopeInfo
	MemberProps []*Occurrence
	Imports     map[string]bool
}

type Definition struct {
	Name  string
	Kind  symbol.SymbolKind
	Range lsp.Range
	URI   string
}

type Reference struct {
	Name  string
	Range lsp.Range
	Def   *Definition
	URI   string
}

type Occurrence struct {
	Name         string
	Range        lsp.Range
	IsDefinition bool
	Def          *Definition
	Kind         symbol.SymbolKind
}

type ScopeInfo struct {
	Range lsp.Range
	Defs  map[string]*Definition
	Parent *ScopeInfo
}

func Analyze(uri, text string) *Document {
	if strings.TrimSpace(text) == "" {
		return &Document{
			URI:          uri,
			Text:         text,
			Program:      nil,
			ParserErrors: nil,
			SymbolErrors: nil,
			Index: &Index{
				Definitions: []*Definition{},
				References:  []*Reference{},
				Occurrences: []*Occurrence{},
				DefsByName:  make(map[string][]*Definition),
				RefsByDef:   make(map[*Definition][]*Reference),
				Scopes:      []*ScopeInfo{},
				MemberProps: []*Occurrence{},
				Imports:     make(map[string]bool),
			},
		}
	}
	l := lexer.New(text)
	p := parser.New(l)
	program := p.ParsePrograme()

	builder := symbol.NewBuilder()
	builder.Visit(program)

	doc := &Document{
		URI:          uri,
		Text:         text,
		Program:      program,
		ParserErrors: p.Errors(),
		SymbolErrors: builder.Errors,
		Index:        BuildIndex(uri, program),
	}

	return doc
}

func (d *Document) Diagnostics() []lsp.Diagnostic {
	var diags []lsp.Diagnostic
	for _, msg := range d.ParserErrors {
		diags = append(diags, diagnosticFromMessage(msg, "parser"))
	}
	for _, msg := range d.SymbolErrors {
		diags = append(diags, diagnosticFromMessage(msg, "symbol"))
	}
	return diags
}

func (d *Document) FindOccurrenceAt(pos lsp.Position) *Occurrence {
	if d == nil || d.Index == nil {
		return nil
	}
	for _, occ := range d.Index.Occurrences {
		if contains(occ.Range, pos) {
			return occ
		}
	}
	return nil
}

func (d *Document) DefinitionsFor(pos lsp.Position) []*Definition {
	occ := d.FindOccurrenceAt(pos)
	if occ == nil || occ.Def == nil {
		return nil
	}
	return []*Definition{occ.Def}
}

func (d *Document) ReferencesFor(pos lsp.Position) []*Reference {
	occ := d.FindOccurrenceAt(pos)
	if occ == nil || occ.Def == nil {
		return nil
	}
	return d.Index.RefsByDef[occ.Def]
}

func (d *Document) CompletionAt(pos lsp.Position) []*Definition {
	if d == nil || d.Index == nil {
		return nil
	}
	var best *ScopeInfo
	for _, sc := range d.Index.Scopes {
		if sc == nil {
			continue
		}
		if contains(sc.Range, pos) {
			if best == nil || rangeContains(best.Range, sc.Range) {
				best = sc
			}
		}
	}
	var defs []*Definition
	if best != nil {
		seen := map[string]bool{}
		for curr := best; curr != nil; curr = curr.Parent {
			for name, def := range curr.Defs {
				if !seen[name] {
					seen[name] = true
					defs = append(defs, def)
				}
			}
		}
	}
	if len(defs) == 0 {
		defs = d.Index.Definitions
	}
	return defs
}

type scope struct {
	parent *scope
	defs   map[string]*Definition
	info   *ScopeInfo
}

func newScope(parent *scope) *scope {
	var parentInfo *ScopeInfo
	if parent != nil {
		parentInfo = parent.info
	}
	return &scope{
		parent: parent,
		defs:   make(map[string]*Definition),
		info:   &ScopeInfo{Defs: make(map[string]*Definition), Parent: parentInfo},
	}
}

func (s *scope) define(def *Definition) {
	s.defs[def.Name] = def
	s.info.Defs[def.Name] = def
}

func (s *scope) resolve(name string) *Definition {
	if d, ok := s.defs[name]; ok {
		return d
	}
	if s.parent != nil {
		return s.parent.resolve(name)
	}
	return nil
}

func BuildIndex(uri string, program *ast.Program) *Index {
	idx := &Index{
		Definitions: []*Definition{},
		References:  []*Reference{},
		Occurrences: []*Occurrence{},
		DefsByName:  make(map[string][]*Definition),
		RefsByDef:   make(map[*Definition][]*Reference),
		Scopes:      []*ScopeInfo{},
		MemberProps: []*Occurrence{},
		Imports:     make(map[string]bool),
	}

	scope := newScope(nil)
	idx.Scopes = append(idx.Scopes, scope.info)

	var visitStatement func(stmt ast.Statement)
	var visitExpression func(expr ast.Expression)

	define := func(name string, kind symbol.SymbolKind, line, col int) *Definition {
		if name == "" {
			return nil
		}
		rng := rangeFromLineCol(line, col, runeLen(name))
		def := &Definition{
			Name:  name,
			Kind:  kind,
			Range: rng,
			URI:   uri,
		}
		scope.define(def)
		idx.Definitions = append(idx.Definitions, def)
		idx.DefsByName[name] = append(idx.DefsByName[name], def)
		idx.Occurrences = append(idx.Occurrences, &Occurrence{
			Name:         name,
			Range:        rng,
			IsDefinition: true,
			Def:          def,
			Kind:         kind,
		})
		return def
	}

	addRef := func(name string, line, col int) {
		if name == "" {
			return
		}
		rng := rangeFromLineCol(line, col, runeLen(name))
		def := scope.resolve(name)
		ref := &Reference{
			Name:  name,
			Range: rng,
			Def:   def,
			URI:   uri,
		}
		idx.References = append(idx.References, ref)
		if def != nil {
			idx.RefsByDef[def] = append(idx.RefsByDef[def], ref)
		}
		idx.Occurrences = append(idx.Occurrences, &Occurrence{
			Name:         name,
			Range:        rng,
			IsDefinition: false,
			Def:          def,
			Kind:         defKind(def),
		})
	}

	enterScope := func(start lsp.Range) {
		scope = newScope(scope)
		scope.info.Range = start
		idx.Scopes = append(idx.Scopes, scope.info)
	}
	exitScope := func(end lsp.Position) {
		if scope.info != nil {
			scope.info.Range.End = end
			if positionBefore(scope.info.Range.End, scope.info.Range.Start) {
				scope.info.Range.End = scope.info.Range.Start
			}
		}
		if scope.parent != nil {
			scope = scope.parent
		}
	}

	visitStatement = func(stmt ast.Statement) {
		switch s := stmt.(type) {
		case *ast.LetStatement:
			if s == nil || s.Name == nil {
				return
			}
			kind := symbol.VARIABLE
			if _, ok := s.Value.(*ast.FunctionLiteral); ok {
				kind = symbol.FUNCTION
			}
			define(s.Name.Value, kind, s.Name.Line(), s.Name.Column())
			if s.Value != nil {
				visitExpression(s.Value)
			}
		case *ast.ConstStatement:
			if s == nil || s.Name == nil {
				return
			}
			kind := symbol.CONSTANT
			if _, ok := s.Value.(*ast.FunctionLiteral); ok {
				kind = symbol.FUNCTION
			}
			define(s.Name.Value, kind, s.Name.Line(), s.Name.Column())
			if s.Value != nil {
				visitExpression(s.Value)
			}
		case *ast.ReturnStatement:
			if s != nil && s.ReturnValue != nil {
				visitExpression(s.ReturnValue)
			}
		case *ast.ExpressionStatement:
			if s != nil && s.Expression != nil {
				visitExpression(s.Expression)
			}
		case *ast.BlockStatement:
			if s == nil {
				return
			}
			enterScope(rangeFromLineCol(s.Line(), s.Column(), 1))
			for _, st := range s.Statements {
				visitStatement(st)
			}
			exitScope(endPositionOfStatements(s.Statements))
		case *ast.StructStatement:
			if s == nil || s.Name == nil {
				return
			}
			define(s.Name.Value, symbol.STRUCT, s.Name.Line(), s.Name.Column())
			for _, v := range s.Fields {
				visitExpression(v)
			}
		case *ast.ImportStatement:
			if s != nil && s.Path != "" {
				idx.Imports[s.Path] = true
			}
		case *ast.BreakStatement, *ast.ContinueStatement:
			return
		}
	}

	visitExpression = func(expr ast.Expression) {
		switch e := expr.(type) {
		case *ast.Identifier:
			if e != nil {
				addRef(e.Value, e.Line(), e.Column())
			}
		case *ast.IntegerLiteral, *ast.Boolean, *ast.StringLiteral, *ast.FloatLiteral, *ast.CharLiteral:
			return
		case *ast.PrefixExpression:
			if e != nil && e.Right != nil {
				visitExpression(e.Right)
			}
		case *ast.InfixExpression:
			if e != nil {
				if e.Left != nil {
					visitExpression(e.Left)
				}
				if e.Right != nil {
					visitExpression(e.Right)
				}
			}
		case *ast.IfExpression:
			if e == nil {
				return
			}
			visitExpression(e.Condition)
			if e.Consequence != nil {
				visitStatement(e.Consequence)
			}
			for _, elif := range e.IfElse {
				if elif != nil {
					visitExpression(elif.Condition)
					visitStatement(elif.Consequence)
				}
			}
			if e.Alternative != nil {
				visitStatement(e.Alternative)
			}
		case *ast.FunctionLiteral:
			if e == nil {
				return
			}
			enterScope(rangeFromLineCol(e.Line(), e.Column(), 1))
			for _, p := range e.Parameters {
				if p != nil {
					define(p.Value, symbol.PARAMETER, p.Line(), p.Column())
				}
			}
			visitStatement(&e.Body)
			exitScope(endPositionOfStatement(&e.Body))
		case *ast.CallExpression:
			if e != nil {
				visitExpression(e.Function)
				for _, a := range e.Arguments {
					visitExpression(a)
				}
			}
		case *ast.ArrayLiteral:
			if e != nil {
				for _, el := range e.Elements {
					visitExpression(el)
				}
			}
		case *ast.IndexExpression:
			if e != nil {
				visitExpression(e.Left)
				visitExpression(e.Index)
			}
		case *ast.HashLiteral:
			if e != nil {
				for k, v := range e.Pairs {
					visitExpression(k)
					visitExpression(v)
				}
			}
		case *ast.StructLiteral:
			if e != nil {
				if e.Name != nil {
					addRef(e.Name.Value, e.Name.Line(), e.Name.Column())
				}
				for _, v := range e.Fields {
					visitExpression(v)
				}
			}
		case *ast.MemberExpression:
			if e != nil {
				if e.Property != nil {
					rng := rangeFromLineCol(e.Property.Line(), e.Property.Column(), runeLen(e.Property.Value))
					idx.MemberProps = append(idx.MemberProps, &Occurrence{
						Name:         e.Property.Value,
						Range:        rng,
						IsDefinition: false,
						Def:          nil,
						Kind:         symbol.STRUCT_FIELD,
					})
				}
				visitExpression(e.Object)
			}
		case *ast.ForExpression:
			if e == nil {
				return
			}
			enterScope(rangeFromLineCol(e.Line(), e.Column(), 1))
			if e.Init != nil {
				visitStatement(e.Init)
			}
			if e.Condition != nil {
				visitExpression(e.Condition)
			}
			if e.Post != nil {
				visitStatement(e.Post)
			}
			if e.Body != nil {
				visitStatement(e.Body)
			}
			exitScope(endPositionOfStatement(e.Body))
		case *ast.WhileExpression:
			if e == nil {
				return
			}
			enterScope(rangeFromLineCol(e.Line(), e.Column(), 1))
			if e.Condition != nil {
				visitExpression(e.Condition)
			}
			if e.Body != nil {
				visitStatement(e.Body)
			}
			exitScope(endPositionOfStatement(e.Body))
		}
	}

	if program != nil {
		for _, stmt := range program.Statements {
			visitStatement(stmt)
		}
	}

	return idx
}

func defKind(def *Definition) symbol.SymbolKind {
	if def == nil {
		return symbol.VARIABLE
	}
	return def.Kind
}

func diagnosticFromMessage(msg, source string) lsp.Diagnostic {
	line, col, ok := parseLineCol(msg)
	if !ok {
		line = 0
		col = 1
	}
	rng := rangeFromLineCol(line, col, 1)
	return lsp.Diagnostic{
		Range:    rng,
		Severity: 1,
		Source:   "",
		Message:  cleanMessage(msg),
	}
}

func parseLineCol(msg string) (int, int, bool) {
	if !strings.HasPrefix(msg, "[Line ") {
		return 0, 0, false
	}
	var line, col int
	_, err := fmt.Sscanf(msg, "[Line %d, Column %d]", &line, &col)
	if err != nil {
		return 0, 0, false
	}
	return line, col, true
}

func cleanMessage(msg string) string {
	if msg == "" {
		return msg
	}
	if i := strings.Index(msg, "]"); i != -1 && i+1 < len(msg) {
		return strings.TrimSpace(msg[i+1:])
	}
	return strings.TrimSpace(msg)
}

func rangeFromLineCol(line, col, length int) lsp.Range {
	startCol := col - 1
	if startCol < 0 {
		startCol = 0
	}
	if length < 1 {
		length = 1
	}
	endCol := startCol + length
	return lsp.Range{
		Start: lsp.Position{Line: line, Character: startCol},
		End:   lsp.Position{Line: line, Character: endCol},
	}
}

func contains(r lsp.Range, pos lsp.Position) bool {
	if pos.Line < r.Start.Line || pos.Line > r.End.Line {
		return false
	}
	if pos.Line == r.Start.Line && pos.Character < r.Start.Character {
		return false
	}
	if pos.Line == r.End.Line && pos.Character >= r.End.Character {
		return false
	}
	return true
}

func rangeContains(a, b lsp.Range) bool {
	if a.Start.Line > b.Start.Line {
		return false
	}
	if a.End.Line < b.End.Line {
		return false
	}
	if a.Start.Line == b.Start.Line && a.Start.Character > b.Start.Character {
		return false
	}
	if a.End.Line == b.End.Line && a.End.Character < b.End.Character {
		return false
	}
	return true
}

func runeLen(s string) int {
	if s == "" {
		return 0
	}
	return utf8.RuneCountInString(s)
}

func positionBefore(a, b lsp.Position) bool {
	if a.Line < b.Line {
		return true
	}
	if a.Line > b.Line {
		return false
	}
	return a.Character < b.Character
}

func endPositionOfStatements(stmts []ast.Statement) lsp.Position {
	if len(stmts) == 0 {
		return lsp.Position{Line: 0, Character: 0}
	}
	return endPositionOfStatement(stmts[len(stmts)-1])
}

func endPositionOfStatement(stmt ast.Statement) lsp.Position {
	if stmt == nil {
		return lsp.Position{Line: 0, Character: 0}
	}
	switch s := stmt.(type) {
	case *ast.LetStatement:
		if s.Name != nil {
			return lsp.Position{Line: s.Name.Line(), Character: s.Name.Column() - 1 + runeLen(s.Name.Value)}
		}
	case *ast.ConstStatement:
		if s.Name != nil {
			return lsp.Position{Line: s.Name.Line(), Character: s.Name.Column() - 1 + runeLen(s.Name.Value)}
		}
	case *ast.ExpressionStatement:
		if s.Expression != nil {
			return endPositionOfExpression(s.Expression)
		}
	case *ast.BlockStatement:
		return endPositionOfStatements(s.Statements)
	case *ast.StructStatement:
		if s.Name != nil {
			return lsp.Position{Line: s.Name.Line(), Character: s.Name.Column() - 1 + runeLen(s.Name.Value)}
		}
	}
	return lsp.Position{Line: stmt.Line(), Character: stmt.Column()}
}

func endPositionOfExpression(expr ast.Expression) lsp.Position {
	if expr == nil {
		return lsp.Position{Line: 0, Character: 0}
	}
	switch e := expr.(type) {
	case *ast.Identifier:
		return lsp.Position{Line: e.Line(), Character: e.Column() - 1 + runeLen(e.Value)}
	case *ast.StringLiteral:
		return lsp.Position{Line: e.Line(), Character: e.Column()}
	case *ast.IntegerLiteral, *ast.Boolean, *ast.FloatLiteral, *ast.CharLiteral:
		return lsp.Position{Line: expr.Line(), Character: expr.Column()}
	case *ast.MemberExpression:
		if e.Property != nil {
			return lsp.Position{Line: e.Property.Line(), Character: e.Property.Column() - 1 + runeLen(e.Property.Value)}
		}
	}
	return lsp.Position{Line: expr.Line(), Character: expr.Column()}
}
