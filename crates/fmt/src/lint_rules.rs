use std::collections::{HashMap, HashSet};

use code_lang::ast::{ast::{Expression, LetPattern}, walk::{Visitor, walk_expression}};

pub enum LintSeverity { Error, Warning, Info }

pub struct LintDiagnostic {
    pub rule: &'static str,
    pub message: String,
    pub line: usize,
    pub colum: usize,
    pub severity: LintSeverity
}

pub trait LintRule {
    fn diagnostic(&mut self) -> &[LintDiagnostic];
}

//rule for unused import
pub struct UnusedImport {
    imported: Vec<(String, usize, usize)>,
    referenced: HashSet<String>,
    diags: Vec<LintDiagnostic>
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
        let name = path.split('/').last().unwrap_or(path);
        self.imported.push((name.to_string(), line, col));
    }

    fn visit_member(&mut self, object: &code_lang::ast::ast::Expression, _property: &code_lang::ast::ast::Expression,
                    _line: usize, _col: usize)
    {
        if let Expression::Ident { value, .. } = object {
            self.referenced.insert(value.clone());
        }
    }
}

impl LintRule for UnusedImport {
    fn diagnostic(&mut self) -> &[LintDiagnostic] {
        for (name, line, col) in &self.imported {
            if !self.referenced.contains(name){
                self.diags.push(LintDiagnostic{
                    rule:"unused-import",
                    message: format!("'{}' is imported but never used", name),
                    line: *line,
                    colum: *col,
                    severity: LintSeverity::Warning,
                });
            }
        }

        &self.diags
    }
}

//rule for shadowedBinding
pub struct ShadowedBinding {
    scope_stack: Vec<HashMap<String, (usize, usize)>>,
    diags: Vec<LintDiagnostic>
}

impl ShadowedBinding {
    pub fn new() -> Self {
        ShadowedBinding {
            scope_stack: vec![HashMap::new()],
            diags: Vec::new(),
        }
    }
}

impl Visitor for ShadowedBinding {
    fn visit_block(&mut self, stmts: &[code_lang::ast::ast::Statement], _line: usize, _col: usize) {
        self.scope_stack.push(HashMap::new());
        for stmt in stmts { self.visit_statement(stmt);}
        self.scope_stack.pop();
    }

    fn visit_let(&mut self, pattern: &code_lang::ast::ast::LetPattern, value: &Expression,line: usize, col: usize) {
        walk_expression(self, value);
        if let LetPattern::Ident(name) = pattern {
            let current_scope = self.scope_stack.last_mut().unwrap();
            if let Some((prev_line, prev_col)) = current_scope.get(name) {
                self.diags.push(LintDiagnostic{
                    rule:"shadowed-binding",
                    message:format!(
                        "'{}'shadows an earlier binding at {}:{}",
                        name, prev_line, prev_col
                    ),
                    line, colum:col,
                    severity:LintSeverity::Warning,
                });
            }else {
                current_scope.insert(name.clone(), (line, col));
            }
        }
    }
}

impl LintRule for ShadowedBinding {
    fn diagnostic(&mut self) -> &[LintDiagnostic] {
        &self.diags
    }
}
