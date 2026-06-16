package fs

import (
	"os"

	"github.com/walonCode/code-lang/internal/ast"
	"github.com/walonCode/code-lang/internal/object"
)

func Module()*object.Module{
	return &object.Module{
		Members: map[string]object.Object{
			"readfile":readFile(),
			"writefile":writeFile(),
		},
	}
}

func readFile()object.Object{
	return &object.Builtin{
		Fn: func(node *ast.CallExpression, args ...object.Object) object.Object {
			if len(args) != 1 {
				return object.NewError(node.Line(), node.Column(), "fs.readfile() takes 1 argument")
			}
			
			strObj, ok := args[0].(*object.String)
			if !ok {
				return object.NewError(node.Line(), node.Column(), "argument must be a string")
			}
			
			filepath := strObj.Value
			
			data, err := os.ReadFile(filepath)
			if err != nil {
				return object.NewError(node.Line(), node.Column(), "invalid filepath")
			}
			
			return &object.String{Value: string(data)}
		},
	}
}

func writeFile()object.Object{
	return &object.Builtin{
		Fn: func(node *ast.CallExpression, args ...object.Object) object.Object {
			if len(args)!= 2 {
				return object.NewError(node.Line(), node.Column(), "fs.writefile() takes 2 argument")
			}
			
			fileObj, ok := args[0].(*object.String)
			if !ok {
				return object.NewError(node.Line(), node.Column(), "fs.writefile() first argument must be a string")
			}
			
			dataObj, ok := args[1].(*object.String)
			if !ok {
				return object.NewError(node.Line(), node.Column(), "fs.writefile() second argument must be a string")
			}
			
			dataToWrite := dataObj.Value
		 	filePath := fileObj.Value
				
			if err := os.WriteFile(filePath, []byte(dataToWrite), 0774);err != nil {
				return object.NewError(node.Line(), node.Column(), "failed to write file")
			}
			
			
			return &object.Boolean{Value: true}
		},
	}
}