package general

import (
	"bufio"
	"fmt"
	"os"
	"os/exec"
	"runtime"
	"strconv"
	"strings"

	"github.com/walonCode/code-lang/internal/ast"
	"github.com/walonCode/code-lang/internal/object"
)

func unwrapObject(obj object.Object) any {
	switch o := obj.(type) {
	case *object.Integer:
		return o.Value
	case *object.String:
		return o.Value
	case *object.Boolean:
		return o.Value
	case *object.Float:
		return o.Value
	case *object.Char:
		return o.Value
	default:
		return o.Inspect()
	}
}

var fmtBuiltins = map[string]*object.Builtin{
	"print": {
		Fn: func(node *ast.CallExpression, args ...object.Object) object.Object {
			for i, value := range args {
				fmt.Print(value.Inspect())
				if i < len(args)-1 {
					fmt.Print(" ")
				}
			}
			fmt.Println()
			return nil
		},
	},
	"printf": {
		Fn: func(node *ast.CallExpression, args ...object.Object) object.Object {
			if len(args) < 2 {
				return nil
			}

			formatStr, isFormat := args[0].(*object.String)

			if isFormat && len(args) > 1 && strings.Contains(formatStr.Value, "%") {
				goArgs := make([]any, len(args)-1)
				for i, arg := range args[1:] {
					goArgs[i] = unwrapObject(arg)
				}
				fmt.Printf(formatStr.Value, goArgs...)
				fmt.Println()
			}
			return nil
		},
	},
	"len": {
		Fn: func(node *ast.CallExpression, args ...object.Object) object.Object {
			if len(args) != 1 {
				return object.NewError(node.Line(), node.Column(), "wrong number of arguments. got=%d, want=1", len(args))
			}

			switch arg := args[0].(type) {
			case *object.Array:
				return &object.Integer{Value: int64(len(arg.Elements))}
			case *object.String:
				return &object.Integer{Value: int64(len(arg.Value))}
			default:
				return object.NewError(node.Line(), node.Column(), "argument to `len` not supported, got %s", args[0].Type())
			}
		},
	},
	"typeof": {
		Fn: func(node *ast.CallExpression, args ...object.Object) object.Object {
			if len(args) != 1 {
				return object.NewError(node.Line(), node.Column(), "wrong number of arguments. got=%d, want=1", len(args))
			}
			return &object.String{Value: string(args[0].Type())}
		},
	},
	"int": {
		Fn: func(node *ast.CallExpression, args ...object.Object) object.Object {
			if len(args) != 1 {
				return object.NewError(node.Line(), node.Column(), "wrong number of arguments. got=%d, want=1", len(args))
			}

			val, ok := args[0].(*object.String)
			if !ok {
				return object.NewError(node.Line(), node.Column(), "input must be a string")
			}

			int, err := strconv.Atoi(val.Value)
			if err != nil {
				return object.NewError(node.Line(), node.Column(), "input is not a string")
			}

			return &object.Integer{Value: int64(int)}
		},
	},
	"float": {
		Fn: func(node *ast.CallExpression, args ...object.Object) object.Object {
			if len(args) != 1 {
				return object.NewError(node.Line(), node.Column(), "wrong number of arguments. got=%d, want=1", len(args))
			}

			switch obj := args[0].(type) {
			case *object.Integer:
				float := float64(obj.Value)
				return &object.Float{Value: float}
			case *object.String:
				float, err := strconv.ParseFloat(obj.Value, 64)
				if err != nil {
					return object.NewError(node.Line(), node.Column(), "input is not a string")
				}

				return &object.Float{Value: float}
			default:
				return object.NewError(node.Line(), node.Column(), "input must be a string or an int")
			}
		},
	},
	"input": {
		Fn: func(node *ast.CallExpression, args ...object.Object) object.Object {
			if len(args) != 1 {
				return object.NewError(node.Line(), node.Column(), "wrong number of arguments. got=%d, want=1", len(args))
			}

			val, ok := args[0].(*object.String)
			if !ok {
				return object.NewError(node.Line(), node.Column(), "input must be a string")
			}

			fmt.Printf("%s: ", val.Value)
			scanner := bufio.NewScanner(os.Stdin)
			scanner.Scan()
			text := scanner.Text()

			return &object.String{Value: text}
		},
	},
	"clear": {
		Fn: func(node *ast.CallExpression, args ...object.Object) object.Object {
			if len(args) == 1 {
				return object.NewError(node.Line(), node.Column(), "function doesn't take any input")
			}

			var cmd *exec.Cmd

			if runtime.GOOS == "windows" {
				cmd = exec.Command("cmd", "/c", "cls")
			} else {
				cmd = exec.Command("clear")
			}
			cmd.Stdout = os.Stdout
			cmd.Run()

			return nil
		},
	},
}

func Module() *object.Module {
	member := map[string]object.Object{}
	for name, fn := range fmtBuiltins {
		member[name] = fn
	}

	return &object.Module{Members: member}
}
