package net

import (
	"fmt"
	"net/http"

	"github.com/walonCode/code-lang/internal/ast"
	"github.com/walonCode/code-lang/internal/object"
)

type ApplyFunctionFunc func(fn object.Object, args []object.Object, node *ast.CallExpression)object.Object

func NetModule(applyFunc ApplyFunctionFunc) *object.Module {
	members := map[string]object.Object{}

	members["server"] = &object.Builtin{
		Fn: func(node *ast.CallExpression, args ...object.Object) object.Object {
			return NewServer(applyFunc)
		},
	}

	return &object.Module{
		Members: members,
	}
}

func NewServer(applyFunc ApplyFunctionFunc) object.Object {
	server := &object.Server{
		Route: map[string]object.Object{},
		ApplyFunc: applyFunc,
		Members: map[string]object.Object{},
	}

	server.Members["listen"] = &object.Builtin{
		Fn: func(node *ast.CallExpression, args ...object.Object) object.Object {
			if len(args) == 0 || len(args) > 2 {
				return object.NewError(node.Line(), node.Column(), "listen expect 1 arugment")
			}

			port, ok := args[0].(*object.Integer)
			if !ok {
				return object.NewError(node.Line(), node.Column(), "port must be an integer")
			}

			var callback object.Object
			if len(args) == 2 {
				callback = args[1]
			}
			
			handler := func( w http.ResponseWriter, r *http.Request){
				key := r.Method + " " + r.URL.Path
				if routeFn, ok := server.Route[key]; ok {
					server.ApplyFunc(routeFn, []object.Object{}, node)
					return
				}
				
				w.Write([]byte("404 Not found"))
			}

			if callback != nil {
				switch cb := callback.(type) {
				case *object.Builtin:
					cb.Fn(nil) // pass nil for node
				case *object.Function:
					applyFunc(cb, []object.Object{}, node)
				default:
					return object.NewError(node.Line(), node.Column(), "callback must be a function")
				}
			}

			err := http.ListenAndServe(fmt.Sprintf(":%d", port.Value), http.HandlerFunc(handler))
			if err != nil {
				fmt.Println("server error", err)
			}

			return nil
		},
	}
	
	server.Members["on"] = &object.Builtin{
		Fn: func(node *ast.CallExpression, args ...object.Object) object.Object {
   			if len(args) != 3 {
                return object.NewError(node.Line(), node.Column(), "on expects 3 arguments: method, path, handler")
            }

            methodStr, ok := args[0].(*object.String)
            if !ok {
                return object.NewError(node.Line(), node.Column(), "method must be a string")
            }

            pathStr, ok := args[1].(*object.String)
            if !ok {
                return object.NewError(node.Line(), node.Column(), "path must be a string")
            }

            callback := args[2]
            switch callback.(type) {
            case *object.Function,*object.Builtin:
            	//ok
            default:
                return object.NewError(node.Line(), node.Column(), "handler must be a function")
            }

            key := methodStr.Value + " " + pathStr.Value
            server.Route[key] = callback

            return nil
		},
	}

	return server
}
