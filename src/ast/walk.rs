use crate::{
    ast::ast::{Expression, LetPattern, Param, Program, Statement},
    token::token::{Token, TokenType},
};

// The Visitor trait.
// Every method has a default implementation that calls the
// corresponding walk_* function, which recurses into children.
// Override only the methods you care about.

pub trait Visitor: Sized {
    fn visit_program(&mut self, program: &Program) {
        walk_program(self, program);
    }

    fn visit_statement(&mut self, stmt: &Statement) {
        walk_statement(self, stmt);
    }

    fn visit_expression(&mut self, expr: &Expression) {
        walk_expression(self, expr);
    }

    // Statement-level hooks — called before walking children
    fn visit_let(&mut self, _pattern: &LetPattern, _value: &Expression, _line: usize, _col: usize) {
    }
    fn visit_const(
        &mut self,
        _pattern: &LetPattern,
        _value: &Expression,
        _line: usize,
        _col: usize,
    ) {
    }
    fn visit_return(&mut self, _value: &Expression, _line: usize, _col: usize) {}
    fn visit_import(&mut self, _path: &str, _line: usize, _col: usize) {}
    fn visit_block(&mut self, _stmts: &[Statement], _line: usize, _col: usize) {}
    fn visit_enum_decl(&mut self, _name: &str, _variants: &[String], _line: usize, _col: usize) {}
    fn visit_struct_decl(
        &mut self,
        _name: &Expression,
        _fields: &std::collections::HashMap<String, Expression>,
    ) {
    }

    // Expression-level hooks
    fn visit_ident(&mut self, _value: &str, _line: usize, _col: usize) {}
    fn visit_int(&mut self, _value: isize, _line: usize, _col: usize) {}
    fn visit_float(&mut self, _value: f64, _line: usize, _col: usize) {}
    fn visit_bool(&mut self, _value: bool, _line: usize, _col: usize) {}
    fn visit_null(&mut self, _line: usize, _col: usize) {}
    fn visit_call(
        &mut self,
        _fn_expr: &Expression,
        _args: &[Expression],
        _line: usize,
        _col: usize,
    ) {
    }
    fn visit_member(
        &mut self,
        _object: &Expression,
        _property: &Expression,
        _line: usize,
        _col: usize,
    ) {
    }
    fn visit_infix(
        &mut self,
        _op: TokenType,
        _left: &Expression,
        _right: &Expression,
        _line: usize,
        _col: usize,
    ) {
    }
    fn visit_function(&mut self, _params: &[Param], _body: &Statement, _line: usize, _col: usize) {}
    fn visit_if(
        &mut self,
        _condition: &Expression,
        _consequence: &Statement,
        _line: usize,
        _col: usize,
    ) {
    }
    fn visit_update(&mut self, _operator: &Token, _target: &Expression, _line: usize, _col: usize) {
    }
}

// walk_* functions recurse into children and call the hooks above.
// Tools that override visit_X but still want child traversal should
// call the corresponding walk_* manually at the end of their override.

pub fn walk_program<V: Visitor>(v: &mut V, program: &Program) {
    for stmt in &program.statements {
        v.visit_statement(stmt);
    }
}

pub fn walk_statement<V: Visitor>(v: &mut V, stmt: &Statement) {
    match stmt {
        Statement::Let {
            pattern,
            value,
            line,
            column,
            ..
        } => {
            v.visit_let(pattern, value, *line, *column);
            v.visit_expression(value);
        }

        Statement::Const {
            pattern,
            value,
            line,
            column,
            ..
        } => {
            v.visit_const(pattern, value, *line, *column);
            v.visit_expression(value);
        }

        Statement::Return {
            value,
            line,
            column,
            ..
        } => {
            v.visit_return(value, *line, *column);
            v.visit_expression(value);
        }

        Statement::Import {
            path, line, column, ..
        } => {
            v.visit_import(path, *line, *column);
            // no children
        }

        Statement::Block {
            statements,
            line,
            column,
            ..
        } => {
            v.visit_block(statements, *line, *column);
            for s in statements {
                v.visit_statement(s);
            }
        }

        Statement::Expression { expr, .. } => {
            v.visit_expression(expr);
        }

        Statement::Enum {
            name,
            variant,
            line,
            column,
            ..
        } => {
            v.visit_enum_decl(name, variant, *line, *column);
        }

        Statement::Struct { name, field, .. } => {
            v.visit_struct_decl(name, field);
            for val in field.values() {
                v.visit_expression(val);
            }
        }

        Statement::Pub { statement, .. } => {
            v.visit_statement(statement);
        }

        Statement::Break { .. } | Statement::Continue { .. } => {
            // leaf nodes, no children
        }
    }
}

pub fn walk_expression<V: Visitor>(v: &mut V, expr: &Expression) {
    match expr {
        Expression::Ident {
            value,
            line,
            column,
            ..
        } => {
            v.visit_ident(value, *line, *column);
        }

        Expression::Int {
            value,
            line,
            column,
            ..
        } => {
            v.visit_int(*value, *line, *column);
        }

        Expression::Float {
            value,
            line,
            column,
            ..
        } => {
            v.visit_float(*value, *line, *column);
        }

        Expression::Boolean {
            value,
            line,
            column,
            ..
        } => {
            v.visit_bool(*value, *line, *column);
        }

        Expression::Null { line, column, .. } => {
            v.visit_null(*line, *column);
        }

        Expression::Call {
            function,
            argument,
            line,
            column,
            ..
        } => {
            v.visit_call(function, argument, *line, *column);
            v.visit_expression(function);
            for arg in argument {
                v.visit_expression(arg);
            }
        }

        Expression::Member {
            object,
            property,
            line,
            column,
            ..
        } => {
            v.visit_member(object, property, *line, *column);
            v.visit_expression(object);
            v.visit_expression(property);
        }

        Expression::Infix {
            left,
            op,
            right,
            line,
            column,
            ..
        } => {
            v.visit_infix(op.token_type.clone(), left, right, *line, *column);
            v.visit_expression(left);
            v.visit_expression(right);
        }

        Expression::Prefix { right, .. } => {
            v.visit_expression(right);
        }

        Expression::Function {
            parameter,
            body,
            line,
            column,
            ..
        } => {
            v.visit_function(parameter, body, *line, *column);
            v.visit_statement(body);
        }

        Expression::If {
            condition,
            consequence,
            alternative,
            if_else,
            line,
            column,
            ..
        } => {
            v.visit_if(condition, consequence, *line, *column);
            v.visit_expression(condition);
            v.visit_statement(consequence);
            if let Some(alt) = alternative {
                v.visit_statement(alt);
            }
            for elif in if_else {
                v.visit_expression(&elif.condition);
                v.visit_statement(&elif.consequences);
            }
        }

        Expression::While {
            condition, body, ..
        } => {
            v.visit_expression(condition);
            v.visit_statement(body);
        }

        Expression::For {
            init,
            condition,
            post,
            body,
            ..
        } => {
            v.visit_statement(init);
            v.visit_expression(condition);
            v.visit_statement(post);
            v.visit_statement(body);
        }

        Expression::ForIn { iterable, body, .. } => {
            v.visit_expression(iterable);
            v.visit_statement(body);
        }

        Expression::Switch { subject, arms, .. } => {
            v.visit_expression(subject);
            for arm in arms {
                v.visit_expression(&arm.pattern);
                v.visit_statement(&arm.body);
            }
        }

        Expression::NullCoalesce { left, right, .. } => {
            v.visit_expression(left);
            v.visit_expression(right);
        }

        Expression::Typeof { value, .. } => {
            v.visit_expression(value);
        }

        Expression::Index { left, index, .. } => {
            v.visit_expression(left);
            v.visit_expression(index);
        }

        Expression::Array { element, .. } => {
            for e in element {
                v.visit_expression(e);
            }
        }

        Expression::HashLiteral { pair, .. } => {
            for (k, val) in pair {
                v.visit_expression(k);
                v.visit_expression(val);
            }
        }

        Expression::InterpolatedString { parts, .. } => {
            for part in parts {
                if let crate::ast::ast::StringSegment::Expr(e) = part {
                    v.visit_expression(e);
                }
            }
        }

        Expression::StructLiteral { fields, .. } => {
            for val in fields.values() {
                v.visit_expression(val);
            }
        }

        Expression::Update {
            operator,
            target,
            line,
            column,
            ..
        } => {
            v.visit_update(operator, target, *line, *column);
            v.visit_expression(target);
        }

        Expression::Char { .. } => {
            // leaf, no children
        }
    }
}
