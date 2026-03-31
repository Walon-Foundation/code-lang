package vm

import (
	"fmt"
	"math"
	"testing"

	"github.com/walonCode/code-lang/internal/ast"
	"github.com/walonCode/code-lang/internal/compiler"
	"github.com/walonCode/code-lang/internal/lexer"
	"github.com/walonCode/code-lang/internal/object"
	"github.com/walonCode/code-lang/internal/parser"
)

func parse(input string) *ast.Program {
	l := lexer.New(input)
	p := parser.New(l)
	return p.ParsePrograme()
}

func testIntegerObject(expected int64, actual object.Object) error {
	result, ok := actual.(*object.Integer)
	if !ok {
		return fmt.Errorf("object is not Integer. got=%T (%+v)",
			actual, actual)
	}
	if result.Value != expected {
		return fmt.Errorf("object has wrong value. got=%d, want=%d",
			result.Value, expected)
	}
	return nil
}

type vmTestCase struct {
	input  string
	expect any
}

func runVmTest(t *testing.T, tests []vmTestCase) {
	t.Helper()

	for _, tt := range tests {
		program := parse(tt.input)

		comp := compiler.New()
		err := comp.Compile(program)
		if err != nil {
			t.Fatalf("compiler error: %s", err)
		}

		vm := New(comp.ByteCode())
		err = vm.Run()

		if err != nil {
			t.Fatalf("vm error: %s", err)
		}

		stackElem := vm.LastPoppedStackElm()

		testExpectedObject(t, tt.expect, stackElem)
	}
}

func testExpectedObject(t *testing.T, expected any, actual object.Object) {
	t.Helper()

	switch expected := expected.(type) {
	case int:
		err := testIntegerObject(int64(expected), actual)
		if err != nil {
			t.Errorf("testIntegerObject failed: %s", err)
		}
	case float64:
		err := testFloatObject(expected, actual)
		if err != nil {
			t.Errorf("testFloatObject failed: %s", err)
		}
	case bool:
		err := testBooleanObject(bool(expected), actual)
		if err != nil {
			t.Errorf("testBooleanObject failed: %s", err)
		}
	}
}

func TestIntegerArithmetic(t *testing.T) {
	tests := []vmTestCase{
		{"1;", 1},
		{"2;", 2},
		{"1 + 2;", 3},
		{"4 + 2;", 6},
		{"1 - 2;", -1},
		{"1 * 2;", 2},
		{"4 / 2;", 2},
		{"50 / 2 * 2 + 10 - 5;", 55},
		{"5 + 5 + 5 + 5 - 10;", 10},
		{"2 * 2 * 2 * 2 * 2;", 32},
		{"5 * 2 + 10;", 20},
		{"5 + 2 * 10;", 25},
		{"5 * (2 + 10);", 60},
		{"5 % 2;", 1},
		{"2 ** 3;", 8},
		{"5 // 2;", 2},
		{"-6 // 2;", -3},
	}

	runVmTest(t, tests)
}

func TestFloatArithmetic(t *testing.T) {
	tests := []vmTestCase{
		{"1.5;", 1.5},
		{"1.5 + 2.5;", 4.0},
		{"5.0 - 2.0;", 3.0},
		{"2.5 * 2.0;", 5.0},
		{"5.0 / 2.0;", 2.5},
		{"1 + 2.5;", 3.5},
		{"2.5 + 1;", 3.5},
		{"5 / 2.0;", 2.5},
		{"5.0 / 2;", 2.5},
		{"5 / 2;", 2},
		{"5.0 % 2.0;", 1.0},
		{"2.0 ** 3.0;", 8.0},
		{"5.0 // 2.0;", 2.0},
		{"5 // 2.0;", 2.0},
		{"5.0 // 2;", 2.0},
	}

	runVmTest(t, tests)
}

func TestAssignmentsAndUpdates(t *testing.T) {
	tests := []vmTestCase{
		{"let x = 1; x;", 1},
		{"let x = 1; x = 2; x;", 2},
		{"let x = 1; x += 2; x;", 3},
		{"let x = 1; x++;", 1},
		{"let x = 1; ++x;", 2},
		{"let x = 1.5; x++;", 1.5},
		{"let x = 1.5; ++x;", 2.5},
	}

	runVmTest(t, tests)
}

func testFloatObject(expected float64, actual object.Object) error {
	result, ok := actual.(*object.Float)
	if !ok {
		return fmt.Errorf("object is not Float. got=%T (%+v)",
			actual, actual)
	}
	if math.Abs(result.Value-expected) > 1e-9 {
		return fmt.Errorf("object has wrong value. got=%f, want=%f",
			result.Value, expected)
	}
	return nil
}

func testBooleanObject(expected bool, actual object.Object) error {
	result, ok := actual.(*object.Boolean)
	if !ok {
		return fmt.Errorf("object is not Boolean. got=%T (%+v)",
			actual, actual)
	}
	if result.Value != expected {
		return fmt.Errorf("object has wrong value. got=%t, want=%t",
			result.Value, expected)
	}
	return nil
}

func TestBooleanExpression(t *testing.T) {
	tests := []vmTestCase{
		{"true;", true},
		{"false;", false},
		{"1 < 2;", true},
		{"1 > 2;", false},
		{"1 < 1;", false},
		{"1 > 1;", false},
		{"1 == 1;", true},
		{"1 != 1;", false},
		{"1 == 2;", false},
		{"1 != 2;", true},
		{"true == true;", true},
		{"false == false;", true},
		{"true == false;", false},
		{"true != false;", true},
		{"false != true;", true},
		{"(1 < 2) == true;", true},
		{"(1 < 2) == false;", false},
		{"(1 > 2) == true;", false},
		{"(1 > 2) == false;", true},
		{"5.4 > 4.5;", true},
		{"5.4 >= 5.4;", true},
		{"5.4 <= 5.5;", true},
		{"5.4 < 4.5;", false},
		{"5.4 == 5.4;", true},
		{"5.4 != 4.5;", true},
		{"5.0 >= 4;", true},
		{"4 <= 5.0;", true},
	}

	runVmTest(t, tests)
}

func TestConditionals(t *testing.T){
	tests := []vmTestCase{
		{"if (true){10;};", 10},
		{"if (false) { 10; } else { 20; }; ", 20},
		{"if (1) { 10; };", 10},
		{"if (1 < 2) { 10; };", 10},
		{"if (1 < 2) { 10; } else { 20; };", 10},
		{"if (1 > 2) { 10; } else { 20; };", 20},
		{"if (1 > 2) { 10; } elseif (1 < 2){20;} else { 30; };", 20},
		{"if (1 > 2) { 10; } elseif(1>3){13;} else { 20; };", 20},	
	}
	
	runVmTest(t, tests)
}
