use crate::ast::ast::{Expression, LetPattern, Program, Statement};
use crate::ast::walk::{Visitor, walk_expression, walk_statement};
use std::collections::HashMap;

pub enum NameKind {
    Let,
    Const,
    Function,    // let f = fn(...) { ... }
    Param,       // fn(x, y) — each parameter
    Import,      // import "math"
    StructName,  // struct Point { ... }
    StructField, // reserved — struct fields live on the struct, not in flat scope
    EnumVariant, // reserved — accessed as Enum.Variant, not as bare names
    ForInKey,    // for k, v in ...  →  k
    ForInValue,  // for k, v in ...  →  v
}

pub struct NameBinding {
    pub name: String,
    pub kind: NameKind,
    pub line: usize,
    pub column: usize,
    pub is_pub: bool,                // true when declared with `pub`
    pub params: Option<Vec<String>>, // parameter names when kind == Function
}

pub type ScopeId = usize;

pub struct Scope {
    pub bindings: Vec<NameBinding>,
    pub parent: Option<ScopeId>,
    pub start: (usize, usize),
    pub end: (usize, usize),
}

pub struct ScopeTree {
    pub scopes: Vec<Scope>,
    pub root: ScopeId,
}

impl ScopeTree {
    pub fn new() -> Self {
        let root = Scope {
            bindings: Vec::new(),
            parent: None,
            start: (0, 0),
            end: (usize::MAX, usize::MAX),
        };
        ScopeTree {
            scopes: vec![root],
            root: 0,
        }
    }
}

impl Default for ScopeTree {
    fn default() -> Self {
        Self::new()
    }
}

impl ScopeTree {
    pub fn push_scope(
        &mut self,
        parent: ScopeId,
        start: (usize, usize),
        end: (usize, usize),
    ) -> ScopeId {
        let id = self.scopes.len();
        self.scopes.push(Scope {
            bindings: vec![],
            parent: Some(parent),
            start,
            end,
        });
        id
    }

    pub fn add_binding(&mut self, scope: ScopeId, binding: NameBinding) {
        self.scopes[scope].bindings.push(binding);
    }

    pub fn name_at(&self, line: usize, col: usize) -> Vec<&NameBinding> {
        let scope_id = self.innermost_scope_at(line, col);
        let mut result = vec![];
        let mut current = Some(scope_id);

        while let Some(id) = current {
            let scope = &self.scopes[id];
            for b in &scope.bindings {
                if b.line < line || (b.line == line && b.column <= col) {
                    result.push(b);
                }
            }
            current = scope.parent;
        }

        result
    }

    fn innermost_scope_at(&self, line: usize, col: usize) -> ScopeId {
        let mut best = self.root;
        let mut best_size = usize::MAX;

        for (id, scope) in self.scopes.iter().enumerate() {
            let (sl, sc) = scope.start;
            let (el, ec) = scope.end;

            let after_start = line > sl || (line == sl && col >= sc);
            let before_end = line < el || (line == el && col <= ec);

            if after_start && before_end {
                let size = (el.saturating_sub(sl))
                    .saturating_mul(10000)
                    .saturating_add(ec);
                if size < best_size {
                    best = id;
                    best_size = size;
                }
            }
        }

        best
    }
}

pub struct ScopeAnalyzer {
    pub tree: ScopeTree,
    current: ScopeId,
}

impl ScopeAnalyzer {
    pub fn new() -> Self {
        let tree = ScopeTree::new();
        let root = tree.root;
        ScopeAnalyzer {
            tree,
            current: root,
        }
    }
}

impl Default for ScopeAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

impl ScopeAnalyzer {
    pub fn analyze(program: &Program) -> ScopeTree {
        let mut analyzer = ScopeAnalyzer::new();
        analyzer.visit_program(program);
        analyzer.tree
    }

    fn add(&mut self, binding: NameBinding) {
        self.tree.add_binding(self.current, binding);
    }

    fn enter_scope(&mut self, start: (usize, usize), end: (usize, usize)) -> ScopeId {
        let prev = self.current;
        let new_id = self.tree.push_scope(self.current, start, end);
        self.current = new_id;
        prev
    }

    fn leave_scope(&mut self, saved: ScopeId) {
        self.current = saved;
    }
}

impl Visitor for ScopeAnalyzer {
    fn visit_statement(&mut self, stmt: &Statement) {
        match stmt {
            Statement::Block {
                statements,
                line,
                column,
                end_line,
                end_column,
                ..
            } => {
                let saved = self.enter_scope((*line, *column), (*end_line, *end_column));
                for s in statements {
                    self.visit_statement(s);
                }
                self.leave_scope(saved);
            }

            // Pub wraps a Let or Const and makes the binding visible to importers.
            // We handle the binding here directly so visit_let / visit_const
            // are NOT called afterward (which would record the name a second time).
            Statement::Pub {
                statement,
                line,
                column,
                ..
            } => match statement.as_ref() {
                Statement::Let { pattern, value, .. } => {
                    self.record_let(pattern, value, *line, *column, true);
                    self.visit_expression(value);
                }
                Statement::Const { pattern, value, .. } => {
                    if let LetPattern::Ident(name) = pattern {
                        self.add(NameBinding {
                            name: name.clone(),
                            kind: NameKind::Const,
                            line: *line,
                            column: *column,
                            is_pub: true,
                            params: None,
                        });
                    }
                    self.visit_expression(value);
                }
                _ => walk_statement(self, statement),
            },

            _ => walk_statement(self, stmt),
        }
    }

    fn visit_expression(&mut self, expr: &Expression) {
        match expr {
            Expression::Function {
                parameter,
                body,
                line,
                column,
                end_line,
                end_column,
                ..
            } => {
                let saved = self.enter_scope((*line, *column), (*end_line, *end_column));
                for param in parameter {
                    self.add(NameBinding {
                        name: param.name.clone(),
                        kind: NameKind::Param,
                        line: *line,
                        column: *column,
                        is_pub: false,
                        params: None,
                    });
                }
                self.visit_statement(body);
                self.leave_scope(saved);
            }

            // for k, v in iterable { body }
            // The iterable is evaluated in the outer scope.
            // k and v are only visible inside the loop body.
            Expression::ForIn {
                key,
                value,
                iterable,
                body,
                line,
                column,
                end_line,
                end_column,
                ..
            } => {
                self.visit_expression(iterable);
                let saved = self.enter_scope((*line, *column), (*end_line, *end_column));
                self.add(NameBinding {
                    name: key.clone(),
                    kind: NameKind::ForInKey,
                    line: *line,
                    column: *column,
                    is_pub: false,
                    params: None,
                });
                if let Some(v) = value {
                    self.add(NameBinding {
                        name: v.clone(),
                        kind: NameKind::ForInValue,
                        line: *line,
                        column: *column,
                        is_pub: false,
                        params: None,
                    });
                }
                self.visit_statement(body);
                self.leave_scope(saved);
            }

            _ => walk_expression(self, expr),
        }
    }

    fn visit_let(&mut self, pattern: &LetPattern, value: &Expression, line: usize, col: usize) {
        self.record_let(pattern, value, line, col, false);
    }

    fn visit_const(&mut self, pattern: &LetPattern, _value: &Expression, line: usize, col: usize) {
        if let LetPattern::Ident(name) = pattern {
            self.add(NameBinding {
                name: name.clone(),
                kind: NameKind::Const,
                line,
                column: col,
                is_pub: false,
                params: None,
            });
        }
    }

    fn visit_import(&mut self, path: &str, line: usize, col: usize) {
        let name = path.split('/').next_back().unwrap_or(path).to_string();
        self.add(NameBinding {
            name,
            kind: NameKind::Import,
            line,
            column: col,
            is_pub: false,
            params: None,
        });
    }

    fn visit_struct_decl(&mut self, name: &Expression, _fields: &HashMap<String, Expression>) {
        if let Expression::Ident {
            value,
            line,
            column,
            ..
        } = name
        {
            self.add(NameBinding {
                name: value.clone(),
                kind: NameKind::StructName,
                line: *line,
                column: *column,
                is_pub: false,
                params: None,
            });
        }
    }

    fn visit_enum_decl(&mut self, name: &str, _variants: &[String], line: usize, col: usize) {
        self.add(NameBinding {
            name: name.to_string(),
            kind: NameKind::Const,
            line,
            column: col,
            is_pub: false,
            params: None,
        });
    }
}

impl ScopeAnalyzer {
    // Shared logic for visit_let and the Pub handler.
    // Detects whether the value is a function so the binding gets
    // NameKind::Function and its parameter names recorded.
    fn record_let(
        &mut self,
        pattern: &LetPattern,
        value: &Expression,
        line: usize,
        col: usize,
        is_pub: bool,
    ) {
        let (kind, params) = match value {
            Expression::Function { parameter, .. } => (
                NameKind::Function,
                Some(parameter.iter().map(|p| p.name.clone()).collect()),
            ),
            _ => (NameKind::Let, None),
        };

        match pattern {
            LetPattern::Ident(name) => {
                self.add(NameBinding {
                    name: name.clone(),
                    kind,
                    line,
                    column: col,
                    is_pub,
                    params,
                });
            }
            LetPattern::Array(names) => {
                for n in names {
                    self.add(NameBinding {
                        name: n.clone(),
                        kind: NameKind::Let,
                        line,
                        column: col,
                        is_pub,
                        params: None,
                    });
                }
            }
            LetPattern::Hash(pairs) => {
                for (_, alias) in pairs {
                    self.add(NameBinding {
                        name: alias.clone(),
                        kind: NameKind::Let,
                        line,
                        column: col,
                        is_pub,
                        params: None,
                    });
                }
            }
        }
    }
}
