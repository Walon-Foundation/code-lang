package evaluator

import (
	"maps"
	"math"
	"os"
	"path/filepath"
	"strings"

	"github.com/walonCode/code-lang/internal/ast"
	"github.com/walonCode/code-lang/internal/lexer"
	"github.com/walonCode/code-lang/internal/object"
	"github.com/walonCode/code-lang/internal/parser"
)

var moduleCache = map[string]*object.Module{}

type Evaluator struct {
	loopDepth   int
	Resolutions map[ast.Node]int
}

func (e *Evaluator) Eval(node ast.Node, env *object.Environment) object.Object {
	switch node := node.(type) {
	//statement
	case *ast.Program:
		return e.evalProgram(node, env)
	case *ast.ExpressionStatement:
		return e.Eval(node.Expression, env)
	case *ast.ReturnStatement:
		val := e.Eval(node.ReturnValue, env)
		if isError(val) {
			return val
		}
		return &object.ReturnValue{Value: val}
	case *ast.BreakStatement:
		return e.evalBreakStatement(node)
	case *ast.ContinueStatement:
		return e.evalContinueStatement(node)
	case *ast.ImportStatement:
		return e.evalImportStatement(node, env)
	case *ast.LetStatement:
		val := e.Eval(node.Value, env)
		if isError(val) {
			return val
		}
		env.Set(node.Name.Value, val)
	case *ast.ConstStatement:
		val := e.Eval(node.Value, env)
		if isError(val) {
			return val
		}
		env.SetConst(node.Name.Value, val)
	case *ast.StructStatement:
		defaults := make(map[string]object.Object)
		for field, exp := range node.Fields {
			val := e.Eval(exp, env)
			if isError(val) {
				return val
			}
			defaults[field] = val
		}

		structType := &object.StructType{
			Name:     node.Name.Value,
			Defaults: defaults,
		}
		env.Set(node.Name.Value, structType)
		return object.NULL

	//expression
	case *ast.IntegerLiteral:
		return &object.Integer{Value: node.Value}
	case *ast.FloatLiteral:
		return &object.Float{Value: node.Value}
	case *ast.StringLiteral:
		return &object.String{Value: node.Value}
	case *ast.CharLiteral:
		return &object.Char{Value: node.Value}
	case *ast.Boolean:
		return nativeBoolToBooleanObject(node.Value)
	case *ast.StructLiteral:
		obj, ok := env.Get(node.Name.Value)
		if !ok {
			return object.NewError(
				node.Line(),
				node.Column(),
				"unknown struct: %s",
				node.Name.Value,
			)
		}
		st, ok := obj.(*object.StructType)
		if !ok {
			return object.NewError(
				node.Line(),
				node.Column(),
				"%s is not a struct",
				node.Name.Value,
			)
		}
		fields := make(map[string]object.Object)
		maps.Copy(st.Defaults, fields)

		for k, exp := range node.Fields {
			val := e.Eval(exp, env)
			if isError(val) {
				return val
			}
			fields[k] = val
		}

		return &object.StructInstance{
			TypeName: st.Name,
			Fields:   fields,
		}
	case *ast.PrefixExpression:
		right := e.Eval(node.Right, env)
		if isError(right) {
			return right
		}
		return evalPrefixExpression(node, right)
	case *ast.UpdateExpression:
		return e.evalUpdateExpression(node, env)
	case *ast.InfixExpression:
		if isAssignment(node.Operator) {
			return e.evalAssignment(node, env)
		}

		// Short-circuit logical operators
		if node.Operator == "&&" || node.Operator == "||" {
			return e.evalLogicalExpression(node, env)
		}

		left := e.Eval(node.Left, env)
		if isError(left) {
			return left
		}
		right := e.Eval(node.Right, env)
		if isError(right) {
			return right
		}
		return evalInfixExpression(node, left, right)
	case *ast.BlockStatement:
		return e.evalBlockStatements(node, env)
	case *ast.IfExpression:
		return e.evalIfExpression(node, env)
	case *ast.Identifier:
		return e.evalIdentifier(node, env)
	case *ast.FunctionLiteral:
		params := node.Parameters
		body := node.Body
		return &object.Function{Parameters: params, Env: env, Body: &body}
	case *ast.CallExpression:
		function := e.Eval(node.Function, env)
		if isError(function) {
			return function
		}
		args := e.evalExpression(node.Arguments, env)
		if len(args) == 1 && isError(args[0]) {
			return args[0]
		}
		return e.applyFunction(function, args, node)
	case *ast.ArrayLiteral:
		elements := e.evalExpression(node.Elements, env)
		if len(elements) == 1 && isError(elements[0]) {
			return elements[0]
		}
		return &object.Array{Elements: elements}
	case *ast.ForExpression:
		return e.evalForExpression(node, env)
	case *ast.WhileExpression:
		return e.evalWhileExpression(node, env)
	case *ast.IndexExpression:
		left := e.Eval(node.Left, env)
		if isError(left) {
			return left
		}
		index := e.Eval(node.Index, env)
		if isError(index) {
			return index
		}
		return evalIndexExpression(left, index, node)
	case *ast.HashLiteral:
		return e.evalHashLiteral(node, env)
	case *ast.MemberExpression:
		obj := e.Eval(node.Object, env)
		if isError(obj) {
			return obj
		}

		return evalMemberExpression(obj, node)
	}

	return nil
}

func (e *Evaluator) evalBreakStatement(node *ast.BreakStatement) object.Object {
	if e.loopDepth == 0 {
		return object.NewError(node.Line(), node.Column(), "break not inside a loop")
	}

	return &object.Break{}
}

func (e *Evaluator) evalContinueStatement(node *ast.ContinueStatement) object.Object {
	if e.loopDepth == 0 {
		return object.NewError(node.Line(), node.Column(), "continue not inside a loop")
	}

	return &object.Continue{}
}

func (e *Evaluator) evalImportStatement(node *ast.ImportStatement, env *object.Environment) object.Object {
	modulePath := node.Path

	if mod, ok := moduleCache[modulePath]; ok {
		env.Set(modulePath, mod)
		return mod
	}

	fileName := filepath.Clean(modulePath + ".cl")
	content, err := os.ReadFile(fileName)
	if err != nil {
		return object.NewError(node.Line(), node.Column(), "could not read module %q : %s", modulePath, err)
	}

	moduleEnv := object.NewEnclosedEnvironment(env)

	l := lexer.New(string(content))
	p := parser.New(l)
	programe := p.ParsePrograme()

	e.Eval(programe, moduleEnv)

	moduleobj := &object.Module{Members: map[string]object.Object{}}
	maps.Copy(moduleobj.Members, moduleEnv.Store)
	env.Set(modulePath, moduleobj)

	moduleCache[modulePath] = moduleobj

	return moduleobj
}

func isAssignment(op string) bool {
	switch op {
	case "=", "+=", "-=", "*=", "/=", "%=", "**=", "//=":
		return true
	default:
		return false
	}
}

func (e *Evaluator) evalAssignment(node *ast.InfixExpression, env *object.Environment) object.Object {
	val := e.Eval(node.Right, env)
	if isError(val) {
		return val
	}

	switch left := node.Left.(type) {
	case *ast.Identifier:
		var finalVal object.Object
		if node.Operator == "=" {
			finalVal = val
		} else {
			currentVal := e.evalIdentifier(left, env)
			if isError(currentVal) {
				return currentVal
			}
			finalVal = evalInfixExpression(node, currentVal, val)
		}

		if isError(finalVal) {
			return finalVal
		}

		if dist, ok := e.Resolutions[left]; ok {
			if !env.UpdateAt(dist, left.Value, finalVal) {
				return object.NewError(node.Line(), node.Column(), "cannot reassign to const: %s", left.Value)
			}
			return finalVal
		}

		_, updated := env.Update(left.Value, finalVal)
		if !updated {
			env.Set(left.Value, finalVal)
		}
		return finalVal

	case *ast.MemberExpression:
		obj := e.Eval(left.Object, env)
		if isError(obj) {
			return obj
		}

		var finalVal object.Object
		if node.Operator == "=" {
			finalVal = val
		} else {
			currentVal := evalMemberExpression(obj, left)
			if isError(currentVal) {
				return currentVal
			}
			finalVal = evalInfixExpression(node, currentVal, val)
		}

		if isError(finalVal) {
			return finalVal
		}

		return evalAssignMember(obj, left, finalVal)

	case *ast.IndexExpression:
		obj := e.Eval(left.Left, env)
		if isError(obj) {
			return obj
		}

		idx := e.Eval(left.Index, env)
		if isError(idx) {
			return idx
		}

		var finalVal object.Object
		if node.Operator == "=" {
			finalVal = val
		} else {
			currentVal := evalIndexExpression(obj, idx, left)
			if isError(currentVal) {
				return currentVal
			}
			finalVal = evalInfixExpression(node, currentVal, val)
		}

		if isError(finalVal) {
			return finalVal
		}

		return evalAssignIndex(obj, idx, finalVal, left)
	}

	return object.NewError(node.Line(), node.Column(), "invalid left-hand side in assignment")
}

func evalAssignIndex(obj, idx, val object.Object, node *ast.IndexExpression) object.Object {
	switch obj := obj.(type) {
	case *object.Array:
		i, ok := idx.(*object.Integer)
		if !ok {
			return object.NewError(node.Line(), node.Column(), "index must be an integer, got %s", idx.Type())
		}
		if i.Value < 0 || i.Value >= int64(len(obj.Elements)) {
			return object.NewError(node.Line(), node.Column(), "index out of range: %d", i.Value)
		}
		obj.Elements[i.Value] = val
		return val
	case *object.Hash:
		hashKey, ok := idx.(object.Hashable)
		if !ok {
			return object.NewError(node.Line(), node.Column(), "unusable as hash key: %s", idx.Type())
		}
		obj.Pairs[hashKey.HashKey()] = object.HashPair{Key: idx, Value: val}
		return val
	default:
		return object.NewError(node.Line(), node.Column(), "index assignment not supported for %s", obj.Type())
	}
}

func evalAssignMember(obj object.Object, node *ast.MemberExpression, val object.Object) object.Object {
	switch obj := obj.(type) {
	case *object.Hash:
		key := &object.String{Value: node.Property.Value}
		obj.Pairs[key.HashKey()] = object.HashPair{
			Key:   key,
			Value: val,
		}

		return val
	case *object.Module:
		obj.Members[node.Property.Value] = val
		return val
	case *object.Server:
		obj.Members[node.Property.Value] = val
		return val
	default:
		return object.NewError(node.Line(), node.Column(), "cannot assign to property %s on %s", node.Property.Value, obj.Type())
	}
}

func evalMemberExpression(obj object.Object, node *ast.MemberExpression) object.Object {
	switch obj := obj.(type) {
	case *object.Hash:
		key := &object.String{Value: node.Property.Value}
		if val, ok := obj.Pairs[object.HashKey(key.HashKey())]; ok {
			return val.Value
		}
		return object.NewError(node.Line(), node.Column(), "property not found: %s", node.Property.Value)
	case *object.Module:
		val, ok := obj.Members[node.Property.Value]
		if !ok {
			return object.NewError(node.Line(), node.Column(), "module has not member %s", node.Property.Value)
		}

		return val
	case *object.Server:
		val, ok := obj.Members[node.Property.Value]
		if !ok {
			return object.NewError(node.Line(), node.Column(), "server has not member %s", node.Property.Value)
		}

		return val
	case *object.StructInstance:
		val, ok := obj.Fields[node.Property.Value]
		if !ok {
			return object.NewError(node.Line(), node.Column(), "unknown field %s on  %s", node.Property.Value, obj.TypeName)
		}
		return val
	default:
		return object.NewError(node.Line(), node.Column(), "cannot access property %s on %s", node.Property.Value, obj.Type())
	}
}

func (e *Evaluator) evalWhileExpression(node *ast.WhileExpression, env *object.Environment) object.Object {
	var result object.Object = object.NULL

	e.loopDepth++
	defer func() { e.loopDepth-- }()

	for {
		if node.Condition != nil {
			condition := e.Eval(node.Condition, env)
			if isError(condition) {
				return condition
			}

			if !isTruthy(condition) {
				break
			}
		}

		result = e.Eval(node.Body, env)
		switch result.(type) {
		case *object.Break:
			return object.NULL
		case *object.Continue:
			continue
		case *object.ReturnValue:
			return result
		}

		if result != nil {
			rt := result.Type()
			if rt == object.RETURN_VALUE_OBJ || rt == object.ERROR_OBJ {
				return result
			}
		}
	}

	return result
}

func (e *Evaluator) evalForExpression(node *ast.ForExpression, env *object.Environment) object.Object {
	forEnv := object.NewEnclosedEnvironment(env)
	if node.Init != nil {
		initRes := e.Eval(node.Init, forEnv)
		if isError(initRes) {
			return initRes
		}
	}

	var result object.Object = object.NULL
	e.loopDepth++
	defer func() { e.loopDepth-- }()

	for {
		if node.Condition != nil {
			condition := e.Eval(node.Condition, forEnv)
			if isError(condition) {
				return condition
			}
			if !isTruthy(condition) {
				break
			}
		}

		result = e.Eval(node.Body, forEnv)
		switch result.(type) {
		case *object.Break:
			return object.NULL
		case *object.Continue:
			continue
		case *object.ReturnValue:
			return result
		}

		if result != nil {
			rt := result.Type()
			if rt == object.RETURN_VALUE_OBJ || rt == object.ERROR_OBJ {
				return result
			}
		}

		if node.Post != nil {
			postRes := e.Eval(node.Post, forEnv)
			if isError(postRes) {
				return postRes
			}
		}
	}

	return result
}

func (e *Evaluator) evalHashLiteral(node *ast.HashLiteral, env *object.Environment) object.Object {
	pairs := make(map[object.HashKey]object.HashPair)

	for keyNode, valueNode := range node.Pairs {
		key := e.Eval(keyNode, env)
		if isError(key) {
			return key
		}

		hashkey, ok := key.(object.Hashable)
		if !ok {
			return object.NewError(node.Line(), node.Column(), "unusable as hash key: %s", key.Type())
		}

		value := e.Eval(valueNode, env)
		if isError(value) {
			return value
		}

		hashed := hashkey.HashKey()
		pairs[hashed] = object.HashPair{Key: key, Value: value}
	}

	return &object.Hash{Pairs: pairs}
}

func evalIndexExpression(left, index object.Object, node *ast.IndexExpression) object.Object {
	switch {
	case left.Type() == object.ARRAY_OBJ && index.Type() == object.INTEGER_OBJ:
		return evalArrayIndexExpression(left, index)
	case left.Type() == object.HASH_OBJ:
		return evalHashIndexExpression(left, index, node)
	case left.Type() == object.STRING_OBJ && index.Type() == object.INTEGER_OBJ:
		return evalStringIndexExpression(left, index)
	default:
		return object.NewError(node.Line(), node.Column(), "index operator not supported: %s", left.Type())
	}
}

func evalStringIndexExpression(left, index object.Object) object.Object {
	strObject := left.(*object.String)
	idx := index.(*object.Integer).Value
	max := int64(len(strObject.Value))

	if idx < 0 || idx > max {
		return object.NULL
	}

	return &object.String{Value: string(strObject.Value[idx])}
}

func evalHashIndexExpression(hash, index object.Object, node *ast.IndexExpression) object.Object {
	hashObj := hash.(*object.Hash)

	key, ok := index.(object.Hashable)
	if !ok {
		return object.NewError(node.Line(), node.Column(), "unusable as hash key: %s", index.Type())
	}

	pair, ok := hashObj.Pairs[key.HashKey()]
	if !ok {
		return object.NULL
	}

	return pair.Value
}

func evalArrayIndexExpression(left, index object.Object) object.Object {
	arrayObj := left.(*object.Array)
	idx := index.(*object.Integer).Value
	max := int64(len(arrayObj.Elements) - 1)

	if idx < 0 || idx > max {
		return object.NULL
	}

	return arrayObj.Elements[idx]
}

func (e *Evaluator) applyFunction(fn object.Object, args []object.Object, node *ast.CallExpression) object.Object {
	switch fn := fn.(type) {
	case *object.Function:
		extendedEnv := extendFunctionEnv(fn, args)
		evaluated := e.Eval(fn.Body, extendedEnv)
		return unwrapReturnValue(evaluated)
	case *object.Builtin:
		return fn.Fn(node, args...)
	default:
		return object.NewError(node.Line(), node.Column(), "not a function: %s", fn.Type())
	}
}

func extendFunctionEnv(fn *object.Function, args []object.Object) *object.Environment {
	env := object.NewEnclosedEnvironment(fn.Env)

	for paramIdx, param := range fn.Parameters {
		env.Set(param.Value, args[paramIdx])
	}

	return env
}

func unwrapReturnValue(obj object.Object) object.Object {
	if returnValue, ok := obj.(*object.ReturnValue); ok {
		return returnValue.Value
	}
	return obj
}

func (e *Evaluator) evalExpression(exps []ast.Expression, env *object.Environment) []object.Object {
	var result []object.Object

	for _, f := range exps {
		evaluated := e.Eval(f, env)
		if isError(evaluated) {
			return []object.Object{evaluated}
		}
		result = append(result, evaluated)
	}
	return result
}

func (e *Evaluator) evalIdentifier(node *ast.Identifier, env *object.Environment) object.Object {
	if dist, ok := e.Resolutions[node]; ok {
		if val, ok := env.GetAt(dist, node.Value); ok {
			return val
		}
	}

	if val, ok := env.Get(node.Value); ok {
		return val
	}
	return object.NewError(node.Line(), node.Column(), "identifier not found: %s", node.Value)
}

func (e *Evaluator) evalProgram(program *ast.Program, env *object.Environment) object.Object {
	var result object.Object
	for _, statement := range program.Statements {
		result = e.Eval(statement, env)

		switch result := result.(type) {
		case *object.ReturnValue:
			return result.Value
		case *object.Error:
			return result
		}
	}

	return result
}

func (e *Evaluator) evalBlockStatements(block *ast.BlockStatement, env *object.Environment) object.Object {
	var result object.Object
	blockEnv := object.NewEnclosedEnvironment(env)

	for _, statement := range block.Statements {
		result = e.Eval(statement, blockEnv)

		if result != nil {
			rt := result.Type()
			if rt == object.RETURN_VALUE_OBJ || rt == object.ERROR_OBJ || rt == object.BREAK_OBJ || rt == object.CONTINUE_OBJ {
				return result
			}
		}
	}

	return result
}

func (e *Evaluator) evalIfExpression(node *ast.IfExpression, env *object.Environment) object.Object {
	condition := e.Eval(node.Condition, env)

	if isError(condition) {
		return condition
	}

	if isTruthy(condition) {
		return e.Eval(node.Consequence, env)
	}

	for _, v := range node.IfElse {
		condition := e.Eval(v.Condition, env)
		if isError(condition) {
			return condition
		}
		if isTruthy(condition) {
			return e.Eval(v.Consequence, env)
		}
	}

	if node.Alternative != nil {
		return e.Eval(node.Alternative, env)
	}

	return object.NULL
}

func isTruthy(obj object.Object) bool {
	switch obj {
	case object.NULL:
		return false
	case object.TRUE:
		return true
	case object.FALSE:
		return false
	default:
		return true
	}
}

// evalLogicalExpression handles && and || with short-circuit evaluation.
// For &&: returns the first falsy value, or the right value if both are truthy.
// For ||: returns the first truthy value, or the right value if both are falsy.
func (e *Evaluator) evalLogicalExpression(node *ast.InfixExpression, env *object.Environment) object.Object {
	left := e.Eval(node.Left, env)
	if isError(left) {
		return left
	}

	if node.Operator == "&&" {
		if !isTruthy(left) {
			return left
		}
		return e.Eval(node.Right, env)
	}

	// node.Operator == "||"
	if isTruthy(left) {
		return left
	}
	return e.Eval(node.Right, env)
}

func evalInfixExpression(node *ast.InfixExpression, left, right object.Object) object.Object {
	switch {
	case node.Operator == "&&":
		return nativeBoolToBooleanObject(isTruthy(left) && isTruthy(right))
	case node.Operator == "||":
		return nativeBoolToBooleanObject(isTruthy(left) || isTruthy(right))
	case left.Type() == object.INTEGER_OBJ && right.Type() == object.INTEGER_OBJ:
		return evalIntegerInfixExpression(node, left, right)
	case left.Type() == object.FLOAT_OBJ && right.Type() == object.FLOAT_OBJ:
		return evalFloatInfixExpression(node, left, right)
	case left.Type() == object.INTEGER_OBJ && right.Type() == object.FLOAT_OBJ:
		return evalFloatAndIntegerInfixExpression(node, left, right)
	case left.Type() == object.FLOAT_OBJ && right.Type() == object.INTEGER_OBJ:
		return evalFloatAndIntegerInfixExpression(node, left, right)
	case node.Operator == "==":
		return nativeBoolToBooleanObject(left == right)
	case node.Operator == "!=":
		return nativeBoolToBooleanObject(left != right)
	case left.Type() == object.STRING_OBJ && right.Type() == object.STRING_OBJ:
		return evalStringInfixExpression(node, left, right)
	case left.Type() == object.CHAR_OBJ && right.Type() == object.CHAR_OBJ:
		return evalCharInfixExpression(node, left, right)
	case isStringOrChar(left) && isStringOrChar(right):
		return evalMixedStringOrCharInfixExpression(node, left, right)
	default:
		if left.Type() != right.Type() {
			return object.NewError(node.Line(), node.Column(), "type mismatch: %s %s %s", left.Type(), node.Operator, right.Type())
		}
		return object.NewError(node.Line(), node.Column(), "unknown operator: %s %s %s", left.Type(), node.Operator, right.Type())
	}
}

func evalFloatAndIntegerInfixExpression(node *ast.InfixExpression, left, right object.Object) object.Object {
	var leftVal float64
	var rightVal float64

	if left.Type() == object.INTEGER_OBJ {
		leftVal = float64(left.(*object.Integer).Value)
	} else {
		leftVal = left.(*object.Float).Value
	}

	if right.Type() == object.INTEGER_OBJ {
		rightVal = float64(right.(*object.Integer).Value)
	} else {
		rightVal = right.(*object.Float).Value
	}

	switch node.Operator {
	case "+":
		return &object.Float{Value: leftVal + rightVal}
	case "-":
		return &object.Float{Value: leftVal - rightVal}
	case "*":
		return &object.Float{Value: leftVal * rightVal}
	case "/":
		if rightVal == 0 {
			return object.NewError(node.Line(), node.Column(), "division by zero: %f / %f", leftVal, rightVal)
		}
		return &object.Float{Value: leftVal / rightVal}
	case "**":
		return &object.Float{Value: (math.Pow(float64(leftVal), float64(rightVal)))}
	case "//":
		if rightVal == 0 {
			return object.NewError(node.Line(), node.Column(), "division by zero: %f // %f", leftVal, rightVal)
		}
		return &object.Float{Value: (math.Floor(float64(leftVal) / float64(rightVal)))}
	case "%":
		if rightVal == 0 {
			return object.NewError(node.Line(), node.Column(), "division by zero: %f %% %f", leftVal, rightVal)
		}
		return &object.Float{Value: (math.Mod(float64(leftVal), float64(rightVal)))}
	case "+=":
		return &object.Float{Value: leftVal + rightVal}
	case "-=":
		return &object.Float{Value: leftVal - rightVal}
	case "*=":
		return &object.Float{Value: leftVal * rightVal}
	case "/=":
		if rightVal == 0 {
			return object.NewError(node.Line(), node.Column(), "division by zero: %f / %f", leftVal, rightVal)
		}
		return &object.Float{Value: leftVal / rightVal}
	case "**=":
		return &object.Float{Value: (math.Pow(float64(leftVal), float64(rightVal)))}
	case "//=":
		if rightVal == 0 {
			return object.NewError(node.Line(), node.Column(), "division by zero: %f // %f", leftVal, rightVal)
		}
		return &object.Float{Value: (math.Floor(float64(leftVal) / float64(rightVal)))}
	case "%=":
		if rightVal == 0 {
			return object.NewError(node.Line(), node.Column(), "division by zero: %f %% %f", leftVal, rightVal)
		}
		return &object.Float{Value: (math.Mod(float64(leftVal), float64(rightVal)))}
	case "<":
		return nativeBoolToBooleanObject(leftVal < rightVal)
	case "<=":
		return nativeBoolToBooleanObject(leftVal <= rightVal)
	case ">":
		return nativeBoolToBooleanObject(leftVal > rightVal)
	case ">=":
		return nativeBoolToBooleanObject(leftVal >= rightVal)
	default:
		return object.NewError(node.Line(), node.Column(), "unknown operator: %s %s %s", left.Type(), node.Operator, right.Type())
	}
}

func evalFloatInfixExpression(node *ast.InfixExpression, left, right object.Object) object.Object {
	leftVal := left.(*object.Float).Value
	rightVal := right.(*object.Float).Value

	switch node.Operator {
	case "+":
		return &object.Float{Value: leftVal + rightVal}
	case "-":
		return &object.Float{Value: leftVal - rightVal}
	case "*":
		return &object.Float{Value: leftVal * rightVal}
	case "/":
		if rightVal == 0 {
			return object.NewError(node.Line(), node.Column(), "division by zero: %f / %f", leftVal, rightVal)
		}
		return &object.Float{Value: leftVal / rightVal}
	case "**":
		return &object.Float{Value: (math.Pow(float64(leftVal), float64(rightVal)))}
	case "//":
		if rightVal == 0 {
			return object.NewError(node.Line(), node.Column(), "division by zero: %f // %f", leftVal, rightVal)
		}
		return &object.Float{Value: (math.Floor(float64(leftVal) / float64(rightVal)))}
	case "%":
		if rightVal == 0 {
			return object.NewError(node.Line(), node.Column(), "division by zero: %f %% %f", leftVal, rightVal)
		}
		return &object.Float{Value: (math.Mod(float64(leftVal), float64(rightVal)))}
	case "+=":
		return &object.Float{Value: leftVal + rightVal}
	case "-=":
		return &object.Float{Value: leftVal - rightVal}
	case "*=":
		return &object.Float{Value: leftVal * rightVal}
	case "/=":
		if rightVal == 0 {
			return object.NewError(node.Line(), node.Column(), "division by zero: %f / %f", leftVal, rightVal)
		}
		return &object.Float{Value: leftVal / rightVal}
	case "**=":
		return &object.Float{Value: (math.Pow(float64(leftVal), float64(rightVal)))}
	case "//=":
		if rightVal == 0 {
			return object.NewError(node.Line(), node.Column(), "division by zero: %f // %f", leftVal, rightVal)
		}
		return &object.Float{Value: (math.Floor(float64(leftVal) / float64(rightVal)))}
	case "%=":
		if rightVal == 0 {
			return object.NewError(node.Line(), node.Column(), "division by zero: %f %% %f", leftVal, rightVal)
		}
		return &object.Float{Value: (math.Mod(float64(leftVal), float64(rightVal)))}
	case "<":
		return nativeBoolToBooleanObject(leftVal < rightVal)
	case ">":
		return nativeBoolToBooleanObject(leftVal > rightVal)
	case "==":
		return nativeBoolToBooleanObject(leftVal == rightVal)
	case "!=":
		return nativeBoolToBooleanObject(leftVal != rightVal)
	case "<=":
		return nativeBoolToBooleanObject(leftVal <= rightVal)
	case ">=":
		return nativeBoolToBooleanObject(leftVal >= rightVal)
	default:
		return object.NewError(node.Line(), node.Column(), "unknown operator: %s %s %s", strings.ToLower(string(left.Type())), node.Operator, strings.ToLower(string(right.Type())))
	}
}

func evalMixedStringOrCharInfixExpression(node *ast.InfixExpression, left, right object.Object) object.Object {
	if node.Operator != "+" {
		return object.NewError(node.Line(), node.Column(), "unknown operator: %s %s %s", left.Type(), node.Operator, right.Type())
	}

	leftVal := left.(*object.String).Value
	rightVal := right.(*object.Char).Value

	return &object.String{Value: leftVal + string(rightVal)}
}

func evalCharInfixExpression(node *ast.InfixExpression, left, right object.Object) object.Object {
	leftVal := left.(*object.Char).Value
	rightVal := right.(*object.Char).Value
	switch node.Operator {
	case "+":
		return &object.String{Value: string(leftVal) + string(rightVal)}
	default:
		return object.NewError(node.Line(), node.Column(), "unknown operator: %s %s %s", left.Type(), node.Operator, right.Type())
	}
}

func evalStringInfixExpression(node *ast.InfixExpression, left, right object.Object) object.Object {
	if node.Operator != "+" {
		return object.NewError(node.Line(), node.Column(), "unknown operator: %s %s %s", left.Type(), node.Operator, right.Type())
	}

	leftVal := left.(*object.String).Value
	rightVal := right.(*object.String).Value

	return &object.String{Value: leftVal + rightVal}
}

func evalIntegerInfixExpression(node *ast.InfixExpression, left, right object.Object) object.Object {
	leftVal := left.(*object.Integer).Value
	rightVal := right.(*object.Integer).Value

	switch node.Operator {
	case "+":
		return &object.Integer{Value: leftVal + rightVal}
	case "-":
		return &object.Integer{Value: leftVal - rightVal}
	case "*":
		return &object.Integer{Value: leftVal * rightVal}
	case "/":
		if rightVal == 0 {
			return object.NewError(node.Line(), node.Column(), "division by zero: %d / %d", leftVal, rightVal)
		}
		return &object.Integer{Value: leftVal / rightVal}
	case "**":
		return &object.Integer{Value: int64(math.Pow(float64(leftVal), float64(rightVal)))}
	case "//":
		if rightVal == 0 {
			return object.NewError(node.Line(), node.Column(), "division by zero: %d // %d", leftVal, rightVal)
		}
		return &object.Integer{Value: int64(math.Floor(float64(leftVal) / float64(rightVal)))}
	case "%":
		if rightVal == 0 {
			return object.NewError(node.Line(), node.Column(), "division by zero: %d %% %d", leftVal, rightVal)
		}
		return &object.Integer{Value: int64(math.Mod(float64(leftVal), float64(rightVal)))}
	case "+=":
		return &object.Integer{Value: leftVal + rightVal}
	case "-=":
		return &object.Integer{Value: leftVal - rightVal}
	case "*=":
		return &object.Integer{Value: leftVal * rightVal}
	case "/=":
		if rightVal == 0 {
			return object.NewError(node.Line(), node.Column(), "division by zero: %d / %d", leftVal, rightVal)
		}
		return &object.Integer{Value: leftVal / rightVal}
	case "**=":
		return &object.Integer{Value: int64(math.Pow(float64(leftVal), float64(rightVal)))}
	case "//=":
		if rightVal == 0 {
			return object.NewError(node.Line(), node.Column(), "division by zero: %d // %d", leftVal, rightVal)
		}
		return &object.Integer{Value: int64(math.Floor(float64(leftVal) / float64(rightVal)))}
	case "%=":
		if rightVal == 0 {
			return object.NewError(node.Line(), node.Column(), "division by zero: %d %% %d", leftVal, rightVal)
		}
		return &object.Integer{Value: int64(math.Mod(float64(leftVal), float64(rightVal)))}
	case "<":
		return nativeBoolToBooleanObject(leftVal < rightVal)
	case ">":
		return nativeBoolToBooleanObject(leftVal > rightVal)
	case "==":
		return nativeBoolToBooleanObject(leftVal == rightVal)
	case "!=":
		return nativeBoolToBooleanObject(leftVal != rightVal)
	case "<=":
		return nativeBoolToBooleanObject(leftVal <= rightVal)
	case ">=":
		return nativeBoolToBooleanObject(leftVal >= rightVal)
	default:
		return object.NewError(node.Line(), node.Column(), "unknown operator: %s %s %s", left.Type(), node.Operator, right.Type())
	}
}

func evalPrefixExpression(node *ast.PrefixExpression, right object.Object) object.Object {
	switch node.Operator {
	case "!":
		return evalBangOperatorExpression(right)
	case "-":
		return evalMinusOperatorExpression(node, right)
	default:
		return object.NewError(node.Line(), node.Column(), "unknown operator: %s%s", node.Operator, right.Type())
	}
}

func (e *Evaluator) evalUpdateExpression(node *ast.UpdateExpression, env *object.Environment) object.Object {
	target := node.Target
	current := e.Eval(target, env)
	if isError(current) {
		return current
	}

	var updated object.Object
	switch v := current.(type) {
	case *object.Integer:
		if node.Operator == "++" {
			updated = &object.Integer{Value: v.Value + 1}
		} else {
			updated = &object.Integer{Value: v.Value - 1}
		}
	case *object.Float:
		if node.Operator == "++" {
			updated = &object.Float{Value: v.Value + 1}
		} else {
			updated = &object.Float{Value: v.Value - 1}
		}
	default:
		return object.NewError(node.Line(), node.Column(), "unsupported operand type for %s: %s", node.Operator, current.Type())
	}

	// Update target in environment / member / index.
	switch t := target.(type) {
	case *ast.Identifier:
		if dist, ok := e.Resolutions[t]; ok {
			if !env.UpdateAt(dist, t.Value, updated) {
				return object.NewError(node.Line(), node.Column(), "cannot reassign to const: %s", t.Value)
			}
		} else {
			_, updatedOk := env.Update(t.Value, updated)
			if !updatedOk {
				env.Set(t.Value, updated)
			}
		}
	case *ast.MemberExpression:
		obj := e.Eval(t.Object, env)
		if isError(obj) {
			return obj
		}
		if res := evalAssignMember(obj, t, updated); isError(res) {
			return res
		}
	case *ast.IndexExpression:
		obj := e.Eval(t.Left, env)
		if isError(obj) {
			return obj
		}
		idx := e.Eval(t.Index, env)
		if isError(idx) {
			return idx
		}
		if res := evalAssignIndex(obj, idx, updated, t); isError(res) {
			return res
		}
	default:
		return object.NewError(node.Line(), node.Column(), "invalid left-hand side in update")
	}

	if node.Prefix {
		return updated
	}
	return current
}

func evalMinusOperatorExpression(node *ast.PrefixExpression, right object.Object) object.Object {
	if right.Type() != object.INTEGER_OBJ {
		return object.NewError(node.Line(), node.Column(), "unknown operator: -%s", right.Type())
	}

	value := right.(*object.Integer).Value
	return &object.Integer{Value: -value}
}

func evalBangOperatorExpression(right object.Object) object.Object {
	switch right {
	case object.TRUE:
		return object.FALSE
	case object.FALSE:
		return object.TRUE
	case object.NULL:
		return object.TRUE
	default:
		return object.FALSE
	}
}

func nativeBoolToBooleanObject(input bool) *object.Boolean {
	if input {
		return object.TRUE
	}
	return object.FALSE
}

func isError(obj object.Object) bool {
	if obj != nil {
		return obj.Type() == object.ERROR_OBJ
	}
	return false
}

func isStringOrChar(obj object.Object) bool {
	return obj.Type() == object.STRING_OBJ || obj.Type() == object.CHAR_OBJ
}
