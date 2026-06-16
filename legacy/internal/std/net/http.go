package net

import (
	"io"
	"net/http"
	"strings"

	"github.com/walonCode/code-lang/internal/ast"
	"github.com/walonCode/code-lang/internal/object"
)

func HttpModule() *object.Module {
	return &object.Module{
		Members: map[string]object.Object{
			"get":    httpGet(),
			"post":   httpPost(),
			"patch":  httpPatch(),
			"delete": httpDelete(),
		},
	}
}

func evalHttpResponse(node *ast.CallExpression, resp *http.Response, err error) object.Object {
	if err != nil {
		return object.NewError(node.Line(), node.Column(), "http request failed: %s", err.Error())
	}
	defer resp.Body.Close()

	data, err := io.ReadAll(resp.Body)
	if err != nil {
		return object.NewError(node.Line(), node.Column(), "failed to read the http response: %s", err.Error())
	}

	headers := make(map[string]object.Object)
	for k, v := range resp.Header {
		arr := &object.Array{Elements: []object.Object{}}
		for _, val := range v {
			arr.Elements = append(arr.Elements, &object.String{Value: val})
		}
		headers[k] = arr
	}

	return &object.Module{
		Members: map[string]object.Object{
			"status":  &object.Integer{Value: int64(resp.StatusCode)},
			"body":    &object.String{Value: string(data)},
			"headers": &object.Module{Members: headers},
		},
	}
}

func httpGet() object.Object {
	return &object.Builtin{
		Fn: func(node *ast.CallExpression, args ...object.Object) object.Object {
			if len(args) != 1 {
				return object.NewError(node.Line(), node.Column(), "http.get expects 1 argument (url)")
			}

			url, ok := args[0].(*object.String)
			if !ok {
				return object.NewError(node.Line(), node.Column(), "url must be a string")
			}

			resp, err := http.Get(url.Value)
			return evalHttpResponse(node, resp, err)
		},
	}
}

func httpPost() object.Object {
	return &object.Builtin{
		Fn: func(node *ast.CallExpression, args ...object.Object) object.Object {
			if len(args) < 2 || len(args) > 3 {
				return object.NewError(node.Line(), node.Column(), "http.post expects 2 or 3 arguments (url, body, [contentType])")
			}

			url, ok := args[0].(*object.String)
			if !ok {
				return object.NewError(node.Line(), node.Column(), "url must be a string")
			}

			body, ok := args[1].(*object.String)
			if !ok {
				return object.NewError(node.Line(), node.Column(), "body must be a string")
			}

			contentType := "application/json"
			if len(args) == 3 {
				ct, ok := args[2].(*object.String)
				if !ok {
					return object.NewError(node.Line(), node.Column(), "contentType must be a string")
				}
				contentType = ct.Value
			}

			resp, err := http.Post(url.Value, contentType, strings.NewReader(body.Value))
			return evalHttpResponse(node, resp, err)
		},
	}
}

func httpPatch() object.Object {
	return &object.Builtin{
		Fn: func(node *ast.CallExpression, args ...object.Object) object.Object {
			if len(args) < 2 || len(args) > 3 {
				return object.NewError(node.Line(), node.Column(), "http.patch expects 2 or 3 arguments (url, body, [contentType])")
			}

			url, ok := args[0].(*object.String)
			if !ok {
				return object.NewError(node.Line(), node.Column(), "url must be a string")
			}

			body, ok := args[1].(*object.String)
			if !ok {
				return object.NewError(node.Line(), node.Column(), "body must be a string")
			}

			req, err := http.NewRequest(http.MethodPatch, url.Value, strings.NewReader(body.Value))
			if err != nil {
				return object.NewError(node.Line(), node.Column(), "failed to create patch request: %s", err.Error())
			}

			contentType := "application/json"
			if len(args) == 3 {
				ct, ok := args[2].(*object.String)
				if !ok {
					return object.NewError(node.Line(), node.Column(), "contentType must be a string")
				}
				contentType = ct.Value
			}
			req.Header.Set("Content-Type", contentType)

			client := &http.Client{}
			resp, err := client.Do(req)
			return evalHttpResponse(node, resp, err)
		},
	}
}

func httpDelete() object.Object {
	return &object.Builtin{
		Fn: func(node *ast.CallExpression, args ...object.Object) object.Object {
			if len(args) != 1 {
				return object.NewError(node.Line(), node.Column(), "http.delete expects 1 argument (url)")
			}

			url, ok := args[0].(*object.String)
			if !ok {
				return object.NewError(node.Line(), node.Column(), "url must be a string")
			}

			req, err := http.NewRequest(http.MethodDelete, url.Value, nil)
			if err != nil {
				return object.NewError(node.Line(), node.Column(), "failed to create delete request: %s", err.Error())
			}

			client := &http.Client{}
			resp, err := client.Do(req)
			return evalHttpResponse(node, resp, err)
		},
	}
}
