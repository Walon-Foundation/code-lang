package arrays

import (
	"github.com/walonCode/code-lang/internal/ast"
	"github.com/walonCode/code-lang/internal/object"
)

var arrayBuiltins = map[string]*object.Builtin{
	"first": {
		Fn: func(node *ast.CallExpression, args ...object.Object) object.Object {
			if len(args) != 1 {
				return object.NewError(node.Line(), node.Column(), "wrong number of arguments. got=%d, want=1", len(args))
			}

			if args[0].Type() != object.ARRAY_OBJ {
				return object.NewError(node.Line(), node.Column(), "argument to `first` must be ARRAY, got %s",
					args[0].Type())
			}

			arr := args[0].(*object.Array)
			if len(arr.Elements) > 0 {
				return arr.Elements[0]
			}
			return object.NULL
		},
	},
	"last": {
		Fn: func(node *ast.CallExpression, args ...object.Object) object.Object {
			if len(args) != 1 {
				return object.NewError(node.Line(), node.Column(), "wrong number of arguments. got=%d, want=1", len(args))
			}

			if args[0].Type() != object.ARRAY_OBJ {
				return object.NewError(node.Line(), node.Column(), "argument to `last` must be ARRAY, got %s",
					args[0].Type())
			}

			arr := args[0].(*object.Array)
			length := len(arr.Elements)
			if len(arr.Elements) > 0 {
				return arr.Elements[length-1]
			}
			return object.NULL
		},
	},
	"rest": {
		Fn: func(node *ast.CallExpression, args ...object.Object) object.Object {
			if len(args) != 1 {
				return object.NewError(node.Line(), node.Column(), "wrong number of arguments. got=%d, want=1", len(args))
			}

			if args[0].Type() != object.ARRAY_OBJ {
				return object.NewError(node.Line(), node.Column(), "argument to `rest` must be ARRAY, got %s",
					args[0].Type())
			}

			arr := args[0].(*object.Array)
			length := len(arr.Elements)
			if length > 0 {
				newElement := make([]object.Object, length-1)
				copy(newElement, arr.Elements[1:length])
				return &object.Array{Elements: newElement}
			}
			return object.NULL
		},
	},
	"push": {
		Fn: func(node *ast.CallExpression, args ...object.Object) object.Object {
			if len(args) != 2 {
				return object.NewError(node.Line(), node.Column(), "wrong number of arguments. got=%d, want=2", len(args))
			}

			if args[0].Type() != object.ARRAY_OBJ {
				return object.NewError(node.Line(), node.Column(), "argument to `rest` must be ARRAY, got %s",
					args[0].Type())
			}

			arr := args[0].(*object.Array)
			length := len(arr.Elements)

			newElement := make([]object.Object, length+1)
			copy(newElement, arr.Elements)
			newElement[length] = args[1]

			return &object.Array{Elements: newElement}
		},
	},
}

func Module() *object.Module {
	members := map[string]object.Object{}
	for name, fn := range arrayBuiltins {
		members[name] = fn
	}
	return &object.Module{Members: members}
}
