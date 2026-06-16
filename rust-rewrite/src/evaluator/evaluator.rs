use std::{cell::RefCell, rc::Rc};

use crate::{ast::ast::{Expression, Program, Statement}, object::object::{Environment, Object}};


pub struct Evaluator {
    pub loop_depth: usize,
}

type Env = Rc<RefCell<Environment>>;

impl Evaluator {
    pub fn eval(&self, node:&Program, env:&Env) -> Object {
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

    fn eval_statement(&self, stmt:&Statement, env:&Env) -> Object {
        match stmt {
            Statement::Block { statements, .. } => {
                let mut result = Object::Null;
                for s in statements {
                    result = self.eval_statement(s, env);
                    if matches!(result, Object::Return(_) | Object::Error {..}){
                        return result;
                    };
                };

                result
            },

            Statement::Const {name, value, .. } => {
                let value = self.eval_expression(value, env);
                if matches!(value, Object::Error { .. } | Object::Return(_)){
                    return value
                }
                env.borrow_mut().set(name.clone(), value);
                Object::Null
            },

            Statement::Let { name, value,..} => {
                let value = self.eval_expression(value, env);
                if matches!(value, Object::Error { .. } | Object::Return(_)){
                    return value;
                }
                env.borrow_mut().set(name.clone(), value);
                Object::Null
            }

            Statement::Expression { expr, .. } => {
                let expr = self.eval_expression(expr, env);
                if matches!(expr, Object::Error { .. } | Object::Return(_)){
                    return expr;
                }
                expr
            }

            Statement::Return { value, .. } => {
                let value = self.eval_expression(value, env);
                if matches!(value, Object::Error { .. } | Object::Return(_)){
                    return value;
                }
                Object::Return(Box::new(value))
            }
        }
    }

    fn eval_expression(&self, expr:&Expression, env:&Env) -> Object {
        match expr {
            Expression::Int { value, .. } => {
                Object::Integer(*value as i64)
            },
            Expression::Float { value, .. } => {
                Object::Float(*value as f64)
            },
            Expression::StringLit { value, .. } => {
                Object::StringType(value.to_string())
            },
            Expression::Char { value, .. } => {
                Object::Char(*value)
            },
            Expression::Boolean { value, .. } => {
                Object::Bool(*value)
            }
            _ => Object::Null
        }
    }
}