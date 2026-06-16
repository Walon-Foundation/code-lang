package strings

import (
	"strings"

	"github.com/walonCode/code-lang/internal/ast"
	"github.com/walonCode/code-lang/internal/object"
)

func Module() *object.Module {
	return &object.Module{
		Members: map[string]object.Object{
			"to_upper":    toUpperFunc(),
			"to_lower":    toLowerFunc(),
			"split":       splitFunc(),
			"join":        joinFunc(),
			"contains":    containsFunc(),
			"replace":     replaceFunc(),
			"trim":        trimFunc(),
			"trim_left":   trimLeftFunc(),
			"trim_right":  trimRightFunc(),
			"starts_with": startsWithFunc(),
			"ends_with":   endsWithFunc(),
			"index":       indexFunc(),
			"count":       countFunc(),
			"repeat":      repeatFunc(),
			"reverse":     reverseFunc(),
		},
	}
}

func toUpperFunc() object.Object {
	return &object.Builtin{
		Fn: func(node *ast.CallExpression, args ...object.Object) object.Object {
			if len(args) != 1 {
				return object.NewError(node.Line(), node.Column(), "strings.to_upper() takes 1 argument")
			}
			s, ok := args[0].(*object.String)
			if !ok {
				return object.NewError(node.Line(), node.Column(), "argument must be a string")
			}
			return &object.String{Value: strings.ToUpper(s.Value)}
		},
	}
}

func toLowerFunc() object.Object {
	return &object.Builtin{
		Fn: func(node *ast.CallExpression, args ...object.Object) object.Object {
			if len(args) != 1 {
				return object.NewError(node.Line(), node.Column(), "strings.to_lower() takes 1 argument")
			}
			s, ok := args[0].(*object.String)
			if !ok {
				return object.NewError(node.Line(), node.Column(), "argument must be a string")
			}
			return &object.String{Value: strings.ToLower(s.Value)}
		},
	}
}

func splitFunc() object.Object {
	return &object.Builtin{
		Fn: func(node *ast.CallExpression, args ...object.Object) object.Object {
			if len(args) != 2 {
				return object.NewError(node.Line(), node.Column(), "strings.split() takes 2 arguments: string and separator")
			}
			s, ok1 := args[0].(*object.String)
			sep, ok2 := args[1].(*object.String)
			if !ok1 || !ok2 {
				return object.NewError(node.Line(), node.Column(), "both arguments must be strings")
			}
			parts := strings.Split(s.Value, sep.Value)
			elements := make([]object.Object, len(parts))
			for i, part := range parts {
				elements[i] = &object.String{Value: part}
			}
			return &object.Array{Elements: elements}
		},
	}
}

func joinFunc() object.Object {
	return &object.Builtin{
		Fn: func(node *ast.CallExpression, args ...object.Object) object.Object {
			if len(args) != 2 {
				return object.NewError(node.Line(), node.Column(), "strings.join() takes 2 arguments: array and separator")
			}
			arr, ok1 := args[0].(*object.Array)
			sep, ok2 := args[1].(*object.String)
			if !ok1 || !ok2 {
				return object.NewError(node.Line(), node.Column(), "first argument must be an array, second must be a string")
			}
			parts := make([]string, len(arr.Elements))
			for i, el := range arr.Elements {
				parts[i] = el.Inspect()
			}
			return &object.String{Value: strings.Join(parts, sep.Value)}
		},
	}
}

func containsFunc() object.Object {
	return &object.Builtin{
		Fn: func(node *ast.CallExpression, args ...object.Object) object.Object {
			if len(args) != 2 {
				return object.NewError(node.Line(), node.Column(), "strings.contains() takes 2 arguments")
			}
			s, ok1 := args[0].(*object.String)
			substr, ok2 := args[1].(*object.String)
			if !ok1 || !ok2 {
				return object.NewError(node.Line(), node.Column(), "both arguments must be strings")
			}
			return &object.Boolean{Value: strings.Contains(s.Value, substr.Value)}
		},
	}
}

func replaceFunc() object.Object {
	return &object.Builtin{
		Fn: func(node *ast.CallExpression, args ...object.Object) object.Object {
			if len(args) != 3 {
				return object.NewError(node.Line(), node.Column(), "strings.replace() takes 3 arguments: string, old, new")
			}
			s, ok1 := args[0].(*object.String)
			old, ok2 := args[1].(*object.String)
			new, ok3 := args[2].(*object.String)
			if !ok1 || !ok2 || !ok3 {
				return object.NewError(node.Line(), node.Column(), "all arguments must be strings")
			}
			return &object.String{Value: strings.ReplaceAll(s.Value, old.Value, new.Value)}
		},
	}
}

func trimFunc() object.Object {
	return &object.Builtin{
		Fn: func(node *ast.CallExpression, args ...object.Object) object.Object {
			if len(args) != 1 {
				return object.NewError(node.Line(), node.Column(), "strings.trim() takes 1 argument")
			}
			s, ok := args[0].(*object.String)
			if !ok {
				return object.NewError(node.Line(), node.Column(), "argument must be a string")
			}
			return &object.String{Value: strings.TrimSpace(s.Value)}
		},
	}
}

func trimLeftFunc() object.Object {
	return &object.Builtin{
		Fn: func(node *ast.CallExpression, args ...object.Object) object.Object {
			if len(args) != 1 {
				return object.NewError(node.Line(), node.Column(), "strings.trim_left() takes 1 argument")
			}
			s, ok := args[0].(*object.String)
			if !ok {
				return object.NewError(node.Line(), node.Column(), "argument must be a string")
			}
			return &object.String{Value: strings.TrimLeft(s.Value, " \t\n\r")}
		},
	}
}

func trimRightFunc() object.Object {
	return &object.Builtin{
		Fn: func(node *ast.CallExpression, args ...object.Object) object.Object {
			if len(args) != 1 {
				return object.NewError(node.Line(), node.Column(), "strings.trim_right() takes 1 argument")
			}
			s, ok := args[0].(*object.String)
			if !ok {
				return object.NewError(node.Line(), node.Column(), "argument must be a string")
			}
			return &object.String{Value: strings.TrimRight(s.Value, " \t\n\r")}
		},
	}
}

func startsWithFunc() object.Object {
	return &object.Builtin{
		Fn: func(node *ast.CallExpression, args ...object.Object) object.Object {
			if len(args) != 2 {
				return object.NewError(node.Line(), node.Column(), "strings.starts_with() takes 2 arguments")
			}
			s, ok1 := args[0].(*object.String)
			prefix, ok2 := args[1].(*object.String)
			if !ok1 || !ok2 {
				return object.NewError(node.Line(), node.Column(), "both arguments must be strings")
			}
			return &object.Boolean{Value: strings.HasPrefix(s.Value, prefix.Value)}
		},
	}
}

func endsWithFunc() object.Object {
	return &object.Builtin{
		Fn: func(node *ast.CallExpression, args ...object.Object) object.Object {
			if len(args) != 2 {
				return object.NewError(node.Line(), node.Column(), "strings.ends_with() takes 2 arguments")
			}
			s, ok1 := args[0].(*object.String)
			suffix, ok2 := args[1].(*object.String)
			if !ok1 || !ok2 {
				return object.NewError(node.Line(), node.Column(), "both arguments must be strings")
			}
			return &object.Boolean{Value: strings.HasSuffix(s.Value, suffix.Value)}
		},
	}
}

func indexFunc() object.Object {
	return &object.Builtin{
		Fn: func(node *ast.CallExpression, args ...object.Object) object.Object {
			if len(args) != 2 {
				return object.NewError(node.Line(), node.Column(), "strings.index() takes 2 arguments")
			}
			s, ok1 := args[0].(*object.String)
			substr, ok2 := args[1].(*object.String)
			if !ok1 || !ok2 {
				return object.NewError(node.Line(), node.Column(), "both arguments must be strings")
			}
			return &object.Integer{Value: int64(strings.Index(s.Value, substr.Value))}
		},
	}
}

func countFunc() object.Object {
	return &object.Builtin{
		Fn: func(node *ast.CallExpression, args ...object.Object) object.Object {
			if len(args) != 2 {
				return object.NewError(node.Line(), node.Column(), "strings.count() takes 2 arguments")
			}
			s, ok1 := args[0].(*object.String)
			substr, ok2 := args[1].(*object.String)
			if !ok1 || !ok2 {
				return object.NewError(node.Line(), node.Column(), "both arguments must be strings")
			}
			return &object.Integer{Value: int64(strings.Count(s.Value, substr.Value))}
		},
	}
}

func repeatFunc() object.Object {
	return &object.Builtin{
		Fn: func(node *ast.CallExpression, args ...object.Object) object.Object {
			if len(args) != 2 {
				return object.NewError(node.Line(), node.Column(), "strings.repeat() takes 2 arguments: string and count")
			}
			s, ok1 := args[0].(*object.String)
			count, ok2 := args[1].(*object.Integer)
			if !ok1 || !ok2 {
				return object.NewError(node.Line(), node.Column(), "first argument must be a string, second must be an integer")
			}
			return &object.String{Value: strings.Repeat(s.Value, int(count.Value))}
		},
	}
}

func reverseFunc() object.Object {
	return &object.Builtin{
		Fn: func(node *ast.CallExpression, args ...object.Object) object.Object {
			if len(args) != 1 {
				return object.NewError(node.Line(), node.Column(), "strings.reverse() takes 1 argument")
			}
			s, ok := args[0].(*object.String)
			if !ok {
				return object.NewError(node.Line(), node.Column(), "argument must be a string")
			}
			runes := []rune(s.Value)
			for i, j := 0, len(runes)-1; i < j; i, j = i+1, j-1 {
				runes[i], runes[j] = runes[j], runes[i]
			}
			return &object.String{Value: string(runes)}
		},
	}
}
