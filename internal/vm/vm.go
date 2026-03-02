package vm

import (
	"fmt"
	"math"

	"github.com/walonCode/code-lang/internal/code"
	"github.com/walonCode/code-lang/internal/compiler"
	"github.com/walonCode/code-lang/internal/object"
)

const STACKSIZE = 2048
const GLOBALS_SIZE = 65536

type VM struct {
	constants    []object.Object
	instructions code.Instructions
	globals      []object.Object

	stack []object.Object
	sp    int // alway point to the next value. Top of stack is stack[sp - 1]
}

func New(bytecode *compiler.Bytecode) *VM {
	return &VM{
		instructions: bytecode.Instructions,
		constants:    bytecode.Constants,
		globals:      make([]object.Object, GLOBALS_SIZE),
		stack:        make([]object.Object, STACKSIZE),
		sp:           0,
	}
}

func NewWithGlobals(bytecode *compiler.Bytecode, globals []object.Object) *VM {
	if globals == nil {
		globals = make([]object.Object, GLOBALS_SIZE)
	}
	return &VM{
		instructions: bytecode.Instructions,
		constants:    bytecode.Constants,
		globals:      globals,
		stack:        make([]object.Object, STACKSIZE),
		sp:           0,
	}
}

func (vm *VM) StackTop() object.Object {
	if vm.sp == 0 {
		return nil
	}

	return vm.stack[vm.sp-1]
}

func (vm *VM) push(o object.Object) error {
	if vm.sp >= STACKSIZE {
		return fmt.Errorf("stack overflow")
	}

	vm.stack[vm.sp] = o
	vm.sp++

	return nil
}

func (vm *VM) pop() object.Object {
	o := vm.stack[vm.sp-1]
	vm.sp--

	return o
}

func (vm *VM) LastPoppedStackElm() object.Object {
	return vm.stack[vm.sp]
}

func (vm *VM) executeBinaryOperation(op code.Opcode) error {
	right := vm.pop()
	left := vm.pop()

	leftType := left.Type()
	rightType := right.Type()

	if leftType == object.INTEGER_OBJ && rightType == object.INTEGER_OBJ {
		return vm.executeBinaryIntegerOperation(op, left, right)
	}

	if leftType == object.FLOAT_OBJ || rightType == object.FLOAT_OBJ {
		leftVal, ok := toFloat64(left)
		if !ok {
			return fmt.Errorf("unsupported type for binary operation: %s %s", leftType, rightType)
		}
		rightVal, ok := toFloat64(right)
		if !ok {
			return fmt.Errorf("unsupported type for binary operation: %s %s", leftType, rightType)
		}
		return vm.executeBinaryFloatOperation(op, leftVal, rightVal)
	}

	return fmt.Errorf("unsupported type for binary operation: %s %s", leftType, rightType)
}

func (vm *VM) executeBinaryIntegerOperation(op code.Opcode, left, right object.Object) error {
	leftValue := left.(*object.Integer).Value
	rightValue := right.(*object.Integer).Value

	var result int64

	switch op {
	case code.OpAdd:
		result = leftValue + rightValue
	case code.OpSub:
		result = leftValue - rightValue
	case code.OpMul:
		result = leftValue * rightValue
	case code.OpDiv:
		if rightValue == 0 {
			return fmt.Errorf("division by zero Error")
		}
		result = leftValue / rightValue
	case code.OpMod:
		if rightValue == 0 {
			return fmt.Errorf("division by zero Error")
		}
		result = int64(math.Mod(float64(leftValue), float64(rightValue)))
	case code.OpPow:
		result = int64(math.Pow(float64(leftValue), float64(rightValue)))
	case code.OpFloorDiv:
		if rightValue == 0 {
			return fmt.Errorf("division by zero Error")
		}
		result = int64(math.Floor(float64(leftValue) / float64(rightValue)))
	default:
		return fmt.Errorf("unknown integer operator: %d", op)
	}

	return vm.push(&object.Integer{Value: result})
}

func (vm *VM) executeBinaryFloatOperation(op code.Opcode, left, right float64) error {
	var result float64

	switch op {
	case code.OpAdd:
		result = left + right
	case code.OpSub:
		result = left - right
	case code.OpMul:
		result = left * right
	case code.OpDiv:
		if right == 0 {
			return fmt.Errorf("division by zero Error")
		}
		result = left / right
	case code.OpMod:
		if right == 0 {
			return fmt.Errorf("division by zero Error")
		}
		result = math.Mod(left, right)
	case code.OpPow:
		result = math.Pow(left, right)
	case code.OpFloorDiv:
		if right == 0 {
			return fmt.Errorf("division by zero Error")
		}
		result = math.Floor(left / right)
	default:
		return fmt.Errorf("unknown float operator: %d", op)
	}

	return vm.push(&object.Float{Value: result})
}

func toFloat64(obj object.Object) (float64, bool) {
	switch v := obj.(type) {
	case *object.Integer:
		return float64(v.Value), true
	case *object.Float:
		return v.Value, true
	default:
		return 0, false
	}
}

func (vm *VM) Run() error {
	for ip := 0; ip < len(vm.instructions); ip++ {
		op := code.Opcode(vm.instructions[ip])

		switch op {
		case code.OpConstant:
			constIndex := code.ReadUint16(vm.instructions[ip+1:])
			ip += 2

			err := vm.push(vm.constants[constIndex])
			if err != nil {
				return err
			}
		case code.OpAdd, code.OpDiv, code.OpSub, code.OpMul, code.OpMod, code.OpPow, code.OpFloorDiv:
			err := vm.executeBinaryOperation(op)
			if err != nil {
				return err
			}
		case code.OpSetGlobal:
			globalIndex := code.ReadUint16(vm.instructions[ip+1:])
			ip += 2
			vm.globals[globalIndex] = vm.pop()
		case code.OpGetGlobal:
			globalIndex := code.ReadUint16(vm.instructions[ip+1:])
			ip += 2
			val := vm.globals[globalIndex]
			if val == nil {
				return fmt.Errorf("undefined variable")
			}
			err := vm.push(val)
			if err != nil {
				return err
			}
		case code.OpDup:
			top := vm.StackTop()
			if top == nil {
				return fmt.Errorf("stack empty")
			}
			err := vm.push(top)
			if err != nil {
				return err
			}
		case code.OpPop:
			vm.pop()
		}
	}

	return nil
}
