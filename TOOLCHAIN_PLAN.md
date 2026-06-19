# Toolchain Build Plan

A detailed, step-by-step plan for building `code-lang-lsp` and
`code-lang-fmt` on top of the interpreter. Each item explains the
problem, what is wrong today, exactly what to change, and pseudo code
for every new function and data structure.

---

## The 10 items in order

| # | Item | Depends on |
|---|---|---|
| 1 | Parser error recovery | nothing |
| 2 | AST end-positions (spans) | nothing |
| 3 | Structured `ParseErrorKind` | nothing |
| 4 | AST visitor / walker trait | nothing |
| 5 | Static scope analysis | 4 |
| 6 | `fmt check` wired up | 1, 3 |
| 7 | `fmt lint` wired up | 3, 4 |
| 8 | LSP diagnostics | 1, 2, 3 |
| 9 | LSP completions | 2, 5 |
| 10 | Install script | nothing |

Items 1, 2, 3, 4 have no dependencies on each other.
**Do them in parallel — they are all isolated changes.**

---

---

# 1 — Parser error recovery

## The problem

Open `src/parser/parser.rs` and look at `parse_program` (line 1110):

```rust
pub fn parse_program(&mut self) -> Program {
    let mut program = Program { statements: vec![] };
    while !self.cur_token_is(TokenType::EOF) {
        if let Some(stmt) = self.parse_statement() {
            program.statements.push(stmt);
        }
        self.next_token();
    }
    program
}
```

And look at almost every parse function — they return `Option<Statement>`
or `Option<Expression>`. When something goes wrong, they push an error
and return `None`. The caller sees `None` and propagates `None` upward
with the `?` operator. Eventually `parse_program` gets `None` from
`parse_statement`, skips the statement, calls `next_token()`, and tries
the next token.

**The bug:** `next_token()` advances by exactly one token. If the
broken statement is `let x = (1 + ;`, the parser fails at `;`, returns
`None`, and `parse_program` calls `next_token()` once — now sitting on
the next statement's first token or somewhere inside the garbage. The
parser is now desynchronized. Everything after the broken statement
either fails to parse or produces garbage AST nodes.

**What this means for tools:**
- `fmt check file.cl` with 5 errors reports only the first one.
  You fix it, run again, get the second one. Fix it, run again…
  This is a terrible experience.
- The LSP re-parses the file on every keystroke. If there is one
  syntax error anywhere, the LSP sees no AST after that point —
  meaning no completions, no hover, no diagnostics past line N.

## What error recovery does

After a parse failure, instead of blindly advancing one token, the
parser **synchronizes** — it skips tokens until it finds a point in
the token stream that looks like the start of a new statement. From
there it can resume parsing correctly.

The set of safe restart tokens are:
- `Semicolon` — end of a statement
- `Let`, `Const` — variable declaration
- `Fn` — function literal at statement level
- `If`, `While`, `For` — control flow
- `Return` — return statement
- `Import` — module import
- `Struct`, `Enum` — type declaration
- `Pub` — visibility modifier
- `RBrace` — end of a block (caller will handle it)
- `EOF` — end of file

## What to add to `parser.rs`

### New method: `synchronize`

Add this private method to the `Parser` impl:

```
fn synchronize(&mut self) {
    // keep consuming tokens until we land on something that looks
    // like the start of a statement, or we hit EOF

    loop {
        match self.cur_token.token_type {

            // a semicolon ends the broken statement —
            // consume it and return so the caller can try the next one
            TokenType::Semicolon => {
                self.next_token();
                return;
            }

            // EOF — stop, nothing left to parse
            TokenType::EOF => return,

            // these all start a new statement —
            // do NOT consume; return and let parse_statement handle it
            TokenType::Let
            | TokenType::Const
            | TokenType::If
            | TokenType::While
            | TokenType::For
            | TokenType::Return
            | TokenType::Import
            | TokenType::Struct
            | TokenType::Enum
            | TokenType::Pub => return,

            // anything else is part of the broken statement — skip it
            _ => {
                self.next_token();
            }
        }
    }
}
```

### Change: `parse_statement`

Currently (line 768) `parse_statement` calls sub-parsers with `?`:

```rust
// current
fn parse_statement(&mut self) -> Option<Statement> {
    match self.cur_token.token_type {
        TokenType::Let    => self.parse_let_statement(),
        TokenType::Const  => self.parse_const_statement(),
        TokenType::Return => self.parse_return_statement(),
        ...
        _                 => self.parse_expression_statement(),
    }
}
```

Change it so that when a sub-parser returns `None`, we call
`synchronize` instead of just returning `None`:

```
fn parse_statement(&mut self) -> Option<Statement> {
    let result = match self.cur_token.token_type {
        TokenType::Let    => self.parse_let_statement(),
        TokenType::Const  => self.parse_const_statement(),
        TokenType::Return => self.parse_return_statement(),
        ...
        _                 => self.parse_expression_statement(),
    };

    if result.is_none() {
        // sub-parser already pushed an error;
        // synchronize to a safe restart point
        self.synchronize();
    }

    result
}
```

### Change: `parse_program`

The loop already skips `None` but calls `next_token()` unconditionally.
After synchronization, the cursor is already on the next safe token —
calling `next_token()` again skips it. Remove the unconditional advance:

```
pub fn parse_program(&mut self) -> Program {
    let mut program = Program { statements: vec![] };

    while !self.cur_token_is(TokenType::EOF) {
        if let Some(stmt) = self.parse_statement() {
            program.statements.push(stmt);
        }
        // only advance if parse_statement did not already advance
        // (synchronize leaves the cursor on a safe token;
        //  successful parses consume their own semicolon)
        //
        // the cleanest way: track whether the token changed
        // OR: always call next_token() but have parse_statement
        //     NOT consume the final semicolon — pick one convention
        // simplest: call next_token() here and have parse_statement
        //           consume up to but NOT including the next statement
        self.next_token();
    }

    program
}
```

> NOTE: There is a token-consumption convention question here.
> Currently most parse functions consume their own trailing semicolon.
> You may need to audit a few of them after adding synchronize to
> make sure the cursor is where parse_program expects it.

## What the output looks like after this change

**Before (today):**
```
$ code-lang-fmt check bad.cl
bad.cl:3:10: expected ASSIGN, got PLUS
```

**After (with recovery):**
```
$ code-lang-fmt check bad.cl
bad.cl:3:10: expected ASSIGN, got PLUS
bad.cl:7:1: unexpected token: }
bad.cl:12:5: identifier not found in expression
3 errors found
```

---

---

# 2 — AST end-positions (spans)

## The problem

Every AST node currently stores only where it **starts**:

```rust
Expression::Ident { value: String, line: usize, column: usize }
```

The LSP protocol works in terms of `Range { start: Position, end: Position }`.
When the LSP publishes a diagnostic it says "this range is an error" and
the editor draws a squiggle under exactly those characters.

Without an end position, the LSP has two bad options:
1. Squiggle from the start of the token to start+1 — one character, barely visible.
2. Squiggle to end of line — often too wide, looks broken.

End positions are also required for:
- **Hover** — user hovers over `math` in `math.sqrt(x)`, editor asks
  "is the cursor inside any node?", server walks AST checking ranges.
- **Go to definition** — same range check.
- **Rename** — find all `Ident` nodes where `value == "x"` and return
  their ranges for the editor to highlight.

## What to change in `ast.rs`

Add `end_line: usize` and `end_col: usize` to every Expression and
Statement variant. Example for the high-value nodes:

```
pub enum Expression {

    Ident {
        value:    String,
        line:     usize,
        column:   usize,
        end_line: usize,   // NEW
        end_col:  usize,   // NEW
    },

    Call {
        function: Box<Expression>,
        argument: Vec<Expression>,
        line:     usize,
        column:   usize,
        end_line: usize,   // NEW — position of closing ')'
        end_col:  usize,   // NEW
    },

    Member {
        object:   Box<Expression>,
        property: Box<Expression>,
        line:     usize,
        column:   usize,
        end_line: usize,   // NEW — position after property name
        end_col:  usize,   // NEW
    },

    Infix {
        left:     Box<Expression>,
        op:       Token,
        right:    Box<Expression>,
        line:     usize,
        column:   usize,
        end_line: usize,   // NEW — end of right operand
        end_col:  usize,   // NEW
    },

    // ... same pattern for all other variants
}

pub enum Statement {

    Let {
        pattern: LetPattern,
        value:   Expression,
        line:    usize,
        column:  usize,
        end_line: usize,   // NEW — position of trailing semicolon
        end_col:  usize,   // NEW
    },

    Import {
        path:    String,
        line:    usize,
        column:  usize,
        end_line: usize,   // NEW
        end_col:  usize,   // NEW
    },

    // ... same pattern
}
```

## What to change in `parser.rs`

For each parse function, record the end position from `self.cur_token`
at the moment the last token of the construct is consumed.

**Pattern for a simple token (Ident):**

```
fn parse_identifier(&mut self) -> Option<Expression> {
    let line   = self.cur_token.line;
    let column = self.cur_token.column;

    let value = match &self.cur_token.token_type {
        TokenType::Ident(v) => v.clone(),
        _                   => return None,
    };

    // end position: same line, column + length of identifier
    let end_line = line;
    let end_col  = column + value.len();

    Some(Expression::Ident { value, line, column, end_line, end_col })
}
```

**Pattern for a compound expression (Call):**

```
fn parse_call_expression(&mut self, function: Expression) -> Option<Expression> {
    let line   = self.cur_token.line;   // position of '('
    let column = self.cur_token.column;

    let args = self.parse_expression_list(TokenType::RParan)?;

    // after parse_expression_list, cur_token is ')'
    let end_line = self.cur_token.line;
    let end_col  = self.cur_token.column + 1;  // +1 to include the ')'

    Some(Expression::Call {
        function: Box::new(function),
        argument: args,
        line, column, end_line, end_col,
    })
}
```

**Pattern for a statement (Let):**

```
fn parse_let_statement(&mut self) -> Option<Statement> {
    let line   = self.cur_token.line;
    let column = self.cur_token.column;

    // ... parse pattern, value ...

    // after consuming the semicolon, cur_token IS the semicolon
    let end_line = self.cur_token.line;
    let end_col  = self.cur_token.column + 1;

    Some(Statement::Let { pattern, value, line, column, end_line, end_col })
}
```

## A helper to make this less repetitive

Add a small helper to the `Parser` that returns the current token's
end position:

```
fn cur_end(&self) -> (usize, usize) {
    let col = self.cur_token.column;
    let len = match &self.cur_token.token_type {
        TokenType::Ident(s)        => s.len(),
        TokenType::Int(_)          => 1,   // approximation; use raw lexeme if stored
        TokenType::StringLit(s)    => s.len() + 2,  // +2 for the quotes
        TokenType::Plus            => 1,
        TokenType::RParan          => 1,
        TokenType::RBrace          => 1,
        TokenType::Semicolon       => 1,
        _                          => 1,
    };
    (self.cur_token.line, col + len)
}
```

Then in each parse function: `let (end_line, end_col) = self.cur_end();`

---

---

# 3 — Structured `ParseErrorKind`

## The problem

Today `ParseError` is:

```rust
pub struct ParseError {
    pub message: String,
    pub line:    usize,
    pub column:  usize,
}
```

`message` is built by string formatting:
`format!("expected {:?}, got {:?}", expected, got)`.

Lint rules and the LSP both need to distinguish error types:
- A `MissingSemicolon` error is a warning in a permissive lint mode.
- An `UnexpectedToken` error is always a hard error.
- An `UnclosedDelimiter` error means everything after is garbage —
  the LSP should suppress diagnostics for the rest of the block.

You cannot distinguish these from the message string reliably.

## What to add

Add a `kind` field to `ParseError`. The message stays — it's still
shown to users. The kind is for programmatic consumers (LSP, lint).

```
// in parser.rs, before the Parser struct

pub enum ParseErrorKind {

    // expected one token type, got a different one
    // example: "expected ASSIGN, got PLUS"
    UnexpectedToken {
        expected: TokenType,
        got:      TokenType,
    },

    // the input ended before the construct was complete
    // example: file ends inside a function body
    UnexpectedEOF,

    // a delimiter was opened but never closed
    // example: "let f = fn(x { x }" — missing ')'
    UnclosedDelimiter {
        open: TokenType,   // the token that was opened
    },

    // a literal value could not be parsed
    // example: a float literal that overflows
    InvalidLiteral {
        raw: String,
    },

    // the parser expected an expression but got something else
    // example: "let x = ;" — semicolon where a value should be
    MissingExpression,

    // a keyword was used in an illegal position
    // example: "break" outside a loop
    IllegalKeyword {
        keyword: TokenType,
    },

    // catch-all for anything not covered above
    Other,
}

pub struct ParseError {
    pub kind:    ParseErrorKind,   // NEW
    pub message: String,
    pub line:    usize,
    pub column:  usize,
}
```

## What to change in `parser.rs`

There is one main helper that creates most errors — `expect_peak`.
Find it (it calls `self.errors.push(ParseError { message: … })`).
Update it to also set `kind`:

```
fn expect_peak(&mut self, t: TokenType) -> bool {
    if self.peak_token_is(&t) {
        self.next_token();
        true
    } else {
        // was:
        //   self.errors.push(ParseError { message: format!(...), ... });
        // now:
        self.errors.push(ParseError {
            kind: ParseErrorKind::UnexpectedToken {
                expected: t.clone(),
                got:      self.peak_token.token_type.clone(),
            },
            message: format!(
                "expected {:?}, got {:?}",
                t,
                self.peak_token.token_type
            ),
            line:   self.peak_token.line,
            column: self.peak_token.column,
        });
        false
    }
}
```

For the EOF case (scattered through the parser where it checks
`TokenType::EOF` unexpectedly):

```
self.errors.push(ParseError {
    kind:    ParseErrorKind::UnexpectedEOF,
    message: "unexpected end of file".to_string(),
    line:    self.cur_token.line,
    column:  self.cur_token.column,
});
```

For unclosed delimiters (e.g. `parse_expression_list` when it hits EOF
before the closing bracket):

```
self.errors.push(ParseError {
    kind: ParseErrorKind::UnclosedDelimiter {
        open: TokenType::LParan,  // or LBracket, LBrace
    },
    message: "unclosed '(' — expected ')'".to_string(),
    line:   open_line,
    column: open_col,
});
```

## How tools use ParseErrorKind

**In `fmt lint`:**
```
for error in &parser.errors {
    match error.kind {
        ParseErrorKind::UnexpectedToken { .. } =>
            report as ERROR,
        ParseErrorKind::MissingExpression =>
            report as WARNING with hint "did you forget a value?",
        ParseErrorKind::UnclosedDelimiter { open } =>
            report as ERROR with hint "close the opening {open:?}",
        _ =>
            report as ERROR,
    }
}
```

**In the LSP:**
```
let severity = match error.kind {
    ParseErrorKind::UnclosedDelimiter { .. } => DiagnosticSeverity::ERROR,
    ParseErrorKind::MissingExpression        => DiagnosticSeverity::WARNING,
    _                                        => DiagnosticSeverity::ERROR,
};
```

---

---

# 4 — AST visitor / walker trait

## The problem

Every tool that needs to analyze the AST — lint rules, scope analysis,
dead code detection, unused variable warnings — has to write the same
recursive match over `Statement` and `Expression` by hand.

That match is ~100 lines, covers all 25+ node variants, and recurses
into children. If you write it in `crates/fmt/src/lint.rs` AND in
`src/analysis/scope.rs`, you have the same 100 lines in two places.
When a new AST node is added (like `NullCoalesce` was in v0.2.2),
both copies must be updated. One will be forgotten. The analyses will
diverge.

The solution is a **Visitor trait** with default implementations that
walk into children automatically. Each tool only overrides the node
types it actually cares about.

## New file: `src/ast/walk.rs`

```
use crate::ast::ast::{
    Expression, Statement, Program, LetPattern, Param, SwitchArm, StringSegment,
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
    fn visit_let(&mut self, _pattern: &LetPattern, _value: &Expression,
                 _line: usize, _col: usize) {}
    fn visit_const(&mut self, _pattern: &LetPattern, _value: &Expression,
                   _line: usize, _col: usize) {}
    fn visit_return(&mut self, _value: &Expression, _line: usize, _col: usize) {}
    fn visit_import(&mut self, _path: &str, _line: usize, _col: usize) {}
    fn visit_block(&mut self, _stmts: &[Statement], _line: usize, _col: usize) {}
    fn visit_enum_decl(&mut self, _name: &str, _variants: &[String],
                       _line: usize, _col: usize) {}
    fn visit_struct_decl(&mut self, _name: &Expression,
                         _fields: &std::collections::HashMap<String, Expression>) {}

    // Expression-level hooks
    fn visit_ident(&mut self, _value: &str, _line: usize, _col: usize) {}
    fn visit_int(&mut self, _value: isize, _line: usize, _col: usize) {}
    fn visit_float(&mut self, _value: f64, _line: usize, _col: usize) {}
    fn visit_bool(&mut self, _value: bool, _line: usize, _col: usize) {}
    fn visit_null(&mut self, _line: usize, _col: usize) {}
    fn visit_call(&mut self, _fn_expr: &Expression, _args: &[Expression],
                  _line: usize, _col: usize) {}
    fn visit_member(&mut self, _object: &Expression, _property: &Expression,
                    _line: usize, _col: usize) {}
    fn visit_infix(&mut self, _left: &Expression, _right: &Expression,
                   _line: usize, _col: usize) {}
    fn visit_function(&mut self, _params: &[Param], _body: &Statement,
                      _line: usize, _col: usize) {}
    fn visit_if(&mut self, _condition: &Expression, _consequence: &Statement,
                _line: usize, _col: usize) {}
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

        Statement::Let { pattern, value, line, column, .. } => {
            v.visit_let(pattern, value, *line, *column);
            v.visit_expression(value);
        }

        Statement::Const { pattern, value, line, column, .. } => {
            v.visit_const(pattern, value, *line, *column);
            v.visit_expression(value);
        }

        Statement::Return { value, line, column, .. } => {
            v.visit_return(value, *line, *column);
            v.visit_expression(value);
        }

        Statement::Import { path, line, column, .. } => {
            v.visit_import(path, *line, *column);
            // no children
        }

        Statement::Block { statements, line, column, .. } => {
            v.visit_block(statements, *line, *column);
            for s in statements {
                v.visit_statement(s);
            }
        }

        Statement::Expression { expr, .. } => {
            v.visit_expression(expr);
        }

        Statement::Enum { name, variant, line, column, .. } => {
            v.visit_enum_decl(name, variant, *line, *column);
        }

        Statement::Struct { name, field } => {
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

        Expression::Ident { value, line, column, .. } => {
            v.visit_ident(value, *line, *column);
        }

        Expression::Int { value, line, column, .. } => {
            v.visit_int(*value, *line, *column);
        }

        Expression::Float { value, line, column, .. } => {
            v.visit_float(*value, *line, *column);
        }

        Expression::Boolean { value, line, column, .. } => {
            v.visit_bool(*value, *line, *column);
        }

        Expression::Null { line, column, .. } => {
            v.visit_null(*line, *column);
        }

        Expression::Call { function, argument, line, column, .. } => {
            v.visit_call(function, argument, *line, *column);
            v.visit_expression(function);
            for arg in argument {
                v.visit_expression(arg);
            }
        }

        Expression::Member { object, property, line, column, .. } => {
            v.visit_member(object, property, *line, *column);
            v.visit_expression(object);
            v.visit_expression(property);
        }

        Expression::Infix { left, right, line, column, .. } => {
            v.visit_infix(left, right, *line, *column);
            v.visit_expression(left);
            v.visit_expression(right);
        }

        Expression::Prefix { right, .. } => {
            v.visit_expression(right);
        }

        Expression::Function { parameter, body, line, column, .. } => {
            v.visit_function(parameter, body, *line, *column);
            v.visit_statement(body);
        }

        Expression::If { condition, consequence, alternative, if_else, line, column, .. } => {
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

        Expression::While { condition, body, .. } => {
            v.visit_expression(condition);
            v.visit_statement(body);
        }

        Expression::For { init, condition, post, body, .. } => {
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

        Expression::Update { target, .. } => {
            v.visit_expression(target);
        }

        Expression::Char { .. } => {
            // leaf, no children
        }
    }
}
```

## Register it in `src/lib.rs`

```
// src/lib.rs
pub mod ast;       // already there
// inside ast, add:
// pub mod walk;
```

Or add a `src/ast/mod.rs` that re-exports:
```
// src/ast/mod.rs  (new file, or inline in lib.rs)
pub mod ast;
pub mod walk;
```

## How a lint rule looks with the Visitor

```
// example: detect 'let x;' where x is never read
struct UnusedVariableLint {
    declared: Vec<(String, usize, usize)>,   // (name, line, col)
    used:     HashSet<String>,
    diags:    Vec<LintDiagnostic>,
}

impl Visitor for UnusedVariableLint {

    fn visit_let(&mut self, pattern, _value, line, col) {
        if let LetPattern::Ident(name) = pattern {
            self.declared.push((name.clone(), line, col));
        }
    }

    fn visit_ident(&mut self, value, _line, _col) {
        self.used.insert(value.to_string());
    }
}

// after walking the whole program:
fn finish(&mut self) {
    for (name, line, col) in &self.declared {
        if !self.used.contains(name) {
            self.diags.push(LintDiagnostic {
                message:  format!("variable '{}' is declared but never used", name),
                line:     *line,
                column:   *col,
                severity: LintSeverity::Warning,
            });
        }
    }
}
```

---

---

# 5 — Static scope analysis

## The problem

The LSP must offer completions without running the program.

When you type `let f = fn(x) { x.` and pause, the editor sends the
server a completion request at the cursor position. The server needs
to know: "at this position, inside this function body, what names are
in scope?" The answer is: `x` (the parameter) plus everything defined
at the outer scope.

The evaluator `Environment` struct does track this — but it only
exists at runtime, after executing statements one by one. For
completions the server needs this information **statically**, from the
AST alone, without running a single statement.

## New file: `src/analysis/scope.rs`

### Data structures

```
pub enum NameKind {
    Let,           // let x = ...
    Const,         // const X = ...
    Param,         // fn(x) — parameter
    Import,        // import "math" — the module name
    StructField,   // field inside a struct literal
    EnumVariant,   // Color.Red — the variant name
}

pub struct NameBinding {
    pub name:   String,
    pub kind:   NameKind,
    pub line:   usize,
    pub column: usize,
    // for functions: their parameter names, useful for signature help
    pub params: Option<Vec<String>>,
}

// A single lexical scope — corresponds to one block { ... }
pub struct Scope {
    pub bindings: Vec<NameBinding>,
    pub parent:   Option<ScopeId>,
    pub start:    (usize, usize),    // (line, col) where this scope opens
    pub end:      (usize, usize),    // (line, col) where it closes
}

pub type ScopeId = usize;

pub struct ScopeTree {
    pub scopes: Vec<Scope>,
    pub root:   ScopeId,
}

impl ScopeTree {
    pub fn new() -> Self {
        let root = Scope {
            bindings: vec![],
            parent:   None,
            start:    (0, 0),
            end:      (usize::MAX, usize::MAX),
        };
        ScopeTree { scopes: vec![root], root: 0 }
    }

    // create a child scope and return its id
    pub fn push_scope(&mut self, parent: ScopeId,
                      start: (usize, usize), end: (usize, usize)) -> ScopeId {
        let id = self.scopes.len();
        self.scopes.push(Scope {
            bindings: vec![],
            parent:   Some(parent),
            start,
            end,
        });
        id
    }

    // add a binding to a specific scope
    pub fn add_binding(&mut self, scope: ScopeId, binding: NameBinding) {
        self.scopes[scope].bindings.push(binding);
    }

    // given a cursor (line, col), find the innermost scope that
    // contains it, then collect all bindings visible from there
    // (own scope + all ancestors up to root)
    pub fn names_at(&self, line: usize, col: usize) -> Vec<&NameBinding> {
        let scope_id = self.innermost_scope_at(line, col);
        let mut result = vec![];
        let mut current = Some(scope_id);
        while let Some(id) = current {
            let scope = &self.scopes[id];
            // include bindings declared BEFORE the cursor
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
            let before_end  = line < el || (line == el && col <= ec);

            if after_start && before_end {
                // size = rough measure of how small this scope is
                let size = (el - sl) * 10000 + ec;
                if size < best_size {
                    best = id;
                    best_size = size;
                }
            }
        }

        best
    }
}
```

### The analyzer: `ScopeAnalyzer`

```
// implements Visitor from item 4

pub struct ScopeAnalyzer {
    pub tree:    ScopeTree,
    current:     ScopeId,
}

impl ScopeAnalyzer {

    pub fn new() -> Self {
        let tree = ScopeTree::new();
        let root = tree.root;
        ScopeAnalyzer { tree, current: root }
    }

    pub fn analyze(program: &Program) -> ScopeTree {
        let mut analyzer = ScopeAnalyzer::new();
        analyzer.visit_program(program);
        analyzer.tree
    }

    fn add(&mut self, binding: NameBinding) {
        self.tree.add_binding(self.current, binding);
    }

    fn enter_scope(&mut self, start: (usize, usize), end: (usize, usize)) -> ScopeId {
        let prev    = self.current;
        let new_id  = self.tree.push_scope(self.current, start, end);
        self.current = new_id;
        prev
    }

    fn leave_scope(&mut self, saved: ScopeId) {
        self.current = saved;
    }
}

impl Visitor for ScopeAnalyzer {

    fn visit_let(&mut self, pattern, value, line, col) {
        // first walk the value (right side of =) so inner functions
        // are analyzed before we bind the name in the outer scope
        walk_expression(self, value);

        match pattern {
            LetPattern::Ident(name) => {
                self.add(NameBinding { name: name.clone(), kind: NameKind::Let, line, column: col, params: None });
            }
            LetPattern::Array(names) => {
                for n in names {
                    self.add(NameBinding { name: n.clone(), kind: NameKind::Let, line, column: col, params: None });
                }
            }
            LetPattern::Hash(pairs) => {
                for (_, alias) in pairs {
                    self.add(NameBinding { name: alias.clone(), kind: NameKind::Let, line, column: col, params: None });
                }
            }
        }
        // do NOT call walk_expression again (we already did it above)
    }

    fn visit_const(&mut self, pattern, value, line, col) {
        // same as visit_let but NameKind::Const
        walk_expression(self, value);
        if let LetPattern::Ident(name) = pattern {
            self.add(NameBinding { name: name.clone(), kind: NameKind::Const, line, column: col, params: None });
        }
    }

    fn visit_import(&mut self, path, line, col) {
        // the module name is the last segment of the path
        // e.g. "math" -> "math", "utils" -> "utils"
        let name = path.split('/').last().unwrap_or(path).to_string();
        self.add(NameBinding { name, kind: NameKind::Import, line, column: col, params: None });
    }

    fn visit_function(&mut self, params, body, line, col) {
        // functions open a new scope
        let end   = /* body end position — needs item 2 */ (line + 100, 0);
        let saved = self.enter_scope((line, col), end);

        // add parameters to the new scope
        let param_names: Vec<String> = params.iter().map(|p| p.name.clone()).collect();
        for param in params {
            if param.name == "self" { continue; }   // skip implicit self
            self.add(NameBinding {
                name:   param.name.clone(),
                kind:   NameKind::Param,
                line,
                column: col,
                params: None,
            });
        }

        // walk the body inside the new scope
        walk_statement(self, body);

        self.leave_scope(saved);
    }

    fn visit_block(&mut self, stmts, line, col) {
        let end   = /* body end position — needs item 2 */ (line + 100, 0);
        let saved = self.enter_scope((line, col), end);

        for stmt in stmts {
            walk_statement(self, stmt);
        }

        self.leave_scope(saved);
    }

    fn visit_enum_decl(&mut self, name, variants, line, col) {
        // add the enum name itself
        self.add(NameBinding { name: name.to_string(), kind: NameKind::Const, line, column: col, params: None });
        // variants are accessed as Name.Variant, not as bare names
        // so we don't add them to scope — the LSP handles them via
        // a separate "after-dot completions" path
    }
}
```

### How the LSP uses it

```
// in the LSP completion handler:
let program    = parser.parse_program();
let scope_tree = ScopeAnalyzer::analyze(&program);
let cursor     = (params.position.line + 1, params.position.character + 1);
let names      = scope_tree.names_at(cursor.0, cursor.1);

let items = names.iter().map(|b| CompletionItem {
    label:  b.name.clone(),
    kind:   match b.kind {
        NameKind::Fn | NameKind::Param => CompletionItemKind::FUNCTION,
        NameKind::Const                => CompletionItemKind::CONSTANT,
        NameKind::Import               => CompletionItemKind::MODULE,
        _                              => CompletionItemKind::VARIABLE,
    },
}).collect();
```

---

---

# 6 — Wire `fmt check`

## File: `crates/fmt/src/main.rs`

Depends on items **1** (error recovery) and **3** (ParseErrorKind).

The skeleton is already there. Replace the `eprintln!("not yet implemented")` body:

```
fn check_files(files: &[PathBuf]) -> anyhow::Result<()> {

    let mut total_errors = 0;

    for path in files {
        // read source
        let src = match std::fs::read_to_string(path) {
            Ok(s)  => s,
            Err(e) => {
                eprintln!("{}: cannot read file: {}", path.display(), e);
                total_errors += 1;
                continue;
            }
        };

        // parse
        let lexer  = code_lang::lexer::lexer::Lexer::new(src);
        let mut parser = code_lang::parser::parser::Parser::new(lexer);
        parser.parse_program();

        if parser.errors.is_empty() {
            // print nothing on success — just exit 0
            // (mirroring rustfmt --check behavior)
        } else {
            for err in &parser.errors {
                // format: path:line:col: error: message
                eprintln!(
                    "{}:{}:{}: error: {}",
                    path.display(), err.line, err.column, err.message
                );
            }
            total_errors += parser.errors.len();
        }
    }

    if total_errors > 0 {
        eprintln!("\n{} error(s) found", total_errors);
        std::process::exit(1);
    }

    Ok(())
}
```

**What the output looks like:**

```
$ code-lang-fmt check src/utils.cl src/main.cl
src/utils.cl:4:12: error: expected ASSIGN, got PLUS
src/utils.cl:9:1: error: unclosed '('
src/main.cl:22:5: error: unexpected end of file

3 error(s) found
```

Exit code 0 if all files are clean, exit code 1 if any errors — this
makes it usable in CI and git hooks.

---

---

# 7 — Wire `fmt lint`

## Files: `crates/fmt/src/main.rs`, `crates/fmt/src/lint.rs` (new)

Depends on items **3** (ParseErrorKind) and **4** (Visitor).

### New file: `crates/fmt/src/lint.rs`

Define the shared `LintDiagnostic` type and the `LintRule` trait:

```
pub enum LintSeverity { Error, Warning, Info }

pub struct LintDiagnostic {
    pub rule:     &'static str,
    pub message:  String,
    pub line:     usize,
    pub column:   usize,
    pub severity: LintSeverity,
}

pub trait LintRule {
    // called after walking the full program
    fn diagnostics(&self) -> &[LintDiagnostic];
}
```

### Rule 1: `UnusedImport`

```
pub struct UnusedImport {
    imported:     Vec<(String, usize, usize)>,   // (module_name, line, col)
    referenced:   HashSet<String>,
    diags:        Vec<LintDiagnostic>,
}

impl Visitor for UnusedImport {

    fn visit_import(&mut self, path, line, col) {
        let name = path.split('/').last().unwrap_or(path);
        self.imported.push((name.to_string(), line, col));
    }

    fn visit_member(&mut self, object, _property, _line, _col) {
        // math.sqrt(...) — 'math' is the object
        if let Expression::Ident { value, .. } = object {
            self.referenced.insert(value.clone());
        }
    }
}

impl LintRule for UnusedImport {
    fn diagnostics(&self) -> &[LintDiagnostic] {
        // build diags lazily (or during a finish() call)
        for (name, line, col) in &self.imported {
            if !self.referenced.contains(name) {
                self.diags.push(LintDiagnostic {
                    rule:     "unused-import",
                    message:  format!("'{}' is imported but never used", name),
                    line:     *line,
                    column:   *col,
                    severity: LintSeverity::Warning,
                });
            }
        }
        &self.diags
    }
}
```

### Rule 2: `ShadowedBinding`

```
pub struct ShadowedBinding {
    // stack of scopes; each scope maps name -> (line, col) of first binding
    scope_stack: Vec<HashMap<String, (usize, usize)>>,
    diags:       Vec<LintDiagnostic>,
}

impl Visitor for ShadowedBinding {

    fn visit_block(&mut self, stmts, _line, _col) {
        self.scope_stack.push(HashMap::new());  // enter scope
        for stmt in stmts { self.visit_statement(stmt); }
        self.scope_stack.pop();                  // leave scope
    }

    fn visit_let(&mut self, pattern, value, line, col) {
        walk_expression(self, value);
        if let LetPattern::Ident(name) = pattern {
            let current_scope = self.scope_stack.last_mut().unwrap();
            if let Some((prev_line, prev_col)) = current_scope.get(name) {
                self.diags.push(LintDiagnostic {
                    rule:    "shadowed-binding",
                    message: format!(
                        "'{}' shadows an earlier binding at {}:{}",
                        name, prev_line, prev_col
                    ),
                    line, column: col,
                    severity: LintSeverity::Warning,
                });
            } else {
                current_scope.insert(name.clone(), (line, col));
            }
        }
    }
}
```

### `lint_files` in `main.rs`

```
fn lint_files(files: &[PathBuf]) -> anyhow::Result<()> {

    let mut total = 0;

    for path in files {
        let src    = std::fs::read_to_string(path)?;
        let lexer  = Lexer::new(src);
        let parser = Parser::new(lexer);
        let program = parser.parse_program();

        // run all rules
        let mut unused_import = UnusedImport::new();
        let mut shadowed      = ShadowedBinding::new();

        walk_program(&mut unused_import, &program);
        walk_program(&mut shadowed,      &program);

        let all_diags: Vec<_> = unused_import.diagnostics()
            .iter()
            .chain(shadowed.diagnostics().iter())
            .collect();

        for diag in &all_diags {
            let level = match diag.severity {
                LintSeverity::Error   => "error",
                LintSeverity::Warning => "warning",
                LintSeverity::Info    => "info",
            };
            eprintln!(
                "{}:{}:{}: [{}] [{}] {}",
                path.display(), diag.line, diag.column,
                level, diag.rule, diag.message
            );
        }

        total += all_diags.len();
    }

    if total > 0 { std::process::exit(1); }
    Ok(())
}
```

---

---

# 8 — Wire LSP diagnostics

## Files: `crates/lsp/Cargo.toml`, `crates/lsp/src/main.rs`

Depends on items **1** (error recovery), **2** (end positions), **3** (ParseErrorKind).

### `crates/lsp/Cargo.toml` — add deps

```toml
[dependencies]
code-lang   = { path = "../..", version = "0.2.2" }
tower-lsp   = "0.20"
tokio       = { version = "1", features = ["full"] }
dashmap     = "5"     # thread-safe hashmap for document cache
```

### `crates/lsp/src/main.rs` — full skeleton

```
use std::sync::Arc;
use dashmap::DashMap;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer, LspService, Server};
use code_lang::lexer::lexer::Lexer;
use code_lang::parser::parser::Parser;

struct Backend {
    client:    Client,
    documents: Arc<DashMap<String, String>>,  // uri -> source text
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {

    async fn initialize(&self, _params: InitializeParams) -> Result<InitializeResult> {
        Ok(InitializeResult {
            capabilities: ServerCapabilities {

                // we want the full document text on every change
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::FULL,
                )),

                // we will provide completions (item 9)
                completion_provider: Some(CompletionOptions {
                    trigger_characters: Some(vec![".".to_string()]),
                    ..Default::default()
                }),

                ..Default::default()
            },
            server_info: Some(ServerInfo {
                name:    "code-lang-lsp".to_string(),
                version: Some(env!("CARGO_PKG_VERSION").to_string()),
            }),
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        self.client
            .log_message(MessageType::INFO, "code-lang-lsp ready")
            .await;
    }

    async fn shutdown(&self) -> Result<()> { Ok(()) }

    // called when the editor opens a file
    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        let uri  = params.text_document.uri.to_string();
        let text = params.text_document.text;
        self.documents.insert(uri.clone(), text.clone());
        self.publish_diagnostics(params.text_document.uri, &text).await;
    }

    // called on every keystroke (FULL sync = full document each time)
    async fn did_change(&self, mut params: DidChangeTextDocumentParams) {
        if let Some(change) = params.content_changes.pop() {
            let uri  = params.text_document.uri.to_string();
            self.documents.insert(uri, change.text.clone());
            self.publish_diagnostics(params.text_document.uri, &change.text).await;
        }
    }

    // called when the editor closes a file — clear diagnostics
    async fn did_close(&self, params: DidCloseTextDocumentParams) {
        let uri = params.text_document.uri.to_string();
        self.documents.remove(&uri);
        self.client
            .publish_diagnostics(params.text_document.uri, vec![], None)
            .await;
    }
}

impl Backend {

    async fn publish_diagnostics(&self, uri: Url, source: &str) {

        let lexer   = Lexer::new(source.to_string());
        let mut parser = Parser::new(lexer);
        parser.parse_program();

        let diagnostics: Vec<Diagnostic> = parser.errors.iter().map(|e| {

            // line/col in LSP are 0-indexed; ours are 1-indexed
            let start = Position {
                line:      (e.line.saturating_sub(1)) as u32,
                character: (e.column.saturating_sub(1)) as u32,
            };

            // end position uses item 2 (end_line, end_col on ParseError)
            // if not yet implemented, use start + 1 as fallback
            let end = Position {
                line:      (e.end_line.saturating_sub(1)) as u32,
                character: e.end_col as u32,
            };

            Diagnostic {
                range:    Range { start, end },
                severity: Some(DiagnosticSeverity::ERROR),
                code:     None,
                message:  e.message.clone(),
                source:   Some("code-lang".to_string()),
                ..Default::default()
            }

        }).collect();

        self.client.publish_diagnostics(uri, diagnostics, None).await;
    }
}

#[tokio::main]
async fn main() {
    let stdin  = tokio::io::stdin();
    let stdout = tokio::io::stdout();
    let (service, socket) = LspService::new(|client| Backend {
        client,
        documents: Arc::new(DashMap::new()),
    });
    Server::new(stdin, stdout, socket).serve(service).await;
}
```

**What happens in the editor:**

User types `let x = (` without closing the paren. On the next
keystroke the LSP re-parses, finds `UnclosedDelimiter`, and sends a
diagnostic. The editor draws a red squiggle under the `(`.

---

---

# 9 — Wire LSP completions

## File: `crates/lsp/src/main.rs`

Depends on items **2** (end positions) and **5** (scope analysis).

### Add to the `LanguageServer` impl

```
async fn completion(
    &self,
    params: CompletionParams,
) -> Result<Option<CompletionResponse>> {

    let uri    = params.text_document_position.text_document.uri.to_string();
    let cursor = params.text_document_position.position;

    // get the cached source text
    let source = match self.documents.get(&uri) {
        Some(s) => s.clone(),
        None    => return Ok(None),
    };

    // parse (errors are fine — we have error recovery from item 1)
    let lexer   = Lexer::new(source);
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();

    // build scope tree (item 5)
    let scope_tree = ScopeAnalyzer::analyze(&program);

    // cursor is 0-indexed in LSP; our positions are 1-indexed
    let line = (cursor.line + 1) as usize;
    let col  = (cursor.character + 1) as usize;

    let bindings = scope_tree.names_at(line, col);

    // local scope completions
    let mut items: Vec<CompletionItem> = bindings.iter().map(|b| {
        CompletionItem {
            label: b.name.clone(),
            kind:  Some(match b.kind {
                NameKind::Param  | NameKind::Let => CompletionItemKind::VARIABLE,
                NameKind::Const                  => CompletionItemKind::CONSTANT,
                NameKind::Import                 => CompletionItemKind::MODULE,
                NameKind::EnumVariant            => CompletionItemKind::ENUM_MEMBER,
                _                               => CompletionItemKind::TEXT,
            }),
            ..Default::default()
        }
    }).collect();

    // if the cursor is after a '.', add stdlib member completions
    // e.g. "math." -> [sqrt, abs, pow, floor, ceil, ...]
    if let Some(module_name) = detect_module_prefix(&source, line, col) {
        if let Some(members) = STDLIB_MEMBERS.get(module_name) {
            for member in *members {
                items.push(CompletionItem {
                    label: member.to_string(),
                    kind:  Some(CompletionItemKind::FUNCTION),
                    ..Default::default()
                });
            }
        }
    }

    Ok(Some(CompletionResponse::Array(items)))
}
```

### The stdlib member table

A static lookup of module name → list of member names.
This is hardcoded — it never changes unless you add stdlib functions.

```
static STDLIB_MEMBERS: &[(&str, &[&str])] = &[
    ("fmt",     &["print", "eprint", "input", "typeof", "to_int", "to_float", "to_str", "clear", "format"]),
    ("math",    &["sqrt", "abs", "pow", "floor", "ceil", "round", "trunc", "log", "log10", "log2",
                  "exp", "sin", "cos", "tan", "min", "max", "clamp", "sign", "gcd", "lcm", "PI", "E"]),
    ("strings", &["to_upper", "to_lower", "trim", "split", "join", "contains",
                  "starts_with", "ends_with", "replace", "index", "count", "repeat",
                  "reverse", "to_chars", "from_chars", "parse_int", "parse_float",
                  "lines", "is_empty", "pad_left", "pad_right"]),
    ("arrays",  &["len", "first", "last", "rest", "pop", "push", "prepend", "concat",
                  "reverse", "slice", "contains", "index_of", "join", "sum", "min", "max",
                  "sort", "unique", "flatten", "zip", "map", "filter", "reduce", "find", "any", "all"]),
    ("hash",    &["keys", "values", "entries", "has_key", "get", "len", "merge", "delete"]),
    ("fs",      &["read_file", "write_file", "append_file", "read_lines", "exists",
                  "is_file", "is_dir", "list_dir", "mkdir", "mkdir_all", "remove", "remove_dir", "copy", "rename"]),
    ("path",    &["join", "basename", "dirname", "stem", "extension", "absolute", "is_absolute"]),
    ("os",      &["args", "platform", "arch", "get_env", "set_env", "get_wd", "hostname", "exit"]),
    ("time",    &["now", "unix", "sleep", "since", "format", "year", "month", "day",
                  "hour", "minute", "second", "RFC3339", "Kitchen"]),
    ("json",    &["parse", "stringify"]),
    ("rand",    &["int", "float", "choice", "shuffle"]),
    ("http",    &["get", "post", "post_json"]),
];
```

### `detect_module_prefix` helper

```
// look backwards from the cursor in the source text
// if the character immediately before the cursor is '.',
// and before that is an identifier, return that identifier
fn detect_module_prefix(source: &str, line: usize, col: usize) -> Option<&str> {
    let lines: Vec<&str> = source.lines().collect();
    let line_text = lines.get(line.saturating_sub(1))?;
    let up_to_cursor = &line_text[..col.min(line_text.len()).saturating_sub(1)];

    if !up_to_cursor.ends_with('.') { return None; }

    let before_dot = &up_to_cursor[..up_to_cursor.len() - 1];
    let ident_start = before_dot.rfind(|c: char| !c.is_alphanumeric() && c != '_')
        .map(|i| i + 1)
        .unwrap_or(0);

    Some(&before_dot[ident_start..])
}
```

---

---

# 10 — Install script

## New file: `install.sh` at repo root

No interpreter changes needed. The script builds the workspace
binaries and places them in `~/.code-lang/bin/` so the VS Code
extension can find them at a fixed known path.

```bash
#!/usr/bin/env sh
set -e

INSTALL_DIR="$HOME/.code-lang/bin"
REPO_URL="https://github.com/Walon-Foundation/code-lang"

# ── helpers ──────────────────────────────────────────────────────────────

info()  { printf '\033[0;36m  info\033[0m  %s\n' "$*"; }
ok()    { printf '\033[0;32m    ok\033[0m  %s\n' "$*"; }
die()   { printf '\033[0;31m error\033[0m  %s\n' "$*" >&2; exit 1; }

# ── check dependencies ───────────────────────────────────────────────────

if ! command -v cargo >/dev/null 2>&1; then
    die "cargo not found — install Rust from https://rustup.rs and try again"
fi

if ! command -v git >/dev/null 2>&1; then
    die "git not found"
fi

RUST_VERSION=$(rustc --version | cut -d' ' -f2)
info "using Rust $RUST_VERSION"

# ── clone and build ──────────────────────────────────────────────────────

TMPDIR=$(mktemp -d)
trap 'rm -rf "$TMPDIR"' EXIT

info "cloning $REPO_URL ..."
git clone --depth 1 "$REPO_URL" "$TMPDIR/code-lang"

info "building workspace (this takes ~30 seconds) ..."
cargo build --release --workspace --manifest-path "$TMPDIR/code-lang/Cargo.toml"

# ── install ──────────────────────────────────────────────────────────────

mkdir -p "$INSTALL_DIR"

for BIN in code-lang code-lang-lsp code-lang-fmt; do
    SRC="$TMPDIR/code-lang/target/release/$BIN"
    if [ -f "$SRC" ]; then
        cp "$SRC" "$INSTALL_DIR/$BIN"
        ok "installed $BIN"
    else
        info "skipping $BIN (not built)"
    fi
done

# ── PATH hint ────────────────────────────────────────────────────────────

case ":$PATH:" in
    *":$INSTALL_DIR:"*)
        ok "$INSTALL_DIR is already in PATH"
        ;;
    *)
        printf '\n'
        info "add this to your shell profile (~/.bashrc, ~/.zshrc, etc.):"
        printf '    export PATH="%s:$PATH"\n\n' "$INSTALL_DIR"
        ;;
esac

printf '\033[0;32m\nInstallation complete.\033[0m\n'
printf '  code-lang --version\n\n'
```

**Usage:**

```
curl -sSf https://raw.githubusercontent.com/Walon-Foundation/code-lang/main/install.sh | sh
```

**Why `~/.code-lang/bin/`**

The VS Code extension will look for `code-lang-lsp` at this path by
default. If it finds it, LSP is enabled automatically — the user does
not need to configure any paths. If it does not find it, the extension
falls back to syntax-highlighting-only mode. This is the same
convention rustup uses (`~/.cargo/bin/`).

---

---

# Dependency map (visual)

```
1 (error recovery) ──────────────────────────┐
2 (end positions)  ──────────┐               │
3 (ParseErrorKind) ──┐       │               │
4 (visitor)  ────────┼──┐    │               │
                     │  │    │               │
                     5  │    │               │
                     │  │    │               │
                     │  │    │               │
              ┌──────┘  │    │               │
              │          │    │               │
              9           7   8               6
         (completions) (lint) (diagnostics) (check)
```

Start with **1, 2, 3, 4** in parallel. They are all changes to
existing files and are completely independent of each other. **5**
can start as soon as **4** is done. **6, 7, 8** can start as soon as
their dependencies are done. **9** is last because it needs both
**2** and **5**. **10** (install script) can be written any time.

---

# Files touched, complete list

| File | Items |
|---|---|
| `src/parser/parser.rs` | 1, 2, 3 |
| `src/ast/ast.rs` | 2 |
| `src/ast/walk.rs` *(new)* | 4 |
| `src/lib.rs` | expose `analysis` module |
| `src/analysis/mod.rs` *(new)* | 5 |
| `src/analysis/scope.rs` *(new)* | 5 |
| `crates/fmt/src/main.rs` | 6, 7 |
| `crates/fmt/src/lint.rs` *(new)* | 7 |
| `crates/lsp/Cargo.toml` | 8, 9 |
| `crates/lsp/src/main.rs` | 8, 9 |
| `install.sh` *(new)* | 10 |
