use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{
    ast::ast::{Expression, Program, Statement},
    lexer::lexer::Lexer,
    object::object::{Environment, Object},
    parser::parser::Parser,
    token::token::{Token, TokenType},
};

pub struct Evaluator {
    pub loop_depth: usize,
    pub module_cache: HashMap<String, Object>,
}

type Env = Rc<RefCell<Environment>>;

impl Evaluator {
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
                    if matches!(result, Object::Return(_) | Object::Error { .. } | Object::Break | Object::Continue) {
                        return result;
                    }
                }
                result
            }

            Statement::Let { name, value, .. } => {
                let val = self.eval_expression(value, env);
                if matches!(val, Object::Error { .. }) { return val; }
                env.borrow_mut().set(name.clone(), val);
                Object::Null
            }

            Statement::Const { name, value, .. } => {
                let val = self.eval_expression(value, env);
                if matches!(val, Object::Error { .. }) { return val; }
                env.borrow_mut().set_const(name.clone(), val);
                Object::Null
            }

            Statement::Return { value, .. } => {
                let val = self.eval_expression(value, env);
                if matches!(val, Object::Error { .. }) { return val; }
                Object::Return(Box::new(val))
            }

            Statement::Expression { expr, .. } => {
                let val = self.eval_expression(expr, env);
                if matches!(val, Object::Error { .. }) { return val; }
                val
            }

            Statement::Struct { name, field } => {
                let mut defaults: HashMap<String, Box<Object>> = HashMap::new();
                for (key, expr) in field {
                    let val = self.eval_expression(expr, env);
                    if matches!(val, Object::Error { .. }) { return val; }
                    defaults.insert(key.clone(), Box::new(val));
                }
                let struct_name = match name.as_ref() {
                    Expression::Ident { value, .. } => value.clone(),
                    _ => return Object::Error { message: "struct name must be an identifier".to_string(), line: 0, column: 0 },
                };
                let obj = Object::StructType { name: struct_name.clone(), default: defaults };
                env.borrow_mut().set(struct_name, obj);
                Object::Null
            }

            Statement::Import { path } => self.eval_import_statement(path, env),

            Statement::Break => {
                if self.loop_depth == 0 {
                    return Object::Error { message: "break outside of loop".to_string(), line: 0, column: 0 };
                }
                Object::Break
            }

            Statement::Continue => {
                if self.loop_depth == 0 {
                    return Object::Error { message: "continue outside of loop".to_string(), line: 0, column: 0 };
                }
                Object::Continue
            }
        }
    }

    fn eval_expression(&mut self, expr: &Expression, env: &Env) -> Object {
        match expr {
            Expression::Int { value, .. } => Object::Integer(*value as i64),
            Expression::Float { value, .. } => Object::Float(*value),
            Expression::StringLit { value, .. } => Object::StringType(value.clone()),
            Expression::Char { value, .. } => Object::Char(*value),
            Expression::Boolean { value, .. } => Object::Bool(*value),

            Expression::Ident { value, line, column } => {
                match env.borrow().get(value) {
                    Some(obj) => obj,
                    None => Object::Error { message: format!("identifier not found: {}", value), line: *line, column: *column },
                }
            }

            Expression::Prefix { op, right, line, column } => {
                let right_val = self.eval_expression(right, env);
                if matches!(right_val, Object::Error { .. }) { return right_val; }
                self.eval_prefix(op, right_val, *line, *column)
            }

            Expression::Infix { left, op, right, line, column } => {
                if self.is_assignment(&op.token_type) {
                    return self.eval_assignment(left, op, right, env, *line, *column);
                }
                if op.token_type == TokenType::And {
                    let l = self.eval_expression(left, env);
                    if matches!(l, Object::Error { .. }) { return l; }
                    if !self.is_truthy(&l) { return l; }
                    return self.eval_expression(right, env);
                }
                if op.token_type == TokenType::Or {
                    let l = self.eval_expression(left, env);
                    if matches!(l, Object::Error { .. }) { return l; }
                    if self.is_truthy(&l) { return l; }
                    return self.eval_expression(right, env);
                }
                let l = self.eval_expression(left, env);
                if matches!(l, Object::Error { .. }) { return l; }
                let r = self.eval_expression(right, env);
                if matches!(r, Object::Error { .. }) { return r; }
                self.eval_infix(op, l, r, *line, *column)
            }

            Expression::If { condition, consequence, alternative, if_else, .. } => {
                let cond = self.eval_expression(condition, env);
                if matches!(cond, Object::Error { .. }) { return cond; }
                if self.is_truthy(&cond) {
                    return self.eval_statement(consequence, env);
                }
                for branch in if_else {
                    let c = self.eval_expression(&branch.condition, env);
                    if matches!(c, Object::Error { .. }) { return c; }
                    if self.is_truthy(&c) {
                        return self.eval_statement(&branch.consequences, env);
                    }
                }
                if let Some(alt) = alternative {
                    return self.eval_statement(alt, env);
                }
                Object::Null
            }

            Expression::Function { parameter, body, .. } => {
                Object::Function {
                    parameters: parameter.clone(),
                    body: body.clone(),
                    env: Rc::clone(env),
                }
            }

            Expression::Call { function, argument, line, column } => {
                let func = self.eval_expression(function, env);
                if matches!(func, Object::Error { .. }) { return func; }
                let args = self.eval_args(argument, env);
                if args.len() == 1 && matches!(args[0], Object::Error { .. }) {
                    return args.into_iter().next().unwrap();
                }
                self.apply_function(func, args, *line, *column)
            }

            Expression::Array { element, .. } => {
                let elems = self.eval_args(element, env);
                if elems.len() == 1 && matches!(elems[0], Object::Error { .. }) {
                    return elems.into_iter().next().unwrap();
                }
                Object::Array(elems)
            }

            Expression::Index { left, index, line, column } => {
                let left_val = self.eval_expression(left, env);
                if matches!(left_val, Object::Error { .. }) { return left_val; }
                let idx = self.eval_expression(index, env);
                if matches!(idx, Object::Error { .. }) { return idx; }
                self.eval_index(left_val, idx, *line, *column)
            }

            Expression::HashLiteral { pair, .. } => {
                let mut pairs = Vec::new();
                for (k_expr, v_expr) in pair {
                    let k = self.eval_expression(k_expr, env);
                    if matches!(k, Object::Error { .. }) { return k; }
                    let v = self.eval_expression(v_expr, env);
                    if matches!(v, Object::Error { .. }) { return v; }
                    pairs.push((k, v));
                }
                Object::Hash(pairs)
            }

            Expression::While { condition, body, .. } => {
                self.loop_depth += 1;
                let mut result = Object::Null;
                loop {
                    let cond = self.eval_expression(condition, env);
                    if matches!(cond, Object::Error { .. }) {
                        self.loop_depth -= 1;
                        return cond;
                    }
                    if !self.is_truthy(&cond) { break; }
                    result = self.eval_statement(body, env);
                    match result {
                        Object::Break => { result = Object::Null; break; }
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

            Expression::For { init, condition, post, body, .. } => {
                let for_env = Environment::new_enclosed(Rc::clone(env));
                let init_res = self.eval_statement(init, &for_env);
                if matches!(init_res, Object::Error { .. }) { return init_res; }
                self.loop_depth += 1;
                let mut result = Object::Null;
                loop {
                    let cond = self.eval_expression(condition, &for_env);
                    if matches!(cond, Object::Error { .. }) {
                        self.loop_depth -= 1;
                        return cond;
                    }
                    if !self.is_truthy(&cond) { break; }
                    result = self.eval_statement(body, &for_env);
                    match result {
                        Object::Break => { result = Object::Null; break; }
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

            Expression::Update { operator, target, prefix, line, column } => {
                let current = self.eval_expression(target, env);
                if matches!(current, Object::Error { .. }) { return current; }
                let updated = match &current {
                    Object::Integer(v) => if matches!(operator.token_type, TokenType::Inc) { Object::Integer(v + 1) } else { Object::Integer(v - 1) },
                    Object::Float(v) => if matches!(operator.token_type, TokenType::Inc) { Object::Float(v + 1.0) } else { Object::Float(v - 1.0) },
                    _ => return Object::Error { message: format!("unsupported operand for update: {}", current.type_name()), line: *line, column: *column },
                };
                match target.as_ref() {
                    Expression::Ident { value: name, .. } => {
                        if !env.borrow_mut().update(name, updated.clone()) {
                            env.borrow_mut().set(name.clone(), updated.clone());
                        }
                    }
                    Expression::Member { object: obj_expr, property, .. } => {
                        if let (Expression::Ident { value: root, .. }, Expression::Ident { value: field, .. }) = (obj_expr.as_ref(), property.as_ref()) {
                            if let Some(mut obj) = env.borrow().get(root) {
                                match &mut obj {
                                    Object::StructInstance { fields, .. } => { fields.insert(field.clone(), updated.clone()); }
                                    _ => {}
                                }
                                env.borrow_mut().update(root, obj);
                            }
                        }
                    }
                    Expression::Index { left: obj_expr, index: idx_expr, .. } => {
                        if let Expression::Ident { value: root, .. } = obj_expr.as_ref() {
                            let idx = self.eval_expression(idx_expr, env);
                            if let Some(mut obj) = env.borrow().get(root) {
                                match &mut obj {
                                    Object::Array(elements) => {
                                        if let Object::Integer(i) = &idx {
                                            let i = *i as usize;
                                            if i < elements.len() { elements[i] = updated.clone(); }
                                        }
                                    }
                                    _ => {}
                                }
                                env.borrow_mut().update(root, obj);
                            }
                        }
                    }
                    _ => {}
                }
                if *prefix { updated } else { current }
            }

            Expression::Member { object, property, line, column } => {
                let obj = self.eval_expression(object, env);
                if matches!(obj, Object::Error { .. }) { return obj; }
                let prop = match property.as_ref() {
                    Expression::Ident { value, .. } => value.clone(),
                    _ => return Object::Error { message: "invalid property".to_string(), line: *line, column: *column },
                };
                self.eval_member_on_obj(&obj, &prop, *line, *column)
            }

            Expression::StructLiteral { name, fields, line, column } => {
                let obj = match env.borrow().get(name) {
                    Some(o) => o,
                    None => return Object::Error { message: format!("unknown struct: {}", name), line: *line, column: *column },
                };
                let mut instance_fields: HashMap<String, Object> = match obj {
                    Object::StructType { ref default, .. } => {
                        default.iter().map(|(k, v)| (k.clone(), *v.clone())).collect()
                    }
                    _ => return Object::Error { message: format!("{} is not a struct", name), line: *line, column: *column },
                };
                for (k, v_expr) in fields {
                    let val = self.eval_expression(v_expr, env);
                    if matches!(val, Object::Error { .. }) { return val; }
                    instance_fields.insert(k.clone(), val);
                }
                Object::StructInstance { type_name: name.clone(), fields: instance_fields }
            }
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
                _ => Object::Error { message: format!("unknown operator: -{}", right.type_name()), line, column },
            },
            _ => Object::Error { message: "unknown prefix operator".to_string(), line, column },
        }
    }

    fn eval_infix(&self, op: &Token, left: Object, right: Object, line: usize, column: usize) -> Object {
        match (&left, &right) {
            (Object::Integer(l), Object::Integer(r)) => self.eval_integer_infix(&op.token_type, *l, *r, line, column),
            (Object::Float(l), Object::Float(r)) => self.eval_float_infix(&op.token_type, *l, *r, line, column),
            (Object::Integer(l), Object::Float(r)) => self.eval_float_infix(&op.token_type, *l as f64, *r, line, column),
            (Object::Float(l), Object::Integer(r)) => self.eval_float_infix(&op.token_type, *l, *r as f64, line, column),
            (Object::StringType(l), Object::StringType(r)) => match &op.token_type {
                TokenType::Plus | TokenType::AddAssign => Object::StringType(l.clone() + r),
                TokenType::EQ => Object::Bool(l == r),
                TokenType::NOTEQ => Object::Bool(l != r),
                _ => Object::Error { message: format!("unknown operator: STRING {} STRING", self.op_str(&op.token_type)), line, column },
            },
            (Object::StringType(l), Object::Char(r)) => match &op.token_type {
                TokenType::Plus => Object::StringType(format!("{}{}", l, r)),
                _ => Object::Error { message: format!("unknown operator: STRING {} CHAR", self.op_str(&op.token_type)), line, column },
            },
            (Object::Char(l), Object::Char(r)) => match &op.token_type {
                TokenType::Plus => Object::StringType(format!("{}{}", l, r)),
                TokenType::EQ => Object::Bool(l == r),
                TokenType::NOTEQ => Object::Bool(l != r),
                _ => Object::Error { message: format!("unknown operator: CHAR {} CHAR", self.op_str(&op.token_type)), line, column },
            },
            (Object::Bool(l), Object::Bool(r)) => match &op.token_type {
                TokenType::EQ => Object::Bool(l == r),
                TokenType::NOTEQ => Object::Bool(l != r),
                _ => Object::Error { message: format!("unknown operator: BOOL {} BOOL", self.op_str(&op.token_type)), line, column },
            },
            _ => {
                if left.type_name() != right.type_name() {
                    Object::Error { message: format!("type mismatch: {} {} {}", left.type_name(), self.op_str(&op.token_type), right.type_name()), line, column }
                } else {
                    Object::Error { message: format!("unknown operator: {} {} {}", left.type_name(), self.op_str(&op.token_type), right.type_name()), line, column }
                }
            }
        }
    }

    fn eval_integer_infix(&self, op: &TokenType, l: i64, r: i64, line: usize, column: usize) -> Object {
        match op {
            TokenType::Plus | TokenType::AddAssign => Object::Integer(l + r),
            TokenType::Minus | TokenType::SubAssign => Object::Integer(l - r),
            TokenType::Asterisk | TokenType::MulAssign => Object::Integer(l * r),
            TokenType::SLASH | TokenType::QuoAssign => {
                if r == 0 { return Object::Error { message: "division by zero".to_string(), line, column }; }
                Object::Integer(l / r)
            }
            TokenType::Rem | TokenType::RemAssign => {
                if r == 0 { return Object::Error { message: "division by zero".to_string(), line, column }; }
                Object::Integer(l % r)
            }
            TokenType::Square => Object::Integer((l as f64).powf(r as f64) as i64),
            TokenType::Floor => {
                if r == 0 { return Object::Error { message: "division by zero".to_string(), line, column }; }
                Object::Integer(((l as f64) / (r as f64)).floor() as i64)
            }
            TokenType::LT => Object::Bool(l < r),
            TokenType::GT => Object::Bool(l > r),
            TokenType::LessThanEqual => Object::Bool(l <= r),
            TokenType::GreaterThanEqual => Object::Bool(l >= r),
            TokenType::EQ => Object::Bool(l == r),
            TokenType::NOTEQ => Object::Bool(l != r),
            _ => Object::Error { message: format!("unknown operator: INTEGER {} INTEGER", self.op_str(op)), line, column },
        }
    }

    fn eval_float_infix(&self, op: &TokenType, l: f64, r: f64, line: usize, column: usize) -> Object {
        match op {
            TokenType::Plus | TokenType::AddAssign => Object::Float(l + r),
            TokenType::Minus | TokenType::SubAssign => Object::Float(l - r),
            TokenType::Asterisk | TokenType::MulAssign => Object::Float(l * r),
            TokenType::SLASH | TokenType::QuoAssign => {
                if r == 0.0 { return Object::Error { message: "division by zero".to_string(), line, column }; }
                Object::Float(l / r)
            }
            TokenType::Rem | TokenType::RemAssign => {
                if r == 0.0 { return Object::Error { message: "division by zero".to_string(), line, column }; }
                Object::Float(l % r)
            }
            TokenType::Square => Object::Float(l.powf(r)),
            TokenType::Floor => {
                if r == 0.0 { return Object::Error { message: "division by zero".to_string(), line, column }; }
                Object::Float((l / r).floor())
            }
            TokenType::LT => Object::Bool(l < r),
            TokenType::GT => Object::Bool(l > r),
            TokenType::LessThanEqual => Object::Bool(l <= r),
            TokenType::GreaterThanEqual => Object::Bool(l >= r),
            TokenType::EQ => Object::Bool(l == r),
            TokenType::NOTEQ => Object::Bool(l != r),
            _ => Object::Error { message: format!("unknown operator: FLOAT {} FLOAT", self.op_str(op)), line, column },
        }
    }

    fn eval_assignment(&mut self, left: &Expression, op: &Token, right: &Expression, env: &Env, line: usize, column: usize) -> Object {
        let val = self.eval_expression(right, env);
        if matches!(val, Object::Error { .. }) { return val; }

        match left {
            Expression::Ident { value: name, .. } => {
                let final_val = if op.token_type == TokenType::Asign {
                    val
                } else {
                    let current = match env.borrow().get(name) {
                        Some(v) => v,
                        None => return Object::Error { message: format!("identifier not found: {}", name), line, column },
                    };
                    self.eval_infix(op, current, val, line, column)
                };
                if matches!(final_val, Object::Error { .. }) { return final_val; }
                if !env.borrow_mut().update(name, final_val.clone()) {
                    env.borrow_mut().set(name.clone(), final_val.clone());
                }
                final_val
            }

            Expression::Member { object: obj_expr, property, .. } => {
                let prop = match property.as_ref() {
                    Expression::Ident { value, .. } => value.clone(),
                    _ => return Object::Error { message: "invalid member property".to_string(), line, column },
                };
                let root = match obj_expr.as_ref() {
                    Expression::Ident { value, .. } => value.clone(),
                    _ => return Object::Error { message: "complex member assignment not supported".to_string(), line, column },
                };
                let mut container = match env.borrow().get(&root) {
                    Some(o) => o,
                    None => return Object::Error { message: format!("identifier not found: {}", root), line, column },
                };
                let final_val = if op.token_type == TokenType::Asign {
                    val
                } else {
                    let current = self.eval_member_on_obj(&container, &prop, line, column);
                    if matches!(current, Object::Error { .. }) { return current; }
                    self.eval_infix(op, current, val, line, column)
                };
                if matches!(final_val, Object::Error { .. }) { return final_val; }
                match &mut container {
                    Object::StructInstance { fields, .. } => { fields.insert(prop, final_val.clone()); }
                    Object::Hash(pairs) => {
                        let key = Object::StringType(prop.clone());
                        let mut found = false;
                        for (k, v) in pairs.iter_mut() {
                            if self.objects_eq(k, &key) { *v = final_val.clone(); found = true; break; }
                        }
                        if !found { pairs.push((key, final_val.clone())); }
                    }
                    Object::Module { members } => { members.insert(prop, final_val.clone()); }
                    _ => return Object::Error { message: format!("cannot assign to property on {}", container.type_name()), line, column },
                }
                env.borrow_mut().update(&root, container);
                final_val
            }

            Expression::Index { left: obj_expr, index: idx_expr, .. } => {
                let root = match obj_expr.as_ref() {
                    Expression::Ident { value, .. } => value.clone(),
                    _ => return Object::Error { message: "complex index assignment not supported".to_string(), line, column },
                };
                let mut container = match env.borrow().get(&root) {
                    Some(o) => o,
                    None => return Object::Error { message: format!("identifier not found: {}", root), line, column },
                };
                let idx = self.eval_expression(idx_expr, env);
                if matches!(idx, Object::Error { .. }) { return idx; }
                let final_val = if op.token_type == TokenType::Asign {
                    val
                } else {
                    let current = self.eval_index(container.clone(), idx.clone(), line, column);
                    if matches!(current, Object::Error { .. }) { return current; }
                    self.eval_infix(op, current, val, line, column)
                };
                if matches!(final_val, Object::Error { .. }) { return final_val; }
                match &mut container {
                    Object::Array(elements) => {
                        if let Object::Integer(i) = &idx {
                            let i = *i as usize;
                            if i >= elements.len() {
                                return Object::Error { message: format!("index out of range: {}", i), line, column };
                            }
                            elements[i] = final_val.clone();
                        } else {
                            return Object::Error { message: "array index must be integer".to_string(), line, column };
                        }
                    }
                    Object::Hash(pairs) => {
                        let mut found = false;
                        for (k, v) in pairs.iter_mut() {
                            if self.objects_eq(k, &idx) { *v = final_val.clone(); found = true; break; }
                        }
                        if !found { pairs.push((idx, final_val.clone())); }
                    }
                    _ => return Object::Error { message: format!("index assignment not supported for {}", container.type_name()), line, column },
                }
                env.borrow_mut().update(&root, container);
                final_val
            }

            _ => Object::Error { message: "invalid left-hand side in assignment".to_string(), line, column },
        }
    }

    fn eval_index(&self, left: Object, index: Object, line: usize, column: usize) -> Object {
        match (&left, &index) {
            (Object::Array(elements), Object::Integer(i)) => {
                let i = *i;
                if i < 0 || i as usize >= elements.len() {
                    return Object::Error { message: format!("index {} out of range (len {})", i, elements.len()), line, column };
                }
                elements[i as usize].clone()
            }
            (Object::Hash(pairs), _) => {
                for (k, v) in pairs {
                    if self.objects_eq(k, &index) { return v.clone(); }
                }
                Object::Error { message: format!("key {} not found in hash", index), line, column }
            }
            (Object::StringType(s), Object::Integer(i)) => {
                let i = *i;
                if i < 0 || i as usize >= s.len() {
                    return Object::Error { message: format!("index {} out of range (len {})", i, s.len()), line, column };
                }
                Object::Char(s.chars().nth(i as usize).unwrap())
            }
            _ => Object::Error { message: format!("index operator not supported: {}", left.type_name()), line, column },
        }
    }

    fn eval_member_on_obj(&self, obj: &Object, prop: &str, line: usize, column: usize) -> Object {
        match obj {
            Object::StructInstance { fields, type_name } => match fields.get(prop) {
                Some(v) => v.clone(),
                None => Object::Error { message: format!("unknown field {} on {}", prop, type_name), line, column },
            },
            Object::Module { members } => match members.get(prop) {
                Some(v) => v.clone(),
                None => Object::Error { message: format!("module has no member {}", prop), line, column },
            },
            Object::Hash(pairs) => {
                let key = Object::StringType(prop.to_string());
                for (k, v) in pairs {
                    if self.objects_eq(k, &key) { return v.clone(); }
                }
                Object::Error { message: format!("property not found: {}", prop), line, column }
            }
            _ => Object::Error { message: format!("cannot access property {} on {}", prop, obj.type_name()), line, column },
        }
    }

    fn eval_args(&mut self, args: &[Expression], env: &Env) -> Vec<Object> {
        let mut result = Vec::new();
        for arg in args {
            let val = self.eval_expression(arg, env);
            if matches!(val, Object::Error { .. }) { return vec![val]; }
            result.push(val);
        }
        result
    }

    fn apply_function(&mut self, func: Object, args: Vec<Object>, line: usize, column: usize) -> Object {
        match func {
            Object::Function { parameters, body, env: func_env } => {
                let extended = Environment::new_enclosed(Rc::clone(&func_env));
                for (param, arg) in parameters.iter().zip(args.iter()) {
                    if let Expression::Ident { value, .. } = param {
                        extended.borrow_mut().set(value.clone(), arg.clone());
                    }
                }
                let result = self.eval_statement(&body, &extended);
                match result {
                    Object::Return(v) => *v,
                    other => other,
                }
            }
            Object::Builtin(f) => f(args),
            _ => Object::Error { message: format!("not a function: {}", func.type_name()), line, column },
        }
    }

    fn is_truthy(&self, obj: &Object) -> bool {
        match obj {
            Object::Null => false,
            Object::Bool(false) => false,
            _ => true,
        }
    }

    fn is_assignment(&self, tt: &TokenType) -> bool {
        matches!(tt, TokenType::Asign | TokenType::AddAssign | TokenType::SubAssign | TokenType::MulAssign | TokenType::QuoAssign | TokenType::RemAssign)
    }

    fn objects_eq(&self, a: &Object, b: &Object) -> bool {
        match (a, b) {
            (Object::Integer(x), Object::Integer(y)) => x == y,
            (Object::Bool(x), Object::Bool(y)) => x == y,
            (Object::StringType(x), Object::StringType(y)) => x == y,
            (Object::Char(x), Object::Char(y)) => x == y,
            _ => false,
        }
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
            TokenType::Asign => "=",
            TokenType::AddAssign => "+=",
            TokenType::SubAssign => "-=",
            TokenType::MulAssign => "*=",
            TokenType::QuoAssign => "/=",
            TokenType::RemAssign => "%=",
            _ => "?",
        }
    }

    fn eval_import_statement(&mut self, path: &String, env: &Env) -> Object {
        if let Some(module) = self.module_cache.get(path) {
            let module = module.clone();
            env.borrow_mut().set(path.to_string(), module.clone());
            return module;
        }

        let file_name = format!("{}.cl", path);
        let content = match std::fs::read_to_string(&file_name) {
            Ok(c) => c,
            Err(e) => return Object::Error { message: format!("could not read module \"{}\": {}", path, e), line: 0, column: 0 },
        };

        let lexer = Lexer::new(content);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();

        let module_env = Environment::new_enclosed(Rc::clone(env));
        self.eval(&program, &module_env);

        let members: HashMap<String, Object> = module_env.borrow().store.clone();
        let module = Object::Module { members };

        env.borrow_mut().set(path.to_string(), module.clone());
        self.module_cache.insert(path.to_string(), module.clone());

        module
    }
}
