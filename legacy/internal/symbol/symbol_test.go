package symbol

import (
	"testing"

	"github.com/walonCode/code-lang/internal/lexer"
	"github.com/walonCode/code-lang/internal/parser"
)

func TestSymbolTable(t *testing.T) {
	input := `
let x = 10;
let y = 20;

fn(a, b) {
	let c = a + b;
	return c;
};

struct MyStruct {
	field1: 1,
	field2: "hello",
};

if (x > 0) {
	let d = 30;
} else {
	let e = 40;
};

for (let i = 0; i < 10; i = i + 1) {
	let f = i;
};
`
	l := lexer.New(input)
	p := parser.New(l)
	program := p.ParsePrograme()

	if len(p.Errors()) != 0 {
		t.Fatalf("parser has %d errors: %v", len(p.Errors()), p.Errors())
	}

	builder := NewBuilder()
	builder.Visit(program)

	// Check global symbols
	expectedGlobal := []struct {
		name string
		kind SymbolKind
	}{
		{"x", VARIABLE},
		{"y", VARIABLE},
		{"MyStruct", STRUCT},
	}

	for _, exp := range expectedGlobal {
		sym := builder.Global.Resolve(exp.name)
		if sym == nil {
			t.Errorf("global symbol %s not found", exp.name)
			continue
		}
		if sym.Kind != exp.kind {
			t.Errorf("global symbol %s has kind %s, expected %s", exp.name, sym.Kind, exp.kind)
		}
	}

	// Check resolution in current scope (which should be global at the end)
	if builder.Current != builder.Global {
		t.Errorf("current scope is not global after visit")
	}
}

func TestNestedScopes(t *testing.T) {
	input := `
let x = 10;
let f = fn(a) {
	let b = 20;
	if (a > 0) {
		let c = 30;
	}
	return b;
};
`
	l := lexer.New(input)
	p := parser.New(l)
	program := p.ParsePrograme()

	builder := NewBuilder()
	builder.Visit(program)

	// We can't easily check nested scopes without more introspection tools or complex traversal
	// But we can check if x and f are in global
	if builder.Global.Resolve("x") == nil {
		t.Errorf("x not found in global")
	}
	if builder.Global.Resolve("f") == nil {
		t.Errorf("f not found in global")
	}
}

func TestConstants(t *testing.T) {
	input := `
const GRAVITY = 9.8;
let x = 10;
`
	l := lexer.New(input)
	p := parser.New(l)
	program := p.ParsePrograme()

	builder := NewBuilder()
	builder.Visit(program)

	sym := builder.Global.Resolve("GRAVITY")
	if sym == nil {
		t.Errorf("constant GRAVITY not found")
		return
	}
	if sym.Kind != CONSTANT {
		t.Errorf("GRAVITY has kind %s, expected CONSTANT", sym.Kind)
	}

	symX := builder.Global.Resolve("x")
	if symX.Kind != VARIABLE {
		t.Errorf("x has kind %s, expected VARIABLE", symX.Kind)
	}
}
