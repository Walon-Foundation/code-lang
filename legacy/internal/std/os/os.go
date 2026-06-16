package os

import (
	"os"
	"runtime"

	"github.com/walonCode/code-lang/internal/ast"
	"github.com/walonCode/code-lang/internal/object"
)

func Module() *object.Module {
	// Prepare args array
	argElements := make([]object.Object, len(os.Args))
	for i, arg := range os.Args {
		argElements[i] = &object.String{Value: arg}
	}

	return &object.Module{
		Members: map[string]object.Object{
			"args":     &object.Array{Elements: argElements},
			"platform": &object.String{Value: runtime.GOOS},
			"arch":     &object.String{Value: runtime.GOARCH},
			"get_env":  getEnvFunc(),
			"set_env":  setEnvFunc(),
			"get_wd":   getWdFunc(),
			"exit":     exitFunc(),
			"hostname": hostnameFunc(),
		},
	}
}

func getEnvFunc() object.Object {
	return &object.Builtin{
		Fn: func(node *ast.CallExpression, args ...object.Object) object.Object {
			if len(args) != 1 {
				return object.NewError(node.Line(), node.Column(), "os.get_env() takes 1 argument")
			}
			key, ok := args[0].(*object.String)
			if !ok {
				return object.NewError(node.Line(), node.Column(), "argument must be a string")
			}
			return &object.String{Value: os.Getenv(key.Value)}
		},
	}
}

func setEnvFunc() object.Object {
	return &object.Builtin{
		Fn: func(node *ast.CallExpression, args ...object.Object) object.Object {
			if len(args) != 2 {
				return object.NewError(node.Line(), node.Column(), "os.set_env() takes 2 arguments: key and value")
			}
			key, ok1 := args[0].(*object.String)
			value, ok2 := args[1].(*object.String)
			if !ok1 || !ok2 {
				return object.NewError(node.Line(), node.Column(), "both arguments must be strings")
			}
			err := os.Setenv(key.Value, value.Value)
			if err != nil {
				return object.NewError(node.Line(), node.Column(), "failed to set env: %s", err.Error())
			}
			return nil
		},
	}
}

func getWdFunc() object.Object {
	return &object.Builtin{
		Fn: func(node *ast.CallExpression, args ...object.Object) object.Object {
			wd, err := os.Getwd()
			if err != nil {
				return object.NewError(node.Line(), node.Column(), "failed to get working directory: %s", err.Error())
			}
			return &object.String{Value: wd}
		},
	}
}

func exitFunc() object.Object {
	return &object.Builtin{
		Fn: func(node *ast.CallExpression, args ...object.Object) object.Object {
			code := 0
			if len(args) == 1 {
				if c, ok := args[0].(*object.Integer); ok {
					code = int(c.Value)
				}
			}
			os.Exit(code)
			return nil
		},
	}
}

func hostnameFunc() object.Object {
	return &object.Builtin{
		Fn: func(node *ast.CallExpression, args ...object.Object) object.Object {
			name, err := os.Hostname()
			if err != nil {
				return object.NewError(node.Line(), node.Column(), "failed to get hostname: %s", err.Error())
			}
			return &object.String{Value: name}
		},
	}
}
