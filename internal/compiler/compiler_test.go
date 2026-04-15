package compiler

import (
	"fmt"
	"math"
	"testing"

	"github.com/walonCode/code-lang/internal/ast"
	"github.com/walonCode/code-lang/internal/code"
	"github.com/walonCode/code-lang/internal/lexer"
	"github.com/walonCode/code-lang/internal/object"
	"github.com/walonCode/code-lang/internal/parser"
)

type compilerTestCase struct {
	input              string
	expectedConstant   []any
	expectInstructions []code.Instructions
}

func TestConditionals(t *testing.T){
	
	tests := []compilerTestCase{
		{
			input: `
			if (true){ 10;}; 3333;
			`,
			expectedConstant: []any{10, 3333},
			expectInstructions: []code.Instructions{
				// 0000
				code.Make(code.OpTrue),
				// 0001
				code.Make(code.OpJumpNotTruthy, 10),
				// 0004
				code.Make(code.OpConstant, 0),
				// 0007
				code.Make(code.OpJump, 11),
				// 0010
				code.Make(code.OpNull),
				// 0011
				code.Make(code.OpPop),
				// 0012
				code.Make(code.OpConstant, 1),
				// 0015
				code.Make(code.OpPop),
			},
		},
		{
			input: `
			if(true){10;} else {20;}; 3333;
			`,
			expectedConstant: []any{10,20,3333},
			expectInstructions: []code.Instructions{
				code.Make(code.OpTrue),
				code.Make(code.OpJumpNotTruthy, 10),
				code.Make(code.OpConstant, 0),
				code.Make(code.OpJump, 13),
				code.Make(code.OpConstant, 1),
				code.Make(code.OpPop),
				code.Make(code.OpConstant, 2),
				code.Make(code.OpPop),
			},
		},
	}
	
	runCompilerTest(t, tests)
}

func TestIntegerArithmetic(t *testing.T) {
	tests := []compilerTestCase{
		{
			input:            "1 + 2;",
			expectedConstant: []any{1, 2},
			expectInstructions: []code.Instructions{
				code.Make(code.OpConstant, 0),
				code.Make(code.OpConstant, 1),
				code.Make(code.OpAdd),
				code.Make(code.OpPop),
			},
		},
		{
			input:            "1 - 2;",
			expectedConstant: []any{1, 2},
			expectInstructions: []code.Instructions{
				code.Make(code.OpConstant, 0),
				code.Make(code.OpConstant, 1),
				code.Make(code.OpSub),
				code.Make(code.OpPop),
			},
		},
		{
			input:            "1 * 2;",
			expectedConstant: []any{1, 2},
			expectInstructions: []code.Instructions{
				code.Make(code.OpConstant, 0),
				code.Make(code.OpConstant, 1),
				code.Make(code.OpMul),
				code.Make(code.OpPop),
			},
		},
		{
			input: "-1;",
			expectedConstant: []any{1},
			expectInstructions: []code.Instructions{
				code.Make(code.OpConstant, 0),
				code.Make(code.OpMinus),
				code.Make(code.OpPop),
			},
		},
		{
			input:            "1 / 2;",
			expectedConstant: []any{1, 2},
			expectInstructions: []code.Instructions{
				code.Make(code.OpConstant, 0),
				code.Make(code.OpConstant, 1),
				code.Make(code.OpDiv),
				code.Make(code.OpPop),
			},
		},
		{
			input:            "5 % 2;",
			expectedConstant: []any{5, 2},
			expectInstructions: []code.Instructions{
				code.Make(code.OpConstant, 0),
				code.Make(code.OpConstant, 1),
				code.Make(code.OpMod),
				code.Make(code.OpPop),
			},
		},
		{
			input:            "2 ** 3;",
			expectedConstant: []any{2, 3},
			expectInstructions: []code.Instructions{
				code.Make(code.OpConstant, 0),
				code.Make(code.OpConstant, 1),
				code.Make(code.OpPow),
				code.Make(code.OpPop),
			},
		},
		{
			input:            "5 // 2;",
			expectedConstant: []any{5, 2},
			expectInstructions: []code.Instructions{
				code.Make(code.OpConstant, 0),
				code.Make(code.OpConstant, 1),
				code.Make(code.OpFloorDiv),
				code.Make(code.OpPop),
			},
		},
		{
			input:            "1; 2;",
			expectedConstant: []any{1, 2},
			expectInstructions: []code.Instructions{
				code.Make(code.OpConstant, 0),
				code.Make(code.OpPop),
				code.Make(code.OpConstant, 1),
				code.Make(code.OpPop),
			},
		},
	}

	runCompilerTest(t, tests)
}

func TestBooleanExpression(t *testing.T) {
	tests := []compilerTestCase{
		{
			input:            "true;",
			expectedConstant: []any{},
			expectInstructions: []code.Instructions{
				code.Make(code.OpTrue),
				code.Make(code.OpPop),
			},
		},
		{
			input: "!true;",
			expectedConstant: []any{},
			expectInstructions: []code.Instructions{
				code.Make(code.OpTrue),
				code.Make(code.OpBang),
				code.Make(code.OpPop),
			},
		},
		{
			input:            "false;",
			expectedConstant: []any{},
			expectInstructions: []code.Instructions{
				code.Make(code.OpFalse),
				code.Make(code.OpPop),
			},
		},
		{
			input:            "1 > 2;",
			expectedConstant: []any{1, 2},
			expectInstructions: []code.Instructions{
				code.Make(code.OpConstant, 0),
				code.Make(code.OpConstant, 1),
				code.Make(code.OpGreaterThan),
				code.Make(code.OpPop),
			},
		},
		{
			input:            "1 < 2;",
			expectedConstant: []any{2, 1},
			expectInstructions: []code.Instructions{
				code.Make(code.OpConstant, 0),
				code.Make(code.OpConstant, 1),
				code.Make(code.OpGreaterThan),
				code.Make(code.OpPop),
			},
		},
		{
			input:            "1 >= 2;",
			expectedConstant: []any{1, 2},
			expectInstructions: []code.Instructions{
				code.Make(code.OpConstant, 0),
				code.Make(code.OpConstant, 1),
				code.Make(code.OpGreaterEq),
				code.Make(code.OpPop),
			},
		},
		{
			input:            "1 <= 2;",
			expectedConstant: []any{1, 2},
			expectInstructions: []code.Instructions{
				code.Make(code.OpConstant, 0),
				code.Make(code.OpConstant, 1),
				code.Make(code.OpLessEq),
				code.Make(code.OpPop),
			},
		},
		{
			input:            "1 >= 2;",
			expectedConstant: []any{1, 2},
			expectInstructions: []code.Instructions{
				code.Make(code.OpConstant, 0),
				code.Make(code.OpConstant, 1),
				code.Make(code.OpGreaterEq),
				code.Make(code.OpPop),
			},
		},
		{
			input:            "1 <= 2;",
			expectedConstant: []any{1, 2},
			expectInstructions: []code.Instructions{
				code.Make(code.OpConstant, 0),
				code.Make(code.OpConstant, 1),
				code.Make(code.OpLessEq),
				code.Make(code.OpPop),
			},
		},
		{
			input:            "1 == 2;",
			expectedConstant: []any{1, 2},
			expectInstructions: []code.Instructions{
				code.Make(code.OpConstant, 0),
				code.Make(code.OpConstant, 1),
				code.Make(code.OpEqual),
				code.Make(code.OpPop),
			},
		},
		{
			input:            "1 != 2;",
			expectedConstant: []any{1, 2},
			expectInstructions: []code.Instructions{
				code.Make(code.OpConstant, 0),
				code.Make(code.OpConstant, 1),
				code.Make(code.OpNotEqual),
				code.Make(code.OpPop),
			},
		},
		{
			input:            "true == false;",
			expectedConstant: []any{},
			expectInstructions: []code.Instructions{
				code.Make(code.OpTrue),
				code.Make(code.OpFalse),
				code.Make(code.OpEqual),
				code.Make(code.OpPop),
			},
		},
		{
			input:            "true != false;",
			expectedConstant: []any{},
			expectInstructions: []code.Instructions{
				code.Make(code.OpTrue),
				code.Make(code.OpFalse),
				code.Make(code.OpNotEqual),
				code.Make(code.OpPop),
			},
		},
	}

	runCompilerTest(t, tests)
}

func TestFloatArithmetic(t *testing.T) {
	tests := []compilerTestCase{
		{
			input:            "1.5 + 2.25;",
			expectedConstant: []any{1.5, 2.25},
			expectInstructions: []code.Instructions{
				code.Make(code.OpConstant, 0),
				code.Make(code.OpConstant, 1),
				code.Make(code.OpAdd),
				code.Make(code.OpPop),
			},
		},
		{
			input:            "3.0 * 2.0;",
			expectedConstant: []any{3.0, 2.0},
			expectInstructions: []code.Instructions{
				code.Make(code.OpConstant, 0),
				code.Make(code.OpConstant, 1),
				code.Make(code.OpMul),
				code.Make(code.OpPop),
			},
		},
		{
			input:            "5.0 % 2.0;",
			expectedConstant: []any{5.0, 2.0},
			expectInstructions: []code.Instructions{
				code.Make(code.OpConstant, 0),
				code.Make(code.OpConstant, 1),
				code.Make(code.OpMod),
				code.Make(code.OpPop),
			},
		},
		{
			input:            "2.0 ** 3.0;",
			expectedConstant: []any{2.0, 3.0},
			expectInstructions: []code.Instructions{
				code.Make(code.OpConstant, 0),
				code.Make(code.OpConstant, 1),
				code.Make(code.OpPow),
				code.Make(code.OpPop),
			},
		},
		{
			input:            "5.0 // 2.0;",
			expectedConstant: []any{5.0, 2.0},
			expectInstructions: []code.Instructions{
				code.Make(code.OpConstant, 0),
				code.Make(code.OpConstant, 1),
				code.Make(code.OpFloorDiv),
				code.Make(code.OpPop),
			},
		},
	}

	runCompilerTest(t, tests)
}

func TestAssignmentsAndUpdates(t *testing.T) {
	tests := []compilerTestCase{
		{
			input:            "let x = 1;",
			expectedConstant: []any{1},
			expectInstructions: []code.Instructions{
				code.Make(code.OpConstant, 0),
				code.Make(code.OpSetGlobal, 0),
			},
		},
		{
			input:            "x = 2;",
			expectedConstant: []any{2},
			expectInstructions: []code.Instructions{
				code.Make(code.OpConstant, 0),
				code.Make(code.OpSetGlobal, 0),
				code.Make(code.OpGetGlobal, 0),
				code.Make(code.OpPop),
			},
		},
		{
			input:            "x += 2;",
			expectedConstant: []any{2},
			expectInstructions: []code.Instructions{
				code.Make(code.OpGetGlobal, 0),
				code.Make(code.OpConstant, 0),
				code.Make(code.OpAdd),
				code.Make(code.OpSetGlobal, 0),
				code.Make(code.OpGetGlobal, 0),
				code.Make(code.OpPop),
			},
		},
		{
			input:            "++x;",
			expectedConstant: []any{1},
			expectInstructions: []code.Instructions{
				code.Make(code.OpGetGlobal, 0),
				code.Make(code.OpConstant, 0),
				code.Make(code.OpAdd),
				code.Make(code.OpSetGlobal, 0),
				code.Make(code.OpGetGlobal, 0),
				code.Make(code.OpPop),
			},
		},
		{
			input:            "x++;",
			expectedConstant: []any{1},
			expectInstructions: []code.Instructions{
				code.Make(code.OpGetGlobal, 0),
				code.Make(code.OpDup),
				code.Make(code.OpConstant, 0),
				code.Make(code.OpAdd),
				code.Make(code.OpSetGlobal, 0),
				code.Make(code.OpPop),
			},
		},
	}

	runCompilerTest(t, tests)
}

func runCompilerTest(t *testing.T, tests []compilerTestCase) {
	t.Helper()

	for _, tt := range tests {
		program := parse(tt.input)

		compiler := New()

		err := compiler.Compile(program)
		if err != nil {
			t.Fatalf("compiler error: %s", err)
		}

		bytecode := compiler.ByteCode()

		err = testInstructions(tt.expectInstructions, bytecode.Instructions)
		if err != nil {
			t.Fatalf("testInstruction failed: %s", err)
		}

		err = testConstants(t, tt.expectedConstant, bytecode.Constants)
		if err != nil {
			t.Fatalf("testConstant failed: %s", err)
		}
	}
}

func parse(input string) *ast.Program {
	l := lexer.New(input)
	p := parser.New(l)

	return p.ParsePrograme()
}

func testInstructions(
	expected []code.Instructions,
	actual code.Instructions,
) error {
	concatted := concatInstructions(expected)
	if len(actual) != len(concatted) {
		return fmt.Errorf("wrong instructions length.\nwant=%q\ngot =%q",
			concatted, actual)
	}
	for i, ins := range concatted {
		if actual[i] != ins {
			return fmt.Errorf("wrong instruction at %d.\nwant=%q\ngot =%q",
				i, concatted, actual)
		}
	}
	return nil
}

func concatInstructions(s []code.Instructions) code.Instructions {
	out := code.Instructions{}
	for _, ins := range s {
		out = append(out, ins...)
	}
	return out
}

func testConstants(
	t *testing.T,
	expected []any,
	actual []object.Object,
) error {
	if len(expected) != len(actual) {
		return fmt.Errorf("wrong number of constants. got=%d, want=%d",
			len(actual), len(expected))
	}
	for i, constant := range expected {
		switch constant := constant.(type) {
		case int:
			err := testIntegerObject(int64(constant), actual[i])
			if err != nil {
				return fmt.Errorf("constant %d - testIntegerObject failed: %s",
					i, err)
			}
		case float64:
			err := testFloatObject(constant, actual[i])
			if err != nil {
				return fmt.Errorf("constant %d - testFloatObject failed: %s",
					i, err)
			}
		}
	}
	return nil
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
