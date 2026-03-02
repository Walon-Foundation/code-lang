package compiler

import (
	"fmt"

	"github.com/walonCode/code-lang/internal/ast"
	"github.com/walonCode/code-lang/internal/code"
	"github.com/walonCode/code-lang/internal/object"
)

type Compiler struct {
	instruction code.Instructions
	constants   []object.Object
	globals     map[string]int
	globalCount int
}

func New() *Compiler {
	return &Compiler{
		instruction: code.Instructions{},
		constants:   []object.Object{},
		globals:     map[string]int{},
		globalCount: 0,
	}
}

func (c *Compiler) Reset() {
	c.instruction = code.Instructions{}
	c.constants = []object.Object{}
}

func (c *Compiler) Compile(node ast.Node) error {
	switch node := node.(type) {
	case *ast.Program:
		for _, s := range node.Statements {
			err := c.Compile(s)
			if err != nil {
				return err
			}
		}
	case *ast.LetStatement:
		err := c.Compile(node.Value)
		if err != nil {
			return err
		}
		index := c.defineGlobal(node.Name.Value)
		c.emit(code.OpSetGlobal, index)
	case *ast.Identifier:
		index, ok := c.resolveGlobal(node.Value)
		if !ok {
			return fmt.Errorf("undefined variable %s", node.Value)
		}
		c.emit(code.OpGetGlobal, index)
	case *ast.ExpressionStatement:
		err := c.Compile(node.Expression)
		if err != nil {
			return err
		}
		c.emit(code.OpPop)

	case *ast.InfixExpression:
		if isAssignmentOperator(node.Operator) {
			return c.compileAssignment(node)
		}
		err := c.Compile(node.Left)
		if err != nil {
			return err
		}

		err = c.Compile(node.Right)
		if err != nil {
			return err
		}

		switch node.Operator {
		case "+":
			c.emit(code.OpAdd)
		case "-":
			c.emit(code.OpSub)
		case "*":
			c.emit(code.OpMul)
		case "/":
			c.emit(code.OpDiv)
		case "%":
			c.emit(code.OpMod)
		case "**":
			c.emit(code.OpPow)
		case "//":
			c.emit(code.OpFloorDiv)
		default:
			return fmt.Errorf("unknown operator %s", node.Operator)
		}

	case *ast.UpdateExpression:
		return c.compileUpdateExpression(node)

	case *ast.IntegerLiteral:
		integer := &object.Integer{Value: node.Value}
		c.emit(code.OpConstant, c.addConstant(integer))
	case *ast.FloatLiteral:
		floatValue := &object.Float{Value: node.Value}
		c.emit(code.OpConstant, c.addConstant(floatValue))
	}

	return nil
}

func isAssignmentOperator(op string) bool {
	switch op {
	case "=", "+=", "-=", "*=", "/=", "%=", "**=", "//=":
		return true
	default:
		return false
	}
}

func (c *Compiler) compileAssignment(node *ast.InfixExpression) error {
	ident, ok := node.Left.(*ast.Identifier)
	if !ok {
		return fmt.Errorf("invalid left-hand side in assignment")
	}

	index := c.defineGlobal(ident.Value)

	if node.Operator == "=" {
		if err := c.Compile(node.Right); err != nil {
			return err
		}
		c.emit(code.OpSetGlobal, index)
		c.emit(code.OpGetGlobal, index)
		return nil
	}

	c.emit(code.OpGetGlobal, index)
	if err := c.Compile(node.Right); err != nil {
		return err
	}

	switch node.Operator {
	case "+=":
		c.emit(code.OpAdd)
	case "-=":
		c.emit(code.OpSub)
	case "*=":
		c.emit(code.OpMul)
	case "/=":
		c.emit(code.OpDiv)
	case "%=":
		c.emit(code.OpMod)
	case "**=":
		c.emit(code.OpPow)
	case "//=":
		c.emit(code.OpFloorDiv)
	default:
		return fmt.Errorf("unknown operator %s", node.Operator)
	}

	c.emit(code.OpSetGlobal, index)
	c.emit(code.OpGetGlobal, index)
	return nil
}

func (c *Compiler) compileUpdateExpression(node *ast.UpdateExpression) error {
	ident, ok := node.Target.(*ast.Identifier)
	if !ok {
		return fmt.Errorf("invalid left-hand side in update")
	}
	index := c.defineGlobal(ident.Value)

	if node.Prefix {
		c.emit(code.OpGetGlobal, index)
		c.emit(code.OpConstant, c.addConstant(&object.Integer{Value: 1}))
		if node.Operator == "++" {
			c.emit(code.OpAdd)
		} else {
			c.emit(code.OpSub)
		}
		c.emit(code.OpSetGlobal, index)
		c.emit(code.OpGetGlobal, index)
		return nil
	}

	c.emit(code.OpGetGlobal, index)
	c.emit(code.OpDup)
	c.emit(code.OpConstant, c.addConstant(&object.Integer{Value: 1}))
	if node.Operator == "++" {
		c.emit(code.OpAdd)
	} else {
		c.emit(code.OpSub)
	}
	c.emit(code.OpSetGlobal, index)
	return nil
}

func (c *Compiler) defineGlobal(name string) int {
	if idx, ok := c.globals[name]; ok {
		return idx
	}
	idx := c.globalCount
	c.globals[name] = idx
	c.globalCount++
	return idx
}

func (c *Compiler) resolveGlobal(name string) (int, bool) {
	idx, ok := c.globals[name]
	return idx, ok
}

func (c *Compiler) addConstant(obj object.Object) int {
	c.constants = append(c.constants, obj)
	return len(c.constants) - 1
}

func (c *Compiler) emit(op code.Opcode, operands ...int) int {
	ins := code.Make(op, operands...)
	pos := c.addInstruction(ins)

	return pos
}

func (c *Compiler) addInstruction(ins []byte) int {
	posNewInstruction := len(c.instruction)
	c.instruction = append(c.instruction, ins...)

	return posNewInstruction
}

func (c *Compiler) ByteCode() *Bytecode {
	return &Bytecode{
		Instructions: c.instruction,
		Constants:    c.constants,
	}
}

type Bytecode struct {
	Instructions code.Instructions
	Constants    []object.Object
}
