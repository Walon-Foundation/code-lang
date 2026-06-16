package time

import (
	"time"

	"github.com/walonCode/code-lang/internal/ast"
	"github.com/walonCode/code-lang/internal/object"
)

func Module() *object.Module {
	return &object.Module{
		Members: map[string]object.Object{
			"now":     nowFunc(),
			"sleep":   sleepFunc(),
			"unix":    unixFunc(),
			"format":  formatFunc(),
			"since":   sinceFunc(),
			"year":    yearFunc(),
			"month":   monthFunc(),
			"day":     dayFunc(),
			"hour":    hourFunc(),
			"minute":  minuteFunc(),
			"second":  secondFunc(),
			"RFC3339": &object.String{Value: time.RFC3339},
			"Kitchen": &object.String{Value: time.Kitchen},
		},
	}
}

func nowFunc() object.Object {
	return &object.Builtin{
		Fn: func(node *ast.CallExpression, args ...object.Object) object.Object {
			return &object.Time{Value: time.Now()}
		},
	}
}

func sleepFunc() object.Object {
	return &object.Builtin{
		Fn: func(node *ast.CallExpression, args ...object.Object) object.Object {
			if len(args) != 1 {
				return object.NewError(node.Line(), node.Column(), "time.sleep() takes 1 argument (ms)")
			}
			ms, ok := args[0].(*object.Integer)
			if !ok {
				return object.NewError(node.Line(), node.Column(), "argument to time.sleep must be an integer")
			}
			time.Sleep(time.Duration(ms.Value) * time.Millisecond)
			return nil
		},
	}
}

func unixFunc() object.Object {
	return &object.Builtin{
		Fn: func(node *ast.CallExpression, args ...object.Object) object.Object {
			return &object.Integer{Value: time.Now().Unix()}
		},
	}
}

func formatFunc() object.Object {
	return &object.Builtin{
		Fn: func(node *ast.CallExpression, args ...object.Object) object.Object {
			if len(args) != 2 {
				return object.NewError(node.Line(), node.Column(), "time.format() takes 2 arguments: time and layout")
			}
			tObj, ok1 := args[0].(*object.Time)
			layout, ok2 := args[1].(*object.String)
			if !ok1 || !ok2 {
				return object.NewError(node.Line(), node.Column(), "arguments must be (Time, String)")
			}
			return &object.String{Value: tObj.Value.Format(layout.Value)}
		},
	}
}

func sinceFunc() object.Object {
	return &object.Builtin{
		Fn: func(node *ast.CallExpression, args ...object.Object) object.Object {
			if len(args) != 1 {
				return object.NewError(node.Line(), node.Column(), "time.since() takes 1 argument (Time)")
			}
			tObj, ok := args[0].(*object.Time)
			if !ok {
				return object.NewError(node.Line(), node.Column(), "argument must be a Time object")
			}
			duration := time.Since(tObj.Value)
			return &object.Integer{Value: int64(duration.Milliseconds())}
		},
	}
}

func yearFunc() object.Object {
	return &object.Builtin{
		Fn: func(node *ast.CallExpression, args ...object.Object) object.Object {
			if len(args) != 1 {
				return object.NewError(node.Line(), node.Column(), "time.year() takes 1 argument (Time)")
			}
			tObj, ok := args[0].(*object.Time)
			if !ok {
				return object.NewError(node.Line(), node.Column(), "argument must be a Time object")
			}
			return &object.Integer{Value: int64(tObj.Value.Year())}
		},
	}
}

func monthFunc() object.Object {
	return &object.Builtin{
		Fn: func(node *ast.CallExpression, args ...object.Object) object.Object {
			if len(args) != 1 {
				return object.NewError(node.Line(), node.Column(), "time.month() takes 1 argument (Time)")
			}
			tObj, ok := args[0].(*object.Time)
			if !ok {
				return object.NewError(node.Line(), node.Column(), "argument must be a Time object")
			}
			return &object.Integer{Value: int64(tObj.Value.Month())}
		},
	}
}

func dayFunc() object.Object {
	return &object.Builtin{
		Fn: func(node *ast.CallExpression, args ...object.Object) object.Object {
			if len(args) != 1 {
				return object.NewError(node.Line(), node.Column(), "time.day() takes 1 argument (Time)")
			}
			tObj, ok := args[0].(*object.Time)
			if !ok {
				return object.NewError(node.Line(), node.Column(), "argument must be a Time object")
			}
			return &object.Integer{Value: int64(tObj.Value.Day())}
		},
	}
}

func hourFunc() object.Object {
	return &object.Builtin{
		Fn: func(node *ast.CallExpression, args ...object.Object) object.Object {
			if len(args) != 1 {
				return object.NewError(node.Line(), node.Column(), "time.hour() takes 1 argument (Time)")
			}
			tObj, ok := args[0].(*object.Time)
			if !ok {
				return object.NewError(node.Line(), node.Column(), "argument must be a Time object")
			}
			return &object.Integer{Value: int64(tObj.Value.Hour())}
		},
	}
}

func minuteFunc() object.Object {
	return &object.Builtin{
		Fn: func(node *ast.CallExpression, args ...object.Object) object.Object {
			if len(args) != 1 {
				return object.NewError(node.Line(), node.Column(), "time.minute() takes 1 argument (Time)")
			}
			tObj, ok := args[0].(*object.Time)
			if !ok {
				return object.NewError(node.Line(), node.Column(), "argument must be a Time object")
			}
			return &object.Integer{Value: int64(tObj.Value.Minute())}
		},
	}
}

func secondFunc() object.Object {
	return &object.Builtin{
		Fn: func(node *ast.CallExpression, args ...object.Object) object.Object {
			if len(args) != 1 {
				return object.NewError(node.Line(), node.Column(), "time.second() takes 1 argument (Time)")
			}
			tObj, ok := args[0].(*object.Time)
			if !ok {
				return object.NewError(node.Line(), node.Column(), "argument must be a Time object")
			}
			return &object.Integer{Value: int64(tObj.Value.Second())}
		},
	}
}
