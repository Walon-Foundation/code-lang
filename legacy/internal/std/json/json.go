package json

import (
	"encoding/json"
	"fmt"

	"github.com/walonCode/code-lang/internal/ast"
	"github.com/walonCode/code-lang/internal/object"
)

func JsonModule() *object.Module {
	return &object.Module{
		Members: map[string]object.Object{
			"parse":     parse(),
			"stringify": stringify(),
		},
	}
}

func parse() object.Object {
	return &object.Builtin{
		Fn: func(node *ast.CallExpression, args ...object.Object) object.Object {
			if len(args) != 1 {
				return object.NewError(node.Line(), node.Column(), "json.parse expect 1 argument")
			}

			strObj, ok := args[0].(*object.String)
			if !ok {
				return object.NewError(node.Line(), node.Column(), "json.parse argument must be a string")
			}

			var data any
			if err := json.Unmarshal([]byte(strObj.Value), &data); err != nil {
				return object.NewError(node.Line(), node.Column(), "json.parse error")
			}

			return toObject(data)
		},
	}
}

func toObject(val any) object.Object {
	switch v := val.(type) {
	case string:
		return &object.String{Value: v}
	case float64: // JSON numbers are float64
		return &object.Integer{Value: int64(v)}
	case bool:
		return &object.Boolean{Value: v}
	case []any:
		arr := &object.Array{Elements: []object.Object{}}
		for _, elem := range v {
			arr.Elements = append(arr.Elements, toObject(elem))
		}
		return arr
	case map[string]any:
		pairs := make(map[object.HashKey]object.HashPair)
		for key, elem := range v {
			objVal := toObject(elem)
			strObj := &object.String{Value: key}
			pairs[strObj.HashKey()] = object.HashPair{
				Key:strObj,
				Value: objVal,
			}
		}
		return &object.Hash{Pairs: pairs}
	default:
		return &object.String{Value: fmt.Sprintf("%v", v)}
	}
}

func stringify() object.Object {
	return &object.Builtin{
		Fn: func(node *ast.CallExpression, args ...object.Object) object.Object {
			if len(args) != 1 {
				return object.NewError(node.Line(), node.Column(), "json.stringify expects 1 argument")
			}

			data, err := toGoValue(args[0])
			if err != nil {
				fmt.Println("err2:", err)
				return object.NewError(node.Line(), node.Column(), "json.stringify: error")
			}

			bytes, err := json.Marshal(data)
			if err != nil {
				fmt.Println("err 3:", err)
				return object.NewError(node.Line(), node.Column(), "json.stringify: error")
			}

			return &object.String{Value: string(bytes)}
		},
	}
}

// recursively convert your Object to Go native types
func toGoValue(obj object.Object) (any, error) {
	switch o := obj.(type) {
	case *object.String:
		return o.Value, nil
	case *object.Integer:
		return o.Value, nil
	case *object.Boolean:
		return o.Value, nil
	case *object.Array:
		arr := make([]any, len(o.Elements))
		for i, elem := range o.Elements {
			v, err := toGoValue(elem)
			if err != nil {
				return nil, err
			}
			arr[i] = v
		}
		return arr, nil
	case *object.Hash:
		m := map[string]any{}
		for _, pair := range o.Pairs {
			keyStr, ok := pair.Key.(*object.String)
			if !ok {
				return nil, fmt.Errorf("json.stringify: hash key must be string")
			}
			val, err := toGoValue(pair.Value)
			if err != nil {
				return nil, err
			}
			m[keyStr.Value] = val
		}
		return m, nil
	default:
		return nil, fmt.Errorf("unsupported type for JSON: %T", obj)
	}
}
