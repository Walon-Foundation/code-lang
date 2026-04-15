package compiler

import (
	"fmt"

	"github.com/walonCode/code-lang/internal/ast"
	"github.com/walonCode/code-lang/internal/code"
	"github.com/walonCode/code-lang/internal/object"
)

type EmittedInstruction struct {
	Opcode   code.Opcode
	Position int
}

type Compiler struct {
	instruction         code.Instructions
	constants           []object.Object
	globals             map[string]int
	globalCount         int
	lastInstruction     EmittedInstruction
	previousInstruction EmittedInstruction
}

func New() *Compiler {
	return &Compiler{
		instruction:         code.Instructions{},
		constants:           []object.Object{},
		globals:             map[string]int{},
		globalCount:         0,
		lastInstruction:     EmittedInstruction{},
		previousInstruction: EmittedInstruction{},
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
	case *ast.BlockStatement:
		for _, s := range node.Statements {
			err := c.Compile(s)
			if err != nil {
				return err
			}
		}
	case *ast.Boolean:
		if node.Value {
			c.emit(code.OpTrue)
		} else {
			c.emit(code.OpFalse)
		}

	case *ast.PrefixExpression:
		err := c.Compile(node.Right)
		if err != nil {
			return err
		}

		switch node.Operator {
		case "!":
			c.emit(code.OpBang)
		case "-":
			c.emit(code.OpMinus)
		default:
			return fmt.Errorf("unknown operator %s", node.Operator)
		}

	case *ast.InfixExpression:
		if node.Operator == "<" {
			err := c.Compile(node.Right)
			if err != nil {
				return err
			}
			err = c.Compile(node.Left)
			if err != nil {
				return err
			}
			c.emit(code.OpGreaterThan)
			return nil
		}
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
		case ">":
			c.emit(code.OpGreaterThan)
		case ">=":
			c.emit(code.OpGreaterEq)
		case "<=":
			c.emit(code.OpLessEq)
		case "==":
			c.emit(code.OpEqual)
		case "!=":
			c.emit(code.OpNotEqual)
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
	case *ast.IfExpression:
		err := c.Compile(node.Condition)
		if err != nil {
			return err
		}

		jumpNotTruthyPos := c.emit(code.OpJumpNotTruthy, 9999)

		err = c.Compile(node.Consequence)
		if err != nil {
			return err
		}

		if c.lastInstructionIsPop() {
			c.removeLastPop()
		}

		afterConseqencePos := len(c.instruction)
		c.changeOperand(jumpNotTruthyPos, afterConseqencePos)

		// after executing this branch, skip the rest
		endJumps := []int{}
		endJumps = append(endJumps, c.emit(code.OpJump, 9999))

		// false of main if goes to here
		nextBranchPos := len(c.instruction)
		c.changeOperand(jumpNotTruthyPos, nextBranchPos)

		// compile all elseif branches
		for _, elseif := range node.IfElse {
			err := c.Compile(elseif.Condition)
			if err != nil {
				return err
			}

			elseifJumpNotTruthyPos := c.emit(code.OpJumpNotTruthy, 9999)

			err = c.Compile(elseif.Consequence)
			if err != nil {
				return err
			}

			if c.lastInstructionIsPop() {
				c.removeLastPop()
			}

			endJumps = append(endJumps, c.emit(code.OpJump, 9999))

			nextBranchPos = len(c.instruction)
			c.changeOperand(elseifJumpNotTruthyPos, nextBranchPos)
		}

		if node.Alternative != nil {
			err := c.Compile(node.Alternative)
			if err != nil {
				return err
			}

			if c.lastInstructionIsPop() {
				c.removeLastPop()
			}
		}else {
			c.emit(code.OpNull)
		}

		// patch all jumps to the end
		afterAllBranchesPos := len(c.instruction)
		for _, pos := range endJumps {
			c.changeOperand(pos, afterAllBranchesPos)
		}
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

	c.setLastInstruction(op, pos)

	return pos
}

func (c *Compiler) setLastInstruction(op code.Opcode, pos int) {
	previous := c.lastInstruction
	last := EmittedInstruction{Opcode: op, Position: pos}

	c.previousInstruction = previous
	c.lastInstruction = last
}

func (c *Compiler) addInstruction(ins []byte) int {
	posNewInstruction := len(c.instruction)
	c.instruction = append(c.instruction, ins...)

	return posNewInstruction
}

func (c *Compiler) lastInstructionIsPop() bool {
	return c.lastInstruction.Opcode == code.OpPop
}

func (c *Compiler) removeLastPop() {
	c.instruction = c.instruction[:c.lastInstruction.Position]
	c.lastInstruction = c.previousInstruction
}

func (c *Compiler) replaceInstruction(pos int, newInstruction []byte) {
	for i := range newInstruction {
		c.instruction[pos+i] = newInstruction[i]
	}
}

func (c *Compiler) changeOperand(opPos int, operand int) {
	op := code.Opcode(c.instruction[opPos])
	newInstruction := code.Make(op, operand)

	c.replaceInstruction(opPos, newInstruction)
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
