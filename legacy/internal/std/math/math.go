package math

import (
	"math"

	"github.com/walonCode/code-lang/internal/ast"
	"github.com/walonCode/code-lang/internal/object"
)

func Module() *object.Module {
	return &object.Module{
		Members: map[string]object.Object{
			"PI":    &object.Float{Value: math.Pi},
			"E":     &object.Float{Value: math.E},
			"sqrt":  sqrtFunc(),
			"floor": floorFunc(),
			"pow":   powFunc(),
			"abs":   absFunc(),
			"sin":   sinFunc(),
			"cos":   cosFunc(),
			"tan":   tanFunc(),
			"round": roundFunc(),
			"ceil":  ceilFunc(),
			"log":   logFunc(),
			"exp":   expFunc(),
			"log10": log10Func(),
			"trunc": truncFunc(),
			"min":   minFunc(),
			"max":   maxFunc(),
		},
	}
}

func extractFloat(obj object.Object) (float64, bool) {
	switch o := obj.(type) {
	case *object.Integer:
		return float64(o.Value), true
	case *object.Float:
		return o.Value, true
	default:
		return 0, false
	}
}

func sqrtFunc() object.Object {
	return &object.Builtin{
		Fn: func(node *ast.CallExpression, args ...object.Object) object.Object {
			if len(args) != 1 {
				return object.NewError(node.Line(), node.Column(), "math.sqrt() takes one argument")
			}
			val, ok := extractFloat(args[0])
			if !ok {
				return object.NewError(node.Line(), node.Column(), "argument to math.sqrt must be a number")
			}
			return &object.Float{Value: math.Sqrt(val)}
		},
	}
}

func floorFunc() object.Object {
	return &object.Builtin{
		Fn: func(node *ast.CallExpression, args ...object.Object) object.Object {
			if len(args) != 1 {
				return object.NewError(node.Line(), node.Column(), "math.floor() takes one argument")
			}
			val, ok := extractFloat(args[0])
			if !ok {
				return object.NewError(node.Line(), node.Column(), "argument to math.floor must be a number")
			}
			return &object.Float{Value: math.Floor(val)}
		},
	}
}

func powFunc() object.Object {
	return &object.Builtin{
		Fn: func(node *ast.CallExpression, args ...object.Object) object.Object {
			if len(args) != 2 {
				return object.NewError(node.Line(), node.Column(), "math.pow() takes two arguments")
			}
			base, ok1 := extractFloat(args[0])
			exp, ok2 := extractFloat(args[1])
			if !ok1 || !ok2 {
				return object.NewError(node.Line(), node.Column(), "arguments to math.pow must be numbers")
			}
			return &object.Float{Value: math.Pow(base, exp)}
		},
	}
}

func absFunc() object.Object {
	return &object.Builtin{
		Fn: func(node *ast.CallExpression, args ...object.Object) object.Object {
			if len(args) != 1 {
				return object.NewError(node.Line(), node.Column(), "math.abs() takes one argument")
			}
			val, ok := extractFloat(args[0])
			if !ok {
				return object.NewError(node.Line(), node.Column(), "argument to math.abs must be a number")
			}
			return &object.Float{Value: math.Abs(val)}
		},
	}
}

func sinFunc() object.Object {
	return &object.Builtin{
		Fn: func(node *ast.CallExpression, args ...object.Object) object.Object {
			if len(args) != 1 {
				return object.NewError(node.Line(), node.Column(), "math.sin() takes one argument")
			}
			val, ok := extractFloat(args[0])
			if !ok {
				return object.NewError(node.Line(), node.Column(), "argument to math.sin must be a number")
			}
			return &object.Float{Value: math.Sin(val)}
		},
	}
}

func cosFunc() object.Object {
	return &object.Builtin{
		Fn: func(node *ast.CallExpression, args ...object.Object) object.Object {
			if len(args) != 1 {
				return object.NewError(node.Line(), node.Column(), "math.cos() takes one argument")
			}
			val, ok := extractFloat(args[0])
			if !ok {
				return object.NewError(node.Line(), node.Column(), "argument to math.cos must be a number")
			}
			return &object.Float{Value: math.Cos(val)}
		},
	}
}

func tanFunc() object.Object {
	return &object.Builtin{
		Fn: func(node *ast.CallExpression, args ...object.Object) object.Object {
			if len(args) != 1 {
				return object.NewError(node.Line(), node.Column(), "math.tan() takes one argument")
			}
			val, ok := extractFloat(args[0])
			if !ok {
				return object.NewError(node.Line(), node.Column(), "argument to math.tan must be a number")
			}
			return &object.Float{Value: math.Tan(val)}
		},
	}
}

func roundFunc() object.Object {
	return &object.Builtin{
		Fn: func(node *ast.CallExpression, args ...object.Object) object.Object {
			if len(args) != 1 {
				return object.NewError(node.Line(), node.Column(), "math.round() takes one argument")
			}
			val, ok := extractFloat(args[0])
			if !ok {
				return object.NewError(node.Line(), node.Column(), "argument to math.round must be a number")
			}
			return &object.Float{Value: math.Round(val)}
		},
	}
}

func ceilFunc() object.Object {
	return &object.Builtin{
		Fn: func(node *ast.CallExpression, args ...object.Object) object.Object {
			if len(args) != 1 {
				return object.NewError(node.Line(), node.Column(), "math.ceil() takes one argument")
			}
			val, ok := extractFloat(args[0])
			if !ok {
				return object.NewError(node.Line(), node.Column(), "argument to math.ceil must be a number")
			}
			return &object.Float{Value: math.Ceil(val)}
		},
	}
}

func logFunc() object.Object {
	return &object.Builtin{
		Fn: func(node *ast.CallExpression, args ...object.Object) object.Object {
			if len(args) != 1 {
				return object.NewError(node.Line(), node.Column(), "math.log() takes one argument")
			}
			val, ok := extractFloat(args[0])
			if !ok {
				return object.NewError(node.Line(), node.Column(), "argument to math.log must be a number")
			}
			return &object.Float{Value: math.Log(val)}
		},
	}
}

func log10Func() object.Object {
	return &object.Builtin{
		Fn: func(node *ast.CallExpression, args ...object.Object) object.Object {
			if len(args) != 1 {
				return object.NewError(node.Line(), node.Column(), "math.log10() takes one argument")
			}
			val, ok := extractFloat(args[0])
			if !ok {
				return object.NewError(node.Line(), node.Column(), "argument to math.log10 must be a number")
			}
			return &object.Float{Value: math.Log10(val)}
		},
	}
}

func expFunc() object.Object {
	return &object.Builtin{
		Fn: func(node *ast.CallExpression, args ...object.Object) object.Object {
			if len(args) != 1 {
				return object.NewError(node.Line(), node.Column(), "math.exp() takes one argument")
			}
			val, ok := extractFloat(args[0])
			if !ok {
				return object.NewError(node.Line(), node.Column(), "argument to math.exp must be a number")
			}
			return &object.Float{Value: math.Exp(val)}
		},
	}
}

func truncFunc() object.Object {
	return &object.Builtin{
		Fn: func(node *ast.CallExpression, args ...object.Object) object.Object {
			if len(args) != 1 {
				return object.NewError(node.Line(), node.Column(), "math.trunc() takes one argument")
			}
			val, ok := extractFloat(args[0])
			if !ok {
				return object.NewError(node.Line(), node.Column(), "argument to math.trunc must be a number")
			}
			return &object.Float{Value: math.Trunc(val)}
		},
	}
}

func minFunc() object.Object {
	return &object.Builtin{
		Fn: func(node *ast.CallExpression, args ...object.Object) object.Object {
			if len(args) < 1 {
				return object.NewError(node.Line(), node.Column(), "math.min() takes at least one argument")
			}
			min, ok := extractFloat(args[0])
			if !ok {
				return object.NewError(node.Line(), node.Column(), "arguments to math.min must be numbers")
			}
			for _, arg := range args[1:] {
				val, ok := extractFloat(arg)
				if !ok {
					return object.NewError(node.Line(), node.Column(), "arguments to math.min must be numbers")
				}
				if val < min {
					min = val
				}
			}
			return &object.Float{Value: min}
		},
	}
}

func maxFunc() object.Object {
	return &object.Builtin{
		Fn: func(node *ast.CallExpression, args ...object.Object) object.Object {
			if len(args) < 1 {
				return object.NewError(node.Line(), node.Column(), "math.max() takes at least one argument")
			}
			max, ok := extractFloat(args[0])
			if !ok {
				return object.NewError(node.Line(), node.Column(), "arguments to math.max must be numbers")
			}
			for _, arg := range args[1:] {
				val, ok := extractFloat(arg)
				if !ok {
					return object.NewError(node.Line(), node.Column(), "arguments to math.max must be numbers")
				}
				if val > max {
					max = val
				}
			}
			return &object.Float{Value: max}
		},
	}
}
