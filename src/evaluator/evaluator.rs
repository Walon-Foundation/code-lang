use std::{cell::RefCell, collections::{HashMap, HashSet}, rc::Rc};

use crate::{
    ast::ast::{Expression, LetPattern, Program, Statement, StringSegment},
    lexer::lexer::Lexer,
    object::object::{CallInfo, Environment, Evaluable, Object},
    parser::parser::Parser,
    token::token::{Token, TokenType},
};

const MAX_CALL_DEPTH: usize = 500;

pub struct Evaluator {
    pub loop_depth: usize,
    pub call_depth: usize,
    pub call_stack: Vec<CallFrame>,
    pub module_cache: HashMap<String, Object>,
}

pub struct CallFrame {
    pub name:String,
    pub call_line: usize,
    pub call_column: usize
}

type Env = Rc<RefCell<Environment>>;

impl Evaluator {
    pub fn new() -> Self {
        let mut e = Evaluator {
            loop_depth: 0,
            call_depth: 0,
            call_stack: Vec::new(),
            module_cache: HashMap::new(),
        };
        e.preload_stdlib();
        e
    }
}

impl Default for Evaluator {
    fn default() -> Self {
        Self::new()
    }
}

impl Evaluator {
    fn preload_stdlib(&mut self) {
        self.module_cache
            .insert("arrays".to_string(), crate::std_lib::array::module());
        self.module_cache
            .insert("strings".to_string(), crate::std_lib::strings::module());
        self.module_cache
            .insert("math".to_string(), crate::std_lib::math::module());
        self.module_cache
            .insert("fs".to_string(), crate::std_lib::fs::module());
        self.module_cache
            .insert("hash".to_string(), crate::std_lib::hash::module());
        self.module_cache
            .insert("fmt".to_string(), crate::std_lib::fmt::module());
        self.module_cache
            .insert("os".to_string(), crate::std_lib::os_mod::module());
        self.module_cache
            .insert("time".to_string(), crate::std_lib::time::module());
        self.module_cache
            .insert("json".to_string(), crate::std_lib::json::module());
        self.module_cache
            .insert("rand".to_string(), crate::std_lib::rand::module());
        self.module_cache
            .insert("path".to_string(), crate::std_lib::path::module());
        self.module_cache
            .insert("http".to_string(), crate::std_lib::http::module());
    }
    pub fn eval(&mut self, node: &Program, env: &Env) -> Object {
        let mut result = Object::Null;
        for stmt in &node.statements {
            result = self.eval_statement(stmt, env);
            match result {
                Object::Return(value) => return *value,
                Object::Error { .. } => return result,
                _ => {}
            }
        }
        result
    }

    fn eval_statement(&mut self, stmt: &Statement, env: &Env) -> Object {
        match stmt {
            Statement::Block { statements, .. } => {
                let block_env = Environment::new_enclosed(Rc::clone(env));
                let mut result = Object::Null;
                for s in statements {
                    result = self.eval_statement(s, &block_env);
                    if matches!(
                        result,
                        Object::Return(_) | Object::Error { .. } | Object::Break | Object::Continue
                    ) {
                        return result;
                    }
                }
                result
            }

            Statement::Pub { statement, .. } => {
                // eval the inner let/const — this sets the name in env normally
                let result = self.eval_statement(statement, env);
                if matches!(result, Object::Error { .. }) {
                    return result;
                }

                // extract the name from the inner statement and mark it as pubed
                let name = match statement.as_ref() {
                    Statement::Let {
                        pattern: LetPattern::Ident(v),
                        ..
                    } => v,
                    Statement::Const {
                        pattern: LetPattern::Ident(v),
                        ..
                    } => v,
                    _ => return result,
                };
                env.borrow_mut().mark_pub(name);
                Object::Null
            }

            Statement::Let {
                pattern,
                value,
                line,
                column,
                ..
            } => {
                let val = self.eval_expression(value, env);
                // errors are stored as values so callers can use is_error()
                match pattern {
                    LetPattern::Ident(n) => {
                        env.borrow_mut().set(n.to_string(), val);
                    }

                    LetPattern::Array(names) => {
                        let elems = match val {
                            Object::Array(e) => e,
                            _ => {
                                return Object::Error {
                                    message: "array destructing requires an array".to_string(),
                                    line: *line,
                                    column: *column,
                                };
                            }
                        };

                        for (i, name) in names.iter().enumerate() {
                            if name == "_" {
                                continue;
                            }
                            let v = elems.get(i).cloned().unwrap_or(Object::Null);
                            env.borrow_mut().set(name.to_string(), v);
                        }
                    }

                    LetPattern::Hash(pairs) => {
                        for (key, alias) in pairs {
                            let v = match &val {
                                Object::Hash(kv) => kv
                                    .iter()
                                    .find(|(k, _)| matches!(k, Object::StringType(s) if s == key))
                                    .map(|(_, v)| v.clone())
                                    .unwrap_or(Object::Null),
                                Object::StructInstance { fields, .. } => {
                                    fields.get(key).cloned().unwrap_or(Object::Null)
                                }
                                _ => {
                                    return Object::Error {
                                        message: "hash destructing required a hash or struct"
                                            .to_string(),
                                        line: *line,
                                        column: *column,
                                    };
                                }
                            };
                            env.borrow_mut().set(alias.to_string(), v)
                        }
                    }
                }

                Object::Null
            }

            Statement::Const {
                pattern,
                value,
                line,
                column,
                ..
            } => {
                let val = self.eval_expression(value, env);
                // errors are stored as values so callers can use is_error()
                match pattern {
                    LetPattern::Ident(n) => {
                        env.borrow_mut().set_const(n.to_string(), val);
                    }

                    LetPattern::Array(names) => {
                        let elems = match val {
                            Object::Array(e) => e,
                            _ => {
                                return Object::Error {
                                    message: "array destructing requires an array".to_string(),
                                    line: *line,
                                    column: *column,
                                };
                            }
                        };

                        for (i, name) in names.iter().enumerate() {
                            if name == "_" {
                                continue;
                            }
                            let v = elems.get(i).cloned().unwrap_or(Object::Null);
                            env.borrow_mut().set_const(name.to_string(), v);
                        }
                    }

                    LetPattern::Hash(pairs) => {
                        for (key, alias) in pairs {
                            let v = match &val {
                                Object::Hash(kv) => kv
                                    .iter()
                                    .find(|(k, _)| matches!(k, Object::StringType(s) if s == key))
                                    .map(|(_, v)| v.clone())
                                    .unwrap_or(Object::Null),
                                Object::StructInstance { fields, .. } => {
                                    fields.get(key).cloned().unwrap_or(Object::Null)
                                }
                                _ => {
                                    return Object::Error {
                                        message: "hash destructing required a hash or struct"
                                            .to_string(),
                                        line: *line,
                                        column: *column,
                                    };
                                }
                            };
                            env.borrow_mut().set_const(alias.to_string(), v)
                        }
                    }
                }

                Object::Null
            }

            Statement::Return { value, .. } => {
                let val = self.eval_expression(value, env);
                if matches!(val, Object::Error { .. }) {
                    return val;
                }
                Object::Return(Box::new(val))
            }

            Statement::Expression { expr, .. } => {
                let val = self.eval_expression(expr, env);
                if matches!(val, Object::Error { .. }) {
                    return val;
                }
                val
            }

            Statement::Struct { name, field, .. } => {
                let mut defaults: HashMap<String, Box<Object>> = HashMap::new();
                for (key, expr) in field {
                    let val = self.eval_expression(expr, env);
                    if matches!(val, Object::Error { .. }) {
                        return val;
                    }
                    defaults.insert(key.clone(), Box::new(val));
                }
                let struct_name = match name.as_ref() {
                    Expression::Ident { value, .. } => value.clone(),
                    _ => {
                        return Object::Error {
                            message: "struct name must be an identifier".to_string(),
                            line: 0,
                            column: 0,
                        };
                    }
                };
                let obj = Object::StructType {
                    name: struct_name.clone(),
                    default: defaults,
                };
                env.borrow_mut().set(struct_name, obj);
                Object::Null
            }

            Statement::Enum { name, variant, .. } => {
                let obj = Object::EnumType {
                    name: name.clone(),
                    variants: variant.clone(),
                };
                env.borrow_mut().set(name.to_string(), obj);
                Object::Null
            }

            Statement::Import {
                path, line, column, ..
            } => self.eval_import_statement(path, *line, *column, env),

            Statement::Break { line, column, .. } => {
                if self.loop_depth == 0 {
                    return Object::Error {
                        message: "break outside of loop".to_string(),
                        line: *line,
                        column: *column,
                    };
                }
                Object::Break
            }

            Statement::Continue { line, column, .. } => {
                if self.loop_depth == 0 {
                    return Object::Error {
                        message: "continue outside of loop".to_string(),
                        line: *line,
                        column: *column,
                    };
                }
                Object::Continue
            }
        }
    }

    fn eval_expression(&mut self, expr: &Expression, env: &Env) -> Object {
        match expr {
            Expression::Int { value, .. } => Object::Integer(*value),
            Expression::Float { value, .. } => Object::Float(*value),
            Expression::InterpolatedString { parts, .. } => {
                let mut result = String::new();
                for part in parts {
                    match part {
                        StringSegment::Literal(s) => result.push_str(s),
                        StringSegment::Expr(expr) => {
                            let val = self.eval_expression(expr, env);
                            if matches!(val, Object::Error { .. }) {
                                return val;
                            }
                            result.push_str(&val.to_string());
                        }
                    }
                }

                Object::StringType(result)
            }
            Expression::Null { .. } => Object::Null,

            Expression::NullCoalesce { left, right, .. } => {
                let l = self.eval_expression(left, env);
                if matches!(l, Object::Null) {
                    self.eval_expression(right, env)
                } else {
                    l
                }
            }
            Expression::Typeof { value, .. } => {
                let object = self.eval_expression(value, env);
                Object::StringType(object.type_name().to_lowercase())
            }

            Expression::Char { value, .. } => Object::Char(*value),
            Expression::Boolean { value, .. } => Object::Bool(*value),

            Expression::Ident {
                value,
                line,
                column,
                ..
            } => match env.borrow().get(value) {
                Some(obj) => obj,
                None => Object::Error {
                    message: format!("identifier not found: {}", value),
                    line: *line,
                    column: *column,
                },
            },

            Expression::Prefix {
                op,
                right,
                line,
                column,
                ..
            } => {
                let right_val = self.eval_expression(right, env);
                if matches!(right_val, Object::Error { .. }) {
                    return right_val;
                }
                self.eval_prefix(op, right_val, *line, *column)
            }

            Expression::Infix {
                left,
                op,
                right,
                line,
                column,
                ..
            } => {
                if self.is_assignment(&op.token_type) {
                    return self.eval_assignment(left, op, right, env, *line, *column);
                }
                if op.token_type == TokenType::And {
                    let l = self.eval_expression(left, env);
                    if matches!(l, Object::Error { .. }) {
                        return l;
                    }
                    if !self.is_truthy(&l) {
                        return l;
                    }
                    return self.eval_expression(right, env);
                }
                if op.token_type == TokenType::Or {
                    let l = self.eval_expression(left, env);
                    if matches!(l, Object::Error { .. }) {
                        return l;
                    }
                    if self.is_truthy(&l) {
                        return l;
                    }
                    return self.eval_expression(right, env);
                }
                let l = self.eval_expression(left, env);
                if matches!(l, Object::Error { .. }) {
                    return l;
                }
                let r = self.eval_expression(right, env);
                if matches!(r, Object::Error { .. }) {
                    return r;
                }
                self.eval_infix(op, l, r, *line, *column)
            }

            Expression::If {
                condition,
                consequence,
                alternative,
                if_else,
                ..
            } => {
                let cond = self.eval_expression(condition, env);
                if matches!(cond, Object::Error { .. }) {
                    return cond;
                }
                if self.is_truthy(&cond) {
                    return self.eval_statement(consequence, env);
                }
                for branch in if_else {
                    let c = self.eval_expression(&branch.condition, env);
                    if matches!(c, Object::Error { .. }) {
                        return c;
                    }
                    if self.is_truthy(&c) {
                        return self.eval_statement(&branch.consequences, env);
                    }
                }
                if let Some(alt) = alternative {
                    return self.eval_statement(alt, env);
                }
                Object::Null
            }

            Expression::Function {
                parameter, body, ..
            } => Object::Function {
                parameters: parameter.clone(),
                body: body.clone(),
                env: Rc::clone(env),
            },

            Expression::Call {
                function,
                argument,
                line,
                column,
                ..
            } => {
                //call for the struct method
                if let Expression::Member {
                    object, property, ..
                } = function.as_ref()
                {
                    let reciever = self.eval_expression(object, env);
                    if matches!(reciever, Object::Error { .. }) {
                        return reciever;
                    }

                    let prop_name = match property.as_ref() {
                        Expression::Ident { value, .. } => value.clone(),
                        _ => {
                            return Object::Error {
                                message: "invalid member expression".to_string(),
                                line: *line,
                                column: *column,
                            };
                        }
                    };

                    let method = self.eval_member_on_obj(&reciever, &prop_name, *line, *column);
                    if matches!(method, Object::Error { .. }) {
                        return method;
                    }

                    if let Object::Function { parameters, .. } = method.clone()
                        && parameters
                            .first()
                            .map(|p| p.name == "self")
                            .unwrap_or(false)
                    {
                        let mut args = self.eval_args(argument, env);
                        if args
                            .first()
                            .map(|a| matches!(a, Object::Error { .. }))
                            .unwrap_or(false)
                        {
                            return args.into_iter().next().unwrap();
                        }
                        args.insert(0, reciever);
                        return self.apply_function(method.clone(), args, *line, *column);
                    }

                    //not a self-method
                    let args = self.eval_args(argument, env);
                    if args
                        .first()
                        .map(|a| matches!(a, Object::Error { .. }))
                        .unwrap_or(false)
                    {
                        return args.into_iter().next().unwrap();
                    }
                    return self.apply_function(method, args, *line, *column);
                }

                let func = self.eval_expression(function, env);
                if matches!(func, Object::Error { .. }) {
                    return func;
                }
                let args = self.eval_args(argument, env);
                self.apply_function(func, args, *line, *column)
            }

            Expression::Array { element, .. } => {
                let elems = self.eval_args(element, env);
                if elems.len() == 1 && matches!(elems[0], Object::Error { .. }) {
                    return elems.into_iter().next().unwrap_or(Object::Null);
                }
                Object::Array(elems)
            }

            Expression::Index {
                left,
                index,
                line,
                column,
                ..
            } => {
                let left_val = self.eval_expression(left, env);
                if matches!(left_val, Object::Error { .. }) {
                    return left_val;
                }
                let idx = self.eval_expression(index, env);
                if matches!(idx, Object::Error { .. }) {
                    return idx;
                }
                self.eval_index(left_val, idx, *line, *column)
            }

            Expression::HashLiteral { pair, .. } => {
                let mut pairs = Vec::new();
                for (k_expr, v_expr) in pair {
                    let k = self.eval_expression(k_expr, env);
                    if matches!(k, Object::Error { .. }) {
                        return k;
                    }
                    let v = self.eval_expression(v_expr, env);
                    if matches!(v, Object::Error { .. }) {
                        return v;
                    }
                    pairs.push((k, v));
                }
                Object::Hash(pairs)
            }

            Expression::While {
                condition, body, ..
            } => {
                self.loop_depth += 1;
                let mut result = Object::Null;
                loop {
                    let cond = self.eval_expression(condition, env);
                    if matches!(cond, Object::Error { .. }) {
                        self.loop_depth -= 1;
                        return cond;
                    }
                    if !self.is_truthy(&cond) {
                        break;
                    }
                    result = self.eval_statement(body, env);
                    match result {
                        Object::Break => {
                            result = Object::Null;
                            break;
                        }
                        Object::Continue => continue,
                        Object::Return(_) | Object::Error { .. } => {
                            self.loop_depth -= 1;
                            return result;
                        }
                        _ => {}
                    }
                }
                self.loop_depth -= 1;
                result
            }

            Expression::For {
                init,
                condition,
                post,
                body,
                ..
            } => {
                let for_env = Environment::new_enclosed(Rc::clone(env));
                let init_res = self.eval_statement(init, &for_env);
                if matches!(init_res, Object::Error { .. }) {
                    return init_res;
                }
                self.loop_depth += 1;
                let mut result = Object::Null;
                loop {
                    let cond = self.eval_expression(condition, &for_env);
                    if matches!(cond, Object::Error { .. }) {
                        self.loop_depth -= 1;
                        return cond;
                    }
                    if !self.is_truthy(&cond) {
                        break;
                    }
                    result = self.eval_statement(body, &for_env);
                    match result {
                        Object::Break => {
                            result = Object::Null;
                            break;
                        }
                        Object::Continue => {}
                        Object::Return(_) | Object::Error { .. } => {
                            self.loop_depth -= 1;
                            return result;
                        }
                        _ => {}
                    }
                    let post_res = self.eval_statement(post, &for_env);
                    if matches!(post_res, Object::Error { .. }) {
                        self.loop_depth -= 1;
                        return post_res;
                    }
                }
                self.loop_depth -= 1;
                result
            }

            Expression::ForIn {
                key,
                value,
                iterable,
                body,
                line,
                column,
                ..
            } => {
                let iterable = self.eval_expression(iterable, env);
                self.loop_depth += 1;

                match iterable {
                    Object::Array(elems) => {
                        for elem in elems {
                            let loop_env = Environment::new_enclosed(Rc::clone(env));
                            loop_env.borrow_mut().set(key.to_string(), elem);
                            let result = self.eval_statement(body, &loop_env);
                            match result {
                                Object::Break => break,
                                Object::Continue => continue,
                                Object::Error { .. } | Object::Return(_) => return result,
                                _ => {}
                            }
                        }
                    }

                    Object::Hash(pairs) => {
                        let some_value =
                            match value {
                                None => return Object::Error {
                                    message:
                                        "hash iteration requires two variables: for k, v in hash"
                                            .to_string(),
                                    line: *line,
                                    column: *column,
                                },
                                Some(v) => v,
                            };
                        for (k, v) in pairs {
                            let loop_env = Environment::new_enclosed(Rc::clone(env));
                            loop_env.borrow_mut().set(key.to_string(), k);
                            loop_env.borrow_mut().set(some_value.clone(), v);
                            let result = self.eval_statement(body, &loop_env);
                            match result {
                                Object::Break => break,
                                Object::Continue => continue,
                                Object::Error { .. } | Object::Return(_) => return result,
                                _ => {}
                            }
                        }
                    }

                    Object::StringType(s) => {
                        for ch in s.chars() {
                            let loop_env = Environment::new_enclosed(Rc::clone(env));
                            loop_env.borrow_mut().set(key.to_string(), Object::Char(ch));
                            let result = self.eval_statement(body, &loop_env);
                            match result {
                                Object::Break => break,
                                Object::Continue => continue,
                                Object::Error { .. } | Object::Return(_) => return result,
                                _ => {}
                            }
                        }
                    }

                    _ => {
                        return Object::Error {
                            message: format!("cannot iterate over {}", iterable.type_name()),
                            line: *line,
                            column: *column,
                        };
                    }
                }

                Object::Null
            }

            Expression::Switch { subject, arms, default, .. } => {
                let subject_val = self.eval_expression(subject, env);
                if matches!(subject_val, Object::Error { .. }) {
                    return subject_val;
                }

                for arm in arms {
                    let pat_val = self.eval_expression(&arm.pattern, env);
                    if matches!(pat_val, Object::Error { .. }) {
                        return pat_val;
                    }
                    if self.objects_equal(&subject_val, &pat_val) {
                        return self.eval_statement(&arm.body, env);
                    }
                }

                if let Some(arm) = default {
                    return self.eval_statement(arm, env)
                }

                Object::Null
            }

            Expression::Update {
                operator,
                target,
                prefix,
                line,
                column,
                ..
            } => {
                let current = self.eval_expression(target, env);
                if matches!(current, Object::Error { .. }) {
                    return current;
                }
                let updated = match &current {
                    Object::Integer(v) => {
                        if matches!(operator.token_type, TokenType::Inc) {
                            v.checked_add(1).map(Object::Integer).unwrap_or_else(||Object::Error { 
                                message: format!("integer overflow: {} + 1", v), 
                                line: *line, 
                                column: *column 
                            })
                        } else {
                            v.checked_sub(1).map(Object::Integer).unwrap_or_else(||Object::Error{
                                message:format!("integer overflow: {} - 1", v),
                                line: *line,
                                column: *column
                            })
                        }
                    }
                    Object::Float(v) => {
                        if matches!(operator.token_type, TokenType::Inc) {
                            Object::Float(v + 1.0)
                        } else {
                            Object::Float(v - 1.0)
                        }
                    }
                    _ => {
                        return Object::Error {
                            message: format!(
                                "unsupported operand for update: {}",
                                current.type_name()
                            ),
                            line: *line,
                            column: *column,
                        };
                    }
                };
                match target.as_ref() {
                    Expression::Ident { value: name, .. } => {
                        if env.borrow().get(name).is_none() {
                            return Object::Error { 
                                message: format!("cannot update undeclared variable '{}'", name), 
                                line: *line, 
                                column: *column 
                            }
                        }

                        env.borrow_mut().set(name.clone(), updated.clone())
                    }
                    Expression::Member {
                        object: obj_expr,
                        property,
                        ..
                    } => {
                        if let (
                            Expression::Ident { value: root, .. },
                            Expression::Ident { value: field, .. },
                        ) = (obj_expr.as_ref(), property.as_ref())
                            && let Some(mut obj) = env.borrow().get(root)
                        {
                            if let Object::StructInstance { fields, .. } = &mut obj {
                                fields.insert(field.clone(), updated.clone());
                            }
                            env.borrow_mut().update(root, obj);
                        }
                    }
                    Expression::Index {
                        left: obj_expr,
                        index: idx_expr,
                        ..
                    } => {
                        if let Expression::Ident { value: root, .. } = obj_expr.as_ref() {
                            let idx = self.eval_expression(idx_expr, env);
                            if let Some(mut obj) = env.borrow().get(root) {
                                if let Object::Array(elements) = &mut obj
                                    && let Object::Integer(i) = &idx
                                {
                                    if *i < 0 || *i as usize >= elements.len() {
                                        return Object::Error {
                                            message: format!(
                                                "index {} out of range (len {})",
                                                i,
                                                elements.len()
                                            ),
                                            line: *line,
                                            column: *column,
                                        };
                                    }
                                    elements[*i as usize] = updated.clone();
                                }
                                env.borrow_mut().update(root, obj);
                            }
                        }
                    }
                    _ => {}
                }
                if *prefix { updated } else { current }
            }

            Expression::Member {
                object,
                property,
                line,
                column,
                ..
            } => {
                let obj = self.eval_expression(object, env);
                if matches!(obj, Object::Error { .. }) {
                    return obj;
                }
                let prop = match property.as_ref() {
                    Expression::Ident { value, .. } => value.clone(),
                    _ => {
                        return Object::Error {
                            message: "invalid property".to_string(),
                            line: *line,
                            column: *column,
                        };
                    }
                };
                self.eval_member_on_obj(&obj, &prop, *line, *column)
            }

            Expression::StructLiteral {
                name,
                fields,
                line,
                column,
                ..
            } => {
                let obj = match env.borrow().get(name) {
                    Some(o) => o,
                    None => {
                        return Object::Error {
                            message: format!("unknown struct: {}", name),
                            line: *line,
                            column: *column,
                        };
                    }
                };
                let mut instance_fields: HashMap<String, Object> = match obj {
                    Object::StructType { ref default, .. } => default
                        .iter()
                        .map(|(k, v)| (k.clone(), *v.clone()))
                        .collect(),
                    _ => {
                        return Object::Error {
                            message: format!("{} is not a struct", name),
                            line: *line,
                            column: *column,
                        };
                    }
                };

                let valid_filed:HashSet<&str> = instance_fields.keys().map(|s| s.as_str()).collect();
                for (k, v_expr) in fields {
                    if !valid_filed.contains(k.as_str()) {
                        return Object::Error{
                            message: format!("unknown field '{}' on struct '{}'", k , name),
                            line: *line,
                            column: *column
                        }
                    }
                    let val = self.eval_expression(v_expr, env);
                    if matches!(val, Object::Error { .. }) {
                        return val;
                    }
                    instance_fields.insert(k.clone(), val);
                }
                Object::StructInstance {
                    type_name: name.clone(),
                    fields: instance_fields,
                }
            }
        }
    }

    pub fn register_globals(&self, env: &Env) {
        env.borrow_mut()
            .set("is_error".to_string(), Object::Builtin(builtin_is_error));
    }

    fn objects_equal(&self, a: &Object, b: &Object) -> bool {
        match (a, b) {
            (Object::Integer(x), Object::Integer(y)) => x == y,
            (Object::Float(x), Object::Float(y)) => x == y,
            (Object::StringType(x), Object::StringType(y)) => x == y,
            (Object::Bool(x), Object::Bool(y)) => x == y,
            (Object::Char(x), Object::Char(y)) => x == y,
            (Object::Null, Object::Null) => true,
            (
                Object::EnumVariant {
                    enum_name: n1,
                    variant: v1,
                },
                Object::EnumVariant {
                    enum_name: n2,
                    variant: v2,
                },
            ) => n1 == n2 && v1 == v2,
            _ => false,
        }
    }

    fn eval_prefix(&self, op: &Token, right: Object, line: usize, column: usize) -> Object {
        match &op.token_type {
            TokenType::Bang => match right {
                Object::Bool(true) => Object::Bool(false),
                Object::Bool(false) => Object::Bool(true),
                Object::Null => Object::Bool(true),
                _ => Object::Bool(false),
            },
            TokenType::Minus => match right {
                Object::Integer(v) => Object::Integer(-v),
                Object::Float(v) => Object::Float(-v),
                _ => Object::Error {
                    message: format!("unknown operator: -{}", right.type_name()),
                    line,
                    column,
                },
            },
            _ => Object::Error {
                message: "unknown prefix operator".to_string(),
                line,
                column,
            },
        }
    }

    fn eval_infix(
        &self,
        op: &Token,
        left: Object,
        right: Object,
        line: usize,
        column: usize,
    ) -> Object {
        match (&left, &right) {
            (Object::Integer(l), Object::Integer(r)) => {
                self.eval_integer_infix(&op.token_type, *l, *r, line, column)
            }
            (Object::Float(l), Object::Float(r)) => {
                self.eval_float_infix(&op.token_type, *l, *r, line, column)
            }
            (Object::Integer(l), Object::Float(r)) => {
                self.eval_float_infix(&op.token_type, *l as f64, *r, line, column)
            }
            (Object::Float(l), Object::Integer(r)) => {
                self.eval_float_infix(&op.token_type, *l, *r as f64, line, column)
            }
            (Object::StringType(l), Object::StringType(r)) => match &op.token_type {
                TokenType::Plus | TokenType::AddAssign => Object::StringType(l.clone() + r),
                TokenType::EQ => Object::Bool(l == r),
                TokenType::NOTEQ => Object::Bool(l != r),
                _ => Object::Error {
                    message: format!(
                        "unknown operator: STRING {} STRING",
                        self.op_str(&op.token_type)
                    ),
                    line,
                    column,
                },
            },
            (Object::StringType(l), Object::Char(r)) => match &op.token_type {
                TokenType::Plus => Object::StringType(format!("{}{}", l, r)),
                _ => Object::Error {
                    message: format!(
                        "unknown operator: STRING {} CHAR",
                        self.op_str(&op.token_type)
                    ),
                    line,
                    column,
                },
            },
            (Object::Char(l), Object::Char(r)) => match &op.token_type {
                TokenType::Plus => Object::StringType(format!("{}{}", l, r)),
                TokenType::EQ => Object::Bool(l == r),
                TokenType::NOTEQ => Object::Bool(l != r),
                _ => Object::Error {
                    message: format!(
                        "unknown operator: CHAR {} CHAR",
                        self.op_str(&op.token_type)
                    ),
                    line,
                    column,
                },
            },
            (Object::Bool(l), Object::Bool(r)) => match &op.token_type {
                TokenType::EQ => Object::Bool(l == r),
                TokenType::NOTEQ => Object::Bool(l != r),
                _ => Object::Error {
                    message: format!(
                        "unknown operator: BOOL {} BOOL",
                        self.op_str(&op.token_type)
                    ),
                    line,
                    column,
                },
            },
            _ => {
                if left.type_name() != right.type_name() {
                    Object::Error {
                        message: format!(
                            "type mismatch: {} {} {}",
                            left.type_name(),
                            self.op_str(&op.token_type),
                            right.type_name()
                        ),
                        line,
                        column,
                    }
                } else {
                    Object::Error {
                        message: format!(
                            "unknown operator: {} {} {}",
                            left.type_name(),
                            self.op_str(&op.token_type),
                            right.type_name()
                        ),
                        line,
                        column,
                    }
                }
            }
        }
    }

    fn eval_integer_infix(
        &self,
        op: &TokenType,
        l: isize,
        r: isize,
        line: usize,
        column: usize,
    ) -> Object {
        match op {
            TokenType::Plus | TokenType::AddAssign => l
                .checked_add(r)
                .map(Object::Integer)
                .unwrap_or_else(|| Object::Error {
                    message: format!("integer overflow: {} + {}", l, r),
                    line,
                    column,
                }),
            TokenType::Minus | TokenType::SubAssign => l
                .checked_sub(r)
                .map(Object::Integer)
                .unwrap_or_else(|| Object::Error {
                    message: format!("integer overflow: {} - {}", l, r),
                    line,
                    column,
                }),
            TokenType::Asterisk | TokenType::MulAssign => l
                .checked_mul(r)
                .map(Object::Integer)
                .unwrap_or_else(|| Object::Error {
                    message: format!("integer overflow: {} * {}", l, r),
                    line,
                    column,
                }),
            TokenType::SLASH | TokenType::QuoAssign => {
                if r == 0 {
                    return Object::Error {
                        message: "division by zero".to_string(),
                        line,
                        column,
                    };
                }
                Object::Integer(l / r)
            }
            TokenType::Rem | TokenType::RemAssign => {
                if r == 0 {
                    return Object::Error {
                        message: "division by zero".to_string(),
                        line,
                        column,
                    };
                }
                Object::Integer(l % r)
            }
            TokenType::Square => {
                let result = (l as f64).powf(r as f64);
                if result > isize::MAX as f64 || result < isize::MIN as f64 {
                    return Object::Error {
                        message: format!("integer overflow: {} ** {}", l, r),
                        line,
                        column,
                    };
                }
                Object::Integer(result as isize)
            }
            TokenType::Floor => {
                if r == 0 {
                    return Object::Error {
                        message: "division by zero".to_string(),
                        line,
                        column,
                    };
                }
                Object::Integer(((l as f64) / (r as f64)).floor() as isize)
            }
            TokenType::LT => Object::Bool(l < r),
            TokenType::GT => Object::Bool(l > r),
            TokenType::LessThanEqual => Object::Bool(l <= r),
            TokenType::GreaterThanEqual => Object::Bool(l >= r),
            TokenType::EQ => Object::Bool(l == r),
            TokenType::NOTEQ => Object::Bool(l != r),
            _ => Object::Error {
                message: format!("unknown operator: INTEGER {} INTEGER", self.op_str(op)),
                line,
                column,
            },
        }
    }

    fn float_guard(v: f64, line: usize, column: usize) -> Object {
        if v.is_nan() {
            Object::Error {
                message: "floating-point operation produced NaN".to_string(),
                line,
                column,
            }
        } else if v.is_infinite() {
            Object::Error {
                message: "floating-point operation produced Infinity".to_string(),
                line,
                column,
            }
        } else {
            Object::Float(v)
        }
    }

    fn eval_float_infix(
        &self,
        op: &TokenType,
        l: f64,
        r: f64,
        line: usize,
        column: usize,
    ) -> Object {
        match op {
            TokenType::Plus | TokenType::AddAssign => Self::float_guard(l + r, line, column),
            TokenType::Minus | TokenType::SubAssign => Self::float_guard(l - r, line, column),
            TokenType::Asterisk | TokenType::MulAssign => Self::float_guard(l * r, line, column),
            TokenType::SLASH | TokenType::QuoAssign => {
                if r == 0.0 {
                    return Object::Error {
                        message: "division by zero".to_string(),
                        line,
                        column,
                    };
                }
                Self::float_guard(l / r, line, column)
            }
            TokenType::Rem | TokenType::RemAssign => {
                if r == 0.0 {
                    return Object::Error {
                        message: "division by zero".to_string(),
                        line,
                        column,
                    };
                }
                Self::float_guard(l % r, line, column)
            }
            TokenType::Square => Self::float_guard(l.powf(r), line, column),
            TokenType::Floor => {
                if r == 0.0 {
                    return Object::Error {
                        message: "division by zero".to_string(),
                        line,
                        column,
                    };
                }
                Self::float_guard((l / r).floor(), line, column)
            }
            TokenType::LT => Object::Bool(l < r),
            TokenType::GT => Object::Bool(l > r),
            TokenType::LessThanEqual => Object::Bool(l <= r),
            TokenType::GreaterThanEqual => Object::Bool(l >= r),
            TokenType::EQ => Object::Bool(l == r),
            TokenType::NOTEQ => Object::Bool(l != r),
            _ => Object::Error {
                message: format!("unknown operator: FLOAT {} FLOAT", self.op_str(op)),
                line,
                column,
            },
        }
    }

    fn eval_assignment(
        &mut self,
        left: &Expression,
        op: &Token,
        right: &Expression,
        env: &Env,
        line: usize,
        column: usize,
    ) -> Object {
        let val = self.eval_expression(right, env);
        if matches!(val, Object::Error { .. }) {
            return val;
        }

        match left {
            Expression::Ident { value: name, .. } => {
                let final_val = if op.token_type == TokenType::Assign {
                    val
                } else {
                    let current = match env.borrow().get(name) {
                        Some(v) => v,
                        None => {
                            return Object::Error {
                                message: format!("identifier not found: {}", name),
                                line,
                                column,
                            };
                        }
                    };
                    self.eval_infix(op, current, val, line, column)
                };
                if matches!(final_val, Object::Error { .. }) {
                    return final_val;
                }
                if !env.borrow_mut().update(name, final_val.clone()) {
                    // env.borrow_mut().set(name.clone(), final_val.clone());
                    return Object::Error { 
                        message: format!("cannot assign to an undeclared variable '{}'; use 'let {} = ....' to declared it first",
                            name, name), 
                        line, column 
                    }
                }
                final_val
            }

            Expression::Member {
                object: obj_expr,
                property,
                ..
            } => {
                let prop = match property.as_ref() {
                    Expression::Ident { value, .. } => value.clone(),
                    _ => {
                        return Object::Error {
                            message: "invalid member property".to_string(),
                            line,
                            column,
                        };
                    }
                };
                let root = match obj_expr.as_ref() {
                    Expression::Ident { value, .. } => value.clone(),
                    _ => {
                        return Object::Error {
                            message: "complex member assignment not supported".to_string(),
                            line,
                            column,
                        };
                    }
                };
                let mut container = match env.borrow().get(&root) {
                    Some(o) => o,
                    None => {
                        return Object::Error {
                            message: format!("identifier not found: {}", root),
                            line,
                            column,
                        };
                    }
                };
                let final_val = if op.token_type == TokenType::Assign {
                    val
                } else {
                    let current = self.eval_member_on_obj(&container, &prop, line, column);
                    if matches!(current, Object::Error { .. }) {
                        return current;
                    }
                    self.eval_infix(op, current, val, line, column)
                };
                if matches!(final_val, Object::Error { .. }) {
                    return final_val;
                }
                match &mut container {
                    Object::StructInstance { fields, .. } => {
                        fields.insert(prop, final_val.clone());
                    }
                    Object::Hash(pairs) => {
                        let key = Object::StringType(prop.clone());
                        let mut found = false;
                        for (k, v) in pairs.iter_mut() {
                            if self.objects_equal(k, &key) {
                                *v = final_val.clone();
                                found = true;
                                break;
                            }
                        }
                        if !found {
                            pairs.push((key, final_val.clone()));
                        }
                    }
                    Object::Module { members, .. } => {
                        members.insert(prop, final_val.clone());
                    }
                    _ => {
                        return Object::Error {
                            message: format!(
                                "cannot assign to property on {}",
                                container.type_name()
                            ),
                            line,
                            column,
                        };
                    }
                }
                env.borrow_mut().update(&root, container);
                final_val
            }

            Expression::Index {
                left: obj_expr,
                index: idx_expr,
                ..
            } => {
                let root = match obj_expr.as_ref() {
                    Expression::Ident { value, .. } => value.clone(),
                    _ => {
                        return Object::Error {
                            message: "complex index assignment not supported".to_string(),
                            line,
                            column,
                        };
                    }
                };
                let mut container = match env.borrow().get(&root) {
                    Some(o) => o,
                    None => {
                        return Object::Error {
                            message: format!("identifier not found: {}", root),
                            line,
                            column,
                        };
                    }
                };
                let idx = self.eval_expression(idx_expr, env);
                if matches!(idx, Object::Error { .. }) {
                    return idx;
                }
                let final_val = if op.token_type == TokenType::Assign {
                    val
                } else {
                    let current = self.eval_index(container.clone(), idx.clone(), line, column);
                    if matches!(current, Object::Error { .. }) {
                        return current;
                    }
                    self.eval_infix(op, current, val, line, column)
                };
                if matches!(final_val, Object::Error { .. }) {
                    return final_val;
                }
                match &mut container {
                    Object::Array(elements) => {
                        if let Object::Integer(i) = &idx {
                            let i_raw = *i;
                            if i_raw < 0 {
                                return Object::Error { 
                                    message: format!("index {} out of range (len {})",i_raw, elements.len()), 
                                    line, column }
                            }

                            let i = i_raw as usize;
                            if i >= elements.len() {
                                return Object::Error {
                                    message: format!("index out of range: {}", i),
                                    line,
                                    column,
                                };
                            }
                            elements[i] = final_val.clone();
                        } else {
                            return Object::Error {
                                message: "array index must be integer".to_string(),
                                line,
                                column,
                            };
                        }
                    }
                    Object::Hash(pairs) => {
                        let mut found = false;
                        for (k, v) in pairs.iter_mut() {
                            if self.objects_equal(k, &idx) {
                                *v = final_val.clone();
                                found = true;
                                break;
                            }
                        }
                        if !found {
                            pairs.push((idx, final_val.clone()));
                        }
                    }
                    _ => {
                        return Object::Error {
                            message: format!(
                                "index assignment not supported for {}",
                                container.type_name()
                            ),
                            line,
                            column,
                        };
                    }
                }
                env.borrow_mut().update(&root, container);
                final_val
            }

            _ => Object::Error {
                message: "invalid left-hand side in assignment".to_string(),
                line,
                column,
            },
        }
    }

    fn eval_index(&self, left: Object, index: Object, line: usize, column: usize) -> Object {
        match (&left, &index) {
            (Object::Array(elements), Object::Integer(i)) => {
                let i = *i;
                if i < 0 || i as usize >= elements.len() {
                    return Object::Error {
                        message: format!("index {} out of range (len {})", i, elements.len()),
                        line,
                        column,
                    };
                }
                elements[i as usize].clone()
            }
            (Object::Hash(pairs), _) => {
                for (k, v) in pairs {
                    if self.objects_equal(k, &index) {
                        return v.clone();
                    }
                }
                Object::Error {
                    message: format!("key {} not found in hash", index),
                    line,
                    column,
                }
            }
            (Object::StringType(s), Object::Integer(i)) => {
                let i = *i;
                let char_count = s.chars().count();
                if i < 0 || i as usize >= char_count {
                    return Object::Error {
                        message: format!("index {} out of range (len {})", i, char_count),
                        line,
                        column,
                    };
                }
                match s.chars().nth(i as usize) {
                    Some(c) => Object::Char(c),
                    None => Object::Error {
                        message: format!("index {} out of range", i),
                        line,
                        column,
                    },
                }
            }
            _ => Object::Error {
                message: format!("index operator not supported: {}", left.type_name()),
                line,
                column,
            },
        }
    }

    fn eval_member_on_obj(&self, obj: &Object, prop: &str, line: usize, column: usize) -> Object {
        match obj {
            Object::StructInstance { fields, type_name } => match fields.get(prop) {
                Some(v) => v.clone(),
                None => Object::Error {
                    message: format!("unknown field {} on {}", prop, type_name),
                    line,
                    column,
                },
            },
            Object::Module {
                name,
                pub_gated,
                members,
            } => match members.get(prop) {
                Some(v) => v.clone(),
                None => {
                    let msg = if *pub_gated {
                        format!("{} has no public member '{}'", name, prop)
                    } else {
                        format!("{} has no member '{}'", name, prop)
                    };
                    Object::Error {
                        message: msg,
                        line,
                        column,
                    }
                }
            },
            Object::Hash(pairs) => {
                let key = Object::StringType(prop.to_string());
                for (k, v) in pairs {
                    if self.objects_equal(k, &key) {
                        return v.clone();
                    }
                }
                Object::Error {
                    message: format!("property not found: {}", prop),
                    line,
                    column,
                }
            }
            Object::EnumType { name, variants } => {
                if variants.contains(&prop.to_string()) {
                    Object::EnumVariant {
                        enum_name: name.clone(),
                        variant: prop.to_string(),
                    }
                } else {
                    Object::Error {
                        message: format!("{} has no variant '{}'", name, prop),
                        line,
                        column,
                    }
                }
            }
            _ => Object::Error {
                message: format!("cannot access property {} on {}", prop, obj.type_name()),
                line,
                column,
            },
        }
    }

    fn eval_args(&mut self, args: &[Expression], env: &Env) -> Vec<Object> {
        let mut result = Vec::new();
        for arg in args {
            let val = self.eval_expression(arg, env);
            if matches!(val, Object::Error { .. }) {
                return vec![val];
            }
            result.push(val);
        }
        result
    }

    pub fn apply_function(
        &mut self,
        func: Object,
        args: Vec<Object>,
        line: usize,
        column: usize,
    ) -> Object {
        match func {
            Object::Function {
                parameters,
                body,
                env: func_env,
            } => {
                if self.call_depth >= MAX_CALL_DEPTH {
                    return Object::Error {
                        message: format!("maximum call depth exceeded ({})", MAX_CALL_DEPTH),
                        line,
                        column,
                    };
                }

                // for self-methods, self is prepended to args — don't count it in user-facing messages
                let is_self_method = parameters
                    .first()
                    .map(|p| p.name == "self")
                    .unwrap_or(false);
                let user_params = if is_self_method {
                    parameters.len().saturating_sub(1)
                } else {
                    parameters.len()
                };
                let user_args = if is_self_method {
                    args.len().saturating_sub(1)
                } else {
                    args.len()
                };
                let required_total = parameters.iter().filter(|p| p.default.is_none()).count();
                let required_user = if is_self_method {
                    required_total.saturating_sub(1)
                } else {
                    required_total
                };

                if args.len() > parameters.len() {
                    return Object::Error {
                        message: format!(
                            "wrong number of arguments: expected {}, got {}",
                            user_params, user_args
                        ),
                        line,
                        column,
                    };
                }

                if args.len() < required_total {
                    return Object::Error {
                        message: format!(
                            "missing arguments: expected at least {}, got {}",
                            required_user, user_args
                        ),
                        line,
                        column,
                    };
                }

                let extended = Environment::new_enclosed(Rc::clone(&func_env));
                for (i, param) in parameters.iter().enumerate() {
                    let val = if i < args.len() {
                        args[i].clone()
                    } else {
                        self.eval_expression(param.default.as_ref().unwrap(), &extended)
                    };

                    extended.borrow_mut().set(param.name.clone(), val)
                }

                self.call_stack.push(CallFrame{
                    name:parameters[0].name.clone(),
                    call_line:line,
                    call_column:column
                });
                
                self.call_depth += 1;
                let result = self.eval_statement(&body, &extended);
                self.call_depth -= 1;
                self.call_stack.pop();
                
                match result {
                    Object::Return(v) => *v,
                    other => other,
                }
            }
            Object::Builtin(f) => {
                let result = f(args, CallInfo { line, column });
                // stamp call-site position onto errors that builtins emit with 0,0
                match result {
                    Object::Error {
                        message,
                        line: 0,
                        column: 0,
                    } => Object::Error {
                        message,
                        line,
                        column,
                    },
                    other => other,
                }
            }
            Object::BuiltinHigherOrder(f) => {
                let result = f(args, CallInfo { line, column }, self);
                match result {
                    Object::Error {
                        message,
                        line: 0,
                        column: 0,
                    } => Object::Error {
                        message,
                        line,
                        column,
                    },
                    other => other,
                }
            }
            _ => Object::Error {
                message: format!("not a function: {}", func.type_name()),
                line,
                column,
            },
        }
    }

    fn is_truthy(&self, obj: &Object) -> bool {
        !matches!(obj, Object::Null | Object::Bool(false))
    }

    fn is_assignment(&self, tt: &TokenType) -> bool {
        matches!(
            tt,
            TokenType::Assign
                | TokenType::AddAssign
                | TokenType::SubAssign
                | TokenType::MulAssign
                | TokenType::QuoAssign
                | TokenType::RemAssign
        )
    }

    fn op_str(&self, tt: &TokenType) -> &str {
        match tt {
            TokenType::Plus => "+",
            TokenType::Minus => "-",
            TokenType::Asterisk => "*",
            TokenType::SLASH => "/",
            TokenType::Rem => "%",
            TokenType::Square => "**",
            TokenType::Floor => "//",
            TokenType::EQ => "==",
            TokenType::NOTEQ => "!=",
            TokenType::LT => "<",
            TokenType::GT => ">",
            TokenType::LessThanEqual => "<=",
            TokenType::GreaterThanEqual => ">=",
            TokenType::And => "&&",
            TokenType::Or => "||",
            TokenType::Assign => "=",
            TokenType::AddAssign => "+=",
            TokenType::SubAssign => "-=",
            TokenType::MulAssign => "*=",
            TokenType::QuoAssign => "/=",
            TokenType::RemAssign => "%=",
            _ => "?",
        }
    }

    fn eval_import_statement(
        &mut self,
        path: &String,
        line: usize,
        column: usize,
        env: &Env,
    ) -> Object {
        if let Some(module) = self.module_cache.get(path) {
            let module = module.clone();
            env.borrow_mut().set(path.to_string(), module.clone());
            return module;
        }

        let file_name = format!("{}.cl", path);
        let content = match std::fs::read_to_string(&file_name) {
            Ok(c) => c,
            Err(e) => {
                return Object::Error {
                    message: format!("could not read module \"{}\": {}", path, e),
                    line,
                    column,
                };
            }
        };

        let lexer = Lexer::new(content);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();

        if !parser.errors.is_empty() {
            let msgs: Vec<String> = parser.errors.iter().map(|e| e.message.clone()).collect();
            let msg = format!("parse errors in \"{}\": {}", path, msgs.join("; "));
            return Object::Error {
                message: msg,
                line,
                column,
            };
        }

        let module_env = Environment::new_enclosed(Rc::clone(env));
        let eval_result = self.eval(&program, &module_env);
        if matches!(eval_result, Object::Error { .. }) {
            return eval_result;
        }

        let env_ref = module_env.borrow();
        let pub_gated = !env_ref.pubs.is_empty();
        let members: HashMap<String, Object> = if !pub_gated {
            // no pub statements → everything is public (backward compatible)
            env_ref.store.clone()
        } else {
            // filter to only pubed names
            env_ref
                .store
                .iter()
                .filter(|(k, _)| env_ref.pubs.contains(*k))
                .map(|(k, v)| (k.clone(), v.clone()))
                .collect()
        };
        drop(env_ref);

        let module = Object::Module {
            name: path.clone(),
            pub_gated,
            members,
        };

        env.borrow_mut().set(path.to_string(), module.clone());
        self.module_cache.insert(path.to_string(), module.clone());

        module
    }
}

impl Evaluable for Evaluator {
    fn call_function(&mut self, func: Object, args: Vec<Object>, info: CallInfo) -> Object {
        self.apply_function(func, args, info.line, info.column)
    }
}

fn builtin_is_error(args: Vec<Object>, _: CallInfo) -> Object {
    match args.as_slice() {
        [val] => Object::Bool(matches!(val, Object::Error { .. })),
        _ => Object::Error {
            message: "is_error expects 1 argument".to_string(),
            line: 0,
            column: 0,
        },
    }
}
