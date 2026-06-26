use std::collections::{HashMap, HashSet};

use code_lang::{
    analysis::scope::ScopeTree,
    ast::{
        ast::{Expression, LetPattern, Statement},
        walk::{Visitor, walk_expression, walk_statement},
    },
    token::token::TokenType,
};

#[allow(dead_code)]
pub enum LintSeverity {
    Error,
    Warning,
    Info,
}
pub struct LintFix {
    pub start_line: usize,
    pub start_col: usize,
    pub end_col: usize,
    #[allow(dead_code)]
    pub end_line: usize,
    pub replacement: String,
}

pub struct LintDiagnostic {
    pub rule: &'static str,
    pub message: String,
    pub line: usize,
    pub colum: usize,
    pub severity: LintSeverity,
    pub fix: Option<LintFix>,
}

pub trait LintRule {
    fn diagnostic(&mut self) -> &[LintDiagnostic];
}

//rule for unused import
pub struct UnusedImport {
    imported: Vec<(String, usize, usize)>,
    referenced: HashSet<String>,
    diags: Vec<LintDiagnostic>,
}

impl UnusedImport {
    pub fn new() -> Self {
        UnusedImport {
            imported: Vec::new(),
            referenced: HashSet::new(),
            diags: Vec::new(),
        }
    }
}

impl Visitor for UnusedImport {
    fn visit_import(&mut self, path: &str, line: usize, col: usize) {
        let name = path.split('/').next_back().unwrap_or(path);
        self.imported.push((name.to_string(), line, col));
    }

    fn visit_member(
        &mut self,
        object: &code_lang::ast::ast::Expression,
        _property: &code_lang::ast::ast::Expression,
        _line: usize,
        _col: usize,
    ) {
        if let Expression::Ident { value, .. } = object {
            self.referenced.insert(value.clone());
        }
    }

    fn visit_ident(&mut self, value: &str, _line: usize, _col: usize) {
        self.referenced.insert(value.to_string());
    }
}

impl LintRule for UnusedImport {
    fn diagnostic(&mut self) -> &[LintDiagnostic] {
        if !self.diags.is_empty() {
            return &self.diags;
        }

        for (name, line, col) in &self.imported {
            if !self.referenced.contains(name) {
                self.diags.push(LintDiagnostic {
                    rule: "unused-import",
                    message: format!("'{}' is imported but never used", name),
                    line: *line,
                    colum: *col,
                    severity: LintSeverity::Warning,
                    fix: Some(LintFix {
                        start_line: *line,
                        start_col: 0,
                        end_line: usize::MAX,
                        end_col: usize::MAX,
                        replacement: "".to_string(),
                    }),
                });
            }
        }

        &self.diags
    }
}

//--------------------------------- rule for shadowedBinding --------------------------------------------------------
pub struct ShadowedBinding {
    scope_stack: Vec<HashMap<String, (usize, usize)>>,
    diags: Vec<LintDiagnostic>,
}

impl ShadowedBinding {
    pub fn new() -> Self {
        ShadowedBinding {
            scope_stack: vec![HashMap::new()],
            diags: Vec::new(),
        }
    }
}

impl ShadowedBinding {
    fn register_pattern(&mut self, pattern: &LetPattern, line: usize, col: usize) {
        let names: Vec<String> = match pattern {
            LetPattern::Ident(n) => vec![n.clone()],
            LetPattern::Array(names) => names.clone(),
            LetPattern::Hash(pairs) => pairs.iter().map(|(_, alias)| alias.clone()).collect(),
        };

        for name in names {
            let scope = self.scope_stack.last_mut().unwrap();
            if let Some((prev_line, prev_col)) = scope.get(&name) {
                self.diags.push(LintDiagnostic {
                    rule: "shadowed-binding",
                    message: format!(
                        "'{}' shadows an earlier binding at {}:{}",
                        name, prev_line, prev_col
                    ),
                    line,
                    colum: col,
                    severity: LintSeverity::Warning,
                    fix: None,
                });
            } else {
                scope.insert(name, (line, col));
            }
        }
    }
}

impl Visitor for ShadowedBinding {
    fn visit_statement(&mut self, stmt: &Statement) {
        if let Statement::Block { statements, .. } = stmt {
            self.scope_stack.push(HashMap::new());
            for s in statements {
                self.visit_statement(s);
            }
            self.scope_stack.pop();
        } else {
            walk_statement(self, stmt);
        }
    }

    fn visit_let(&mut self, pattern: &LetPattern, _value: &Expression, line: usize, col: usize) {
        self.register_pattern(pattern, line, col);
    }

    fn visit_const(&mut self, pattern: &LetPattern, _value: &Expression, line: usize, col: usize) {
        self.register_pattern(pattern, line, col);
    }

    fn visit_expression(&mut self, expr: &Expression) {
        
    }
}

impl LintRule for ShadowedBinding {
    fn diagnostic(&mut self) -> &[LintDiagnostic] {
        &self.diags
    }
}

// ---------------------------------- unused variable -------------------------------
pub struct UnusedVariable {
    // (line, col, is_ident) — is_ident=true means plain `let x`, fix is safe
    declared: HashMap<String, (usize, usize, bool)>,
    used: HashSet<String>,
    diags: Vec<LintDiagnostic>,
}

impl UnusedVariable {
    pub fn new() -> Self {
        UnusedVariable {
            declared: HashMap::new(),
            used: HashSet::new(),
            diags: vec![],
        }
    }
}

impl Visitor for UnusedVariable {
    fn visit_let(&mut self, pattern: &LetPattern, _value: &Expression, line: usize, col: usize) {
        match pattern {
            LetPattern::Ident(n) => {
                self.declared.insert(n.clone(), (line, col, true));
            }
            LetPattern::Array(names) => {
                for name in names {
                    self.declared.insert(name.clone(), (line, col, false));
                }
            }
            LetPattern::Hash(pairs) => {
                for (_, alias) in pairs {
                    self.declared.insert(alias.clone(), (line, col, false));
                }
            }
        }
    }

    fn visit_const(
        &mut self,
        pattern: &LetPattern,
        _value: &Expression,
        line: usize,
        col: usize,
    )
    {
        match pattern {
            LetPattern::Array(names) => for name in names { self.declared.insert(name.to_string(), (line, col, false));},
            LetPattern::Hash(pairs) => for (_, alias) in pairs  { self.declared.insert(alias.to_string(), (line, col, false));},
            LetPattern::Ident(n) => { self.declared.insert(n.to_string(), (line,col, false));}
        }
    }

    fn visit_ident(&mut self, value: &str, _line: usize, _col: usize) {
        self.used.insert(value.to_string());
    }
}

impl LintRule for UnusedVariable {
    fn diagnostic(&mut self) -> &[LintDiagnostic] {
        if !self.diags.is_empty() {
            return &self.diags;
        };

        for (name, (line, col, is_ident)) in &self.declared {
            if !self.used.contains(&name.to_string()) {
                // fix is only reliable for plain `let x` — for destructuring
                // the name position inside {..} or [..] is not stored in the AST
                let fix = if *is_ident {
                    Some(LintFix {
                        start_line: *line,
                        start_col: col + 3,
                        end_line: *line,
                        end_col: col + 3 + name.len(),
                        replacement: format!("_{}", name),
                    })
                } else {
                    None
                };
                self.diags.push(LintDiagnostic {
                    rule: "unused-variable",
                    message: format!("'{}' is declared but never used", name),
                    line: *line,
                    colum: *col,
                    severity: LintSeverity::Warning,
                    fix,
                })
            }
        }

        &self.diags
    }
}

// ---------------------------------- rule const reassignment --------------------------------------

pub struct ConstReassignment {
    consts: HashSet<String>,
    diags: Vec<LintDiagnostic>,
}

impl ConstReassignment {
    pub fn new() -> Self {
        ConstReassignment {
            consts: HashSet::new(),
            diags: vec![],
        }
    }
}

impl Visitor for ConstReassignment {
    fn visit_const(
        &mut self,
        pattern: &LetPattern,
        _value: &Expression,
        _line: usize,
        _col: usize,
    ) {
        let names: Vec<String> = match pattern {
            LetPattern::Ident(n) => vec![n.clone()],
            LetPattern::Array(names) => names.clone(),
            LetPattern::Hash(pairs) => pairs.iter().map(|(_, alias)| alias.clone()).collect(),
        };

        for name in names {
            self.consts.insert(name);
        }
    }

    fn visit_infix(
        &mut self,
        op: TokenType,
        left: &Expression,
        _right: &Expression,
        line: usize,
        col: usize,
    ) {
        if let TokenType::Assign = op
            && let Expression::Ident { value: name, .. } = left
            && self.consts.contains(name)
        {
            self.diags.push(LintDiagnostic {
                rule: "const-reassignment",
                message: format!("cannot reassign const '{}'", name),
                line,
                colum: col,
                severity: LintSeverity::Error,
                fix: None,
            });
        }
    }

    fn visit_update(
        &mut self,
        operator: &code_lang::token::token::Token,
        target: &Expression,
        line: usize,
        col: usize,
    ) {
        if let Expression::Ident { value: name, .. } = target
            && self.consts.contains(name)
        {
            self.diags.push(LintDiagnostic {
                rule: "const-reassignment",
                message: format!(
                    "cannot update const '{}' with operator {:?}",
                    name, operator.token_type
                ),
                line,
                colum: col,
                severity: LintSeverity::Error,
                fix: None,
            });
        }
    }
}

impl LintRule for ConstReassignment {
    fn diagnostic(&mut self) -> &[LintDiagnostic] {
        &self.diags
    }
}

// ---------------------------------- DeadCode ------------------------------------
pub struct DeadCode {
    returned: bool,
    diags: Vec<LintDiagnostic>,
}

impl DeadCode {
    pub fn new() -> Self {
        DeadCode {
            returned: false,
            diags: vec![],
        }
    }
}

impl Visitor for DeadCode {
    fn visit_statement(&mut self, stmt: &Statement) {
        if self.returned {
            self.diags.push(LintDiagnostic {
                rule: "dead-code",
                severity: LintSeverity::Warning,
                fix: None,
                message: "unreachable code".to_string(),
                line: 0,
                colum: 0,
            });
            return;
        }

        if let Statement::Return { .. } = stmt {
            walk_statement(self, stmt);
            self.returned = true;
            return;
        }

        if let Statement::Block { statements, .. } = stmt {
            let saved = self.returned;
            self.returned = false;

            for s in statements {
                self.visit_statement(s);
            }

            self.returned = saved;
        }

        walk_statement(self, stmt);
    }
}

impl LintRule for DeadCode {
    fn diagnostic(&mut self) -> &[LintDiagnostic] {
        &self.diags
    }
}

// --------------------------------- rule empty block ------------------------------------
pub struct EmptyBlock {
    diags: Vec<LintDiagnostic>,
}

impl EmptyBlock {
    pub fn new() -> Self {
        EmptyBlock { diags: vec![] }
    }
}

impl Visitor for EmptyBlock {
    fn visit_expression(&mut self, expr: &Expression) {
        if let Expression::If { consequence, .. } = expr
            && let Statement::Block {
                statements,
                line,
                column,
                ..
            } = &**consequence
            && statements.is_empty()
        {
            self.diags.push(LintDiagnostic {
                severity: LintSeverity::Warning,
                line: *line,
                colum: *column,
                rule: "empty-block",
                message: "empty if body - did you forget to add statements".to_string(),
                fix: None,
            });
        }

        if let Expression::While { body, .. } = expr
            && let Statement::Block {
                statements,
                line,
                column,
                ..
            } = &**body
            && statements.is_empty()
        {
            self.diags.push(LintDiagnostic {
                severity: LintSeverity::Warning,
                line: *line,
                colum: *column,
                rule: "empty-block",
                message: "empty while body - did you forget to add statements".to_string(),
                fix: None,
            });
        }

        if let Expression::For { body, .. } = expr
            && let Statement::Block {
                statements,
                line,
                column,
                ..
            } = &**body
            && statements.is_empty()
        {
            self.diags.push(LintDiagnostic {
                severity: LintSeverity::Warning,
                line: *line,
                colum: *column,
                rule: "empty-block",
                message: "empty for body - did you forget to add statements".to_string(),
                fix: None,
            });
        }

        if let Expression::Function { body, .. } = expr
            && let Statement::Block {
                statements,
                line,
                column,
                ..
            } = &**body
            && statements.is_empty()
        {
            self.diags.push(LintDiagnostic {
                severity: LintSeverity::Warning,
                line: *line,
                colum: *column,
                rule: "empty-block",
                message: "empty function body - did you forget to add statements".to_string(),
                fix: None,
            });
        }

        if let Expression::ForIn { body, .. } = expr
            && let Statement::Block {
                statements,
                line,
                column,
                ..
            } = &**body
            && statements.is_empty()
        {
            self.diags.push(LintDiagnostic {
                severity: LintSeverity::Warning,
                line: *line,
                colum: *column,
                rule: "empty-block",
                message: "empty for-in body - did you forget to add statements".to_string(),
                fix: None,
            });
        }

        walk_expression(self, expr);
    }
}

impl LintRule for EmptyBlock {
    fn diagnostic(&mut self) -> &[LintDiagnostic] {
        &self.diags
    }
}

// ----------------------------------- undefined variable ----------------------------------------
const BUILTINS: &[&str] = &[
    "fmt", "arrays", "math", "strings", "hash", "json", "fs", "os", "path", "rand", "time", "http",
    "true", "false", "null", "self",
];

pub struct UndefinedVariable {
    scope_tree: ScopeTree,
    diags: Vec<LintDiagnostic>,
}

impl UndefinedVariable {
    pub fn new(tree: ScopeTree) -> Self {
        UndefinedVariable {
            scope_tree: tree,
            diags: vec![],
        }
    }
}

impl Visitor for UndefinedVariable {
    fn visit_ident(&mut self, value: &str, line: usize, col: usize) {
        if BUILTINS.contains(&value) {
            return;
        }
        if value.starts_with('_') {
            return;
        }

        let visible = self.scope_tree.name_at(line, col);
        let found = visible.iter().any(|b| b.name == value);
        if !found {
            self.diags.push(LintDiagnostic {
                rule: "undefined-variable",
                severity: LintSeverity::Error,
                line,
                colum: col,
                message: format!("'{}' is not defined", value),
                fix: None,
            });
        }
    }

    fn visit_member(
        &mut self,
        object: &Expression,
        _property: &Expression,
        _line: usize,
        _col: usize,
    )
    {
        self.visit_expression(object);
    }
}

impl LintRule for UndefinedVariable {
    fn diagnostic(&mut self) -> &[LintDiagnostic] {
        &self.diags
    }
}
