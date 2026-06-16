package hash

import (
	"maps"

	"github.com/walonCode/code-lang/internal/ast"
	"github.com/walonCode/code-lang/internal/object"
)

func Module() *object.Module {
	return &object.Module{
		Members: map[string]object.Object{
			"keys":    keysFunc(),
			"values":  valuesFunc(),
			"has_key": containsKeyFunc(),
			"merge":   mergeFunc(),
			"delete":  deleteFunc(),
		},
	}
}

func keysFunc() object.Object {
	return &object.Builtin{
		Fn: func(node *ast.CallExpression, args ...object.Object) object.Object {
			if len(args) != 1 {
				return object.NewError(node.Line(), node.Column(), "hash.keys() takes 1 argument")
			}
			hash, ok := args[0].(*object.Hash)
			if !ok {
				return object.NewError(node.Line(), node.Column(), "argument must be a hash")
			}

			keys := make([]object.Object, 0, len(hash.Pairs))
			for _, pair := range hash.Pairs {
				keys = append(keys, pair.Key)
			}
			return &object.Array{Elements: keys}
		},
	}
}

func valuesFunc() object.Object {
	return &object.Builtin{
		Fn: func(node *ast.CallExpression, args ...object.Object) object.Object {
			if len(args) != 1 {
				return object.NewError(node.Line(), node.Column(), "hash.values() takes 1 argument")
			}
			hash, ok := args[0].(*object.Hash)
			if !ok {
				return object.NewError(node.Line(), node.Column(), "argument must be a hash")
			}

			values := make([]object.Object, 0, len(hash.Pairs))
			for _, pair := range hash.Pairs {
				values = append(values, pair.Value)
			}
			return &object.Array{Elements: values}
		},
	}
}

func containsKeyFunc() object.Object {
	return &object.Builtin{
		Fn: func(node *ast.CallExpression, args ...object.Object) object.Object {
			if len(args) != 2 {
				return object.NewError(node.Line(), node.Column(), "hash.has_key() takes 2 arguments: hash and key")
			}
			hash, ok := args[0].(*object.Hash)
			if !ok {
				return object.NewError(node.Line(), node.Column(), "first argument must be a hash")
			}

			hashable, ok := args[1].(object.Hashable)
			if !ok {
				return object.NewError(node.Line(), node.Column(), "key must be hashable")
			}

			_, ok = hash.Pairs[hashable.HashKey()]
			return &object.Boolean{Value: ok}
		},
	}
}

func mergeFunc() object.Object {
	return &object.Builtin{
		Fn: func(node *ast.CallExpression, args ...object.Object) object.Object {
			if len(args) != 2 {
				return object.NewError(node.Line(), node.Column(), "hash.merge() takes 2 arguments")
			}
			h1, ok1 := args[0].(*object.Hash)
			h2, ok2 := args[1].(*object.Hash)
			if !ok1 || !ok2 {
				return object.NewError(node.Line(), node.Column(), "both arguments must be hashes")
			}

			newPairs := make(map[object.HashKey]object.HashPair)
			maps.Copy(newPairs, h1.Pairs)
			maps.Copy(newPairs, h2.Pairs)

			return &object.Hash{Pairs: newPairs}
		},
	}
}

func deleteFunc() object.Object {
	return &object.Builtin{
		Fn: func(node *ast.CallExpression, args ...object.Object) object.Object {
			if len(args) != 2 {
				return object.NewError(node.Line(), node.Column(), "hash.delete() takes 2 arguments: hash and key")
			}
			hash, ok := args[0].(*object.Hash)
			if !ok {
				return object.NewError(node.Line(), node.Column(), "first argument must be a hash")
			}

			hashable, ok := args[1].(object.Hashable)
			if !ok {
				return object.NewError(node.Line(), node.Column(), "key must be hashable")
			}

			delete(hash.Pairs, hashable.HashKey())
			return hash
		},
	}
}
