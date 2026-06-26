# code-lang Internals — How the Language Works

> This document explains the full execution pipeline from source text to a result.
> It is written for someone reading or modifying the interpreter source, not for
> end users of the language.

---

## Overview

code-lang is a **tree-walking interpreter** written in Rust. There is no bytecode compiler and no virtual machine — the AST (Abstract Syntax Tree) is evaluated directly. The pipeline is linear:

```
Source text (.cl file or REPL input)
       │
       ▼
   [ Lexer ]          src/lexer/lexer.rs
       │  produces Token stream
       ▼
   [ Parser ]         src/parser/parser.rs
       │  produces Program (AST)
       ▼
   [ Evaluator ]      src/evaluator/evaluator.rs
       │  walks AST, returns Object
       ▼
   [ Result ]         printed to stdout or returned to REPL
```

Each stage is independent — the lexer knows nothing about the parser, the parser knows nothing about the evaluator. The stages communicate through two data structures: `Token` (lexer → parser) and `Program` / `Expression` / `Statement` (parser → evaluator).

---

## Entry Points

**File:** `src/main.rs`

The binary has two modes, selected by whether a file argument is provided:

```
code-lang             → run_repl()
code-lang script.cl   → execute(file_contents)
```

Both modes live in `src/repl/repl.rs`.

`execute()` is a single-shot pipeline: lex → parse → eval → print result → exit.  
`run_repl()` runs the same pipeline in a loop, reusing the same `Evaluator` and `Environment` across inputs so variables declared in one line are visible in the next.

The only file extension accepted in script mode is `.cl` — any other extension is rejected before reading the file.

---

## Stage 1 — Lexer

**File:** `src/lexer/lexer.rs`  
**Key type:** `Lexer`  
**Output:** a stream of `Token` values, one per call to `next_token()`

### What it does

The lexer converts raw source text into a flat sequence of tokens. It works character by character, maintaining:

- `position` — index of the character currently being examined
- `read_position` — index of the next character (one ahead)
- `ch` — the current character
- `line` / `column` — current source position (updated on every `read_char()`)

Every token carries its `line` and `column` so error messages can point to the right place.

### How `next_token()` works

1. Skip whitespace (spaces, tabs, newlines, carriage returns)
2. Look at `self.ch` and match it:
   - Single-character tokens: `(`, `)`, `{`, `}`, `[`, `]`, `;`, `,`, `:`
   - Two-character tokens: peek at `self.peak_char()` to decide — `==`, `!=`, `=>`, `++`, `--`, `+=`, `-=`, `*=`, `/=`, `%=`, `<=`, `>=`, `//`, `&&`, `||`, `??`
   - Comments: `#` skips to end of line; `/*` skips to `*/` — neither produces a token, `next_token()` calls itself recursively
   - String literal `"..."` — calls `read_string()` which handles interpolation
   - Char literal `'x'` — calls `read_char_type()`
   - Identifier or keyword — calls `read_identifier()` then `lookup_ident()` to check if it's a keyword
   - Number — calls `read_number()` which handles both integers and floats (a `.` mid-number switches to float mode)
3. Advance `position` and return the token

### Keyword map

`lookup_ident()` in `src/token/token.rs` maps identifier strings to keyword token types:

```
"fn"       → Function       "let"      → Let
"const"    → Const          "if"       → If
"else"     → Else           "elseif"   → ElseIf
"while"    → While          "for"      → For
"return"   → Return         "break"    → Break
"continue" → Continue       "in"       → In
"import"   → Import         "struct"   → Struct
"switch"   → Switch         "enum"     → Enum
"pub"      → Pub            "typeof"   → Typeof
"null"     → Null           "true"     → True
"false"    → False          "default"  → Default
anything else → Ident(string)
```

### String interpolation

`read_string()` handles `"..."` literals with `${}` embedded expressions. It scans character by character building a `Vec<StringPart>`:

- Ordinary characters accumulate into `StringPart::Literal(string)`
- `${` triggers a nested scan: reads until the matching `}` (tracking brace depth), storing the raw source text of the expression as `StringPart::Expr(source)`

The parser later re-lexes and re-parses each `StringPart::Expr` source string to produce the final `Expression::InterpolatedString`.

### Token types

All token types are in `src/token/token.rs` as the `TokenType` enum. Notable ones:

| Token | Represents |
|---|---|
| `Int(isize)` | Integer literal, value embedded in token |
| `Float(f64)` | Float literal |
| `Char(char)` | Character literal `'x'` |
| `InterpolatedString(Vec<StringPart>)` | String with optional `${}` segments |
| `Ident(String)` | Any name that is not a keyword |
| `FatArrow` | `=>` used in switch arms |
| `Floor` | `//` integer division |
| `Square` | `**` power |
| `NullCoalesce` | `??` |
| `Inc` / `Dec` | `++` / `--` |
| `EOF` | End of input |
| `ILLEGAL` | Character the lexer cannot classify |

---

## Stage 2 — Parser

**File:** `src/parser/parser.rs`  
**Key type:** `Parser`  
**Output:** `Program` — a list of `Statement` nodes

### How the parser works

code-lang uses a **Pratt parser** (top-down operator precedence parsing). This is the same technique used by V8's JavaScript parser and `rustc`'s expression parser.

The parser holds:
- `cur_token` — the token currently being examined
- `peek_token` — the next token (one ahead)
- `errors: Vec<ParseError>` — collected errors (parser never panics)

`next_token()` advances both: `cur_token ← peek_token`, `peek_token ← lexer.next_token()`.

### Top level: `parse_program()`

Calls `parse_statement()` in a loop until `EOF`. Each statement is appended to `Program::statements`.

### Statement parsing

`parse_statement()` dispatches on `cur_token.token_type`:

| Token | Calls |
|---|---|
| `Let` | `parse_let_statement()` |
| `Const` | `parse_const_statement()` |
| `Return` | `parse_return_statement()` |
| `Import` | `parse_import_statement()` |
| `Break` | `parse_break_statement()` |
| `Continue` | `parse_continue_statement()` |
| `Struct` | `parse_struct_statement()` |
| `Enum` | `parse_enum_statement()` |
| `Pub` | `parse_pub_statement()` |
| anything else | `parse_expression_statement()` |

### Expression parsing: Pratt (TDOP)

`parse_expression(precedence)` is the core. It:

1. Calls the **prefix function** for `cur_token` to get a left-hand expression
2. Loops: while `peek_token` has higher precedence than `precedence`, calls the **infix function** for `peek_token`, passing in the current left-hand expression

Prefix functions handle things that start an expression:
- Literal tokens (`Int`, `Float`, `Char`, `Bool`, `Null`, `InterpolatedString`) → literal `Expression` nodes
- `Ident` → `Expression::Ident`
- `Bang` / `Minus` → `Expression::Prefix`
- `LParan` → grouped expression (recurse, expect `)`)
- `LBracket` → array literal
- `LBrace` → hash literal
- `Function` → `parse_function_literal()`
- `If` → `parse_if_expression()`
- `While` → `parse_while_expression()`
- `For` → `parse_for_expression()` (detects `for-in` if `In` token follows the variable)
- `Switch` → `parse_switch_expression()`
| `Typeof` → `parse_typeof_expression()`

Infix functions handle binary operators — `+`, `-`, `*`, `/`, `==`, `!=`, `<`, `>`, `<=`, `>=`, `&&`, `||`, `??`, `**`, `//`, `%`:
- All emit `Expression::Infix`
- `LParan` as infix → `Expression::Call` (function call)
- `LBracket` as infix → `Expression::Index` (array/hash index)
- `Dot` as infix → `Expression::Member` (property access)
- `Inc` / `Dec` as infix → `Expression::Update` (postfix)

### Precedence levels

```
Lowest      = 0    (default)
Assign      = 1    = += -= *= /= %=
NullCoal    = 2    ??
Or          = 3    ||
And         = 4    &&
Equals      = 5    == !=
LessGreater = 6    < > <= >=
Sum         = 7    + -
Product     = 8    * / % // **
Prefix      = 9    -x !x (not an infix level)
Postfix     = 10   x++ x-- x() x[] x.y
```

### Error handling

The parser never panics. When an unexpected token is encountered, it pushes a `ParseError { message, line, column }` and tries to continue (usually returning `None` from the current parse function, which causes the caller to also return `None`). All errors are collected in `parser.errors` and checked after `parse_program()` returns. If any errors exist, evaluation is skipped.

### Switch arm parsing

`parse_switch_expression()` loops until `}`. At the top of each iteration, after `next_token()`, it checks if `cur_token` is `Default`. If so:
- Expect `=>`
- Parse the body (block or expression)
- Consume optional comma
- Store body in `default: Option<Box<Statement>>`
- `continue` to next iteration

Normal arms: parse the pattern expression, expect `=>`, parse the body, consume optional comma, push `SwitchArm { pattern, body }`.

---

## Stage 3 — AST

**File:** `src/ast/ast.rs`

The AST is composed of three main types:

### `Program`

The root. Contains `statements: Vec<Statement>`.

### `Statement`

An enum covering all statement forms:

| Variant | Represents |
|---|---|
| `Let { pattern, value, line, column, .. }` | `let x = expr` |
| `Const { pattern, value, .. }` | `const x = expr` |
| `Return { value, .. }` | `return expr` |
| `Expression { expr, .. }` | A statement that is just an expression |
| `Block { statements, .. }` | `{ stmt; stmt; }` |
| `Import { path, .. }` | `import "module"` |
| `Break` / `Continue` | Loop control |
| `Struct { name, field, .. }` | Struct definition |
| `Enum { name, variant, .. }` | Enum definition |
| `Pub { statement, .. }` | Wraps a `Let` or `Const` to mark it exported |

`LetPattern` covers the left-hand side of `let`/`const`:
- `Ident(String)` — `let x = ...`
- `Array(Vec<String>)` — `let [a, b] = ...`
- `Hash(Vec<(String, String)>)` — `let { x, y: alias } = ...`

### `Expression`

An enum covering all expression forms. Every variant carries `line` and `column` for error reporting.

| Variant | Represents |
|---|---|
| `Ident` | A variable name |
| `Int(isize)` | Integer literal |
| `Float(f64)` | Float literal |
| `Char(char)` | Character literal |
| `Boolean(bool)` | `true` / `false` |
| `Null` | `null` |
| `InterpolatedString { parts }` | `"hello ${name}"` |
| `Prefix { op, right }` | `-x`, `!flag` |
| `Infix { left, op, right }` | `a + b`, `x == y` |
| `NullCoalesce { left, right }` | `a ?? b` |
| `Update { operator, target, prefix }` | `x++`, `--y` |
| `Typeof { value }` | `typeof expr` |
| `Call { function, argument }` | `fn(args)` |
| `Index { left, index }` | `arr[i]`, `hash[key]` |
| `Member { object, property }` | `obj.field` |
| `Array { element }` | `[1, 2, 3]` |
| `HashLiteral { pair }` | `{ "a": 1 }` |
| `Function { parameter, body }` | `fn(x, y) { ... }` |
| `If { condition, consequence, alternative, if_else }` | Full if/elseif/else |
| `While { condition, body }` | `while (cond) { }` |
| `For { init, condition, post, body }` | C-style for loop |
| `ForIn { key, value, iterable, body }` | `for (k, v in hash)` |
| `Switch { subject, arms, default }` | `switch (x) { ... }` |
| `StructLiteral { name, fields }` | `Point { x: 1, y: 2 }` |

### Supporting types

- `SwitchArm { pattern: Expression, body: Box<Statement> }` — one arm of a switch
- `Param { name: String, default: Option<Box<Expression>> }` — one function parameter
- `ElseIF { condition: Expression, consequences: Statement }` — one elseif branch
- `StringSegment` — either `Literal(String)` or `Expr(Box<Expression>)`, used inside `InterpolatedString`

---

## Stage 4 — Evaluator

**File:** `src/evaluator/evaluator.rs`  
**Key type:** `Evaluator`

### Evaluator struct

```rust
pub struct Evaluator {
    pub loop_depth: usize,         // how many loops are currently active
    pub call_depth: usize,         // how deep in function calls we are
    pub module_cache: HashMap<String, Object>,  // stdlib modules, loaded once
}
```

`module_cache` is populated in `preload_stdlib()` called from `new()`. All 12 stdlib modules are loaded into the cache at startup, not on first import.

`register_globals()` injects global builtins into the root environment. Currently only `is_error` lives here.

### Constant: `MAX_CALL_DEPTH = 500`

Function calls beyond this depth return an error instead of causing a Rust stack overflow.

### `eval()` — top level

```
eval(program, env):
  for each statement in program.statements:
    result = eval_statement(statement, env)
    if result is Return(v) → return v          (unwrap return value)
    if result is Error     → return immediately (propagate errors)
  return last result
```

### `eval_statement()` — statement dispatch

Matches on the `Statement` variant and calls the appropriate handler:

| Statement | Handler |
|---|---|
| `Block` | Creates an enclosed environment; evaluates each statement; stops on Return/Error/Break/Continue |
| `Let` | Evaluates value, binds to name in env via `env.set()` |
| `Const` | Same as Let but via `env.set_const()` — blocks future reassignment |
| `Return` | Evaluates value, wraps in `Object::Return` |
| `Break` | Returns `Object::Break` |
| `Continue` | Returns `Object::Continue` |
| `Import` | Checks module cache, reads file from disk, lexes/parses/evals it |
| `Struct` | Evaluates default field expressions, stores as `Object::StructType` |
| `Enum` | Stores as `Object::EnumType` |
| `Pub` | Evals inner statement, then calls `env.mark_pub(name)` |
| `Expression` | Calls `eval_expression()`, returns result directly |

### `eval_expression()` — expression dispatch

The large match at the heart of the evaluator. Key arms:

**Literals** — `Int`, `Float`, `Char`, `Boolean`, `Null` wrap directly into the corresponding `Object` variant.

**Ident** — calls `env.get(name)`. Returns `Object::Error` if not found ("identifier not found").

**InterpolatedString** — evaluates each `StringSegment::Expr` part and formats it to a string, concatenates with `StringSegment::Literal` parts.

**Prefix** — evaluates right operand, calls `eval_prefix()`:
- `!` → negates booleans, converts Null to true, everything else to false
- `-` → negates integers and floats, errors on other types

**Infix** — evaluates both operands, dispatches on type pair:
- Integer + Integer → `eval_integer_infix()` with checked arithmetic
- Float + Float → `eval_float_infix()` with NaN/Infinity guard
- Integer + Float (or vice versa) → both converted to float, then float infix
- String + String → concatenation (for `+` and `+=`)

**If** — evaluates condition, checks truthiness (`is_truthy()`), evaluates the matching branch. Truthiness: everything is truthy except `null` and `false`.

**While** — loop with `loop_depth` tracking. `Object::Break` unwinds and returns Null. `Object::Continue` skips to next iteration. Errors propagate out.

**For** — evaluates init statement, then loops: check condition, eval body, eval post statement.

**ForIn** — iterates `Object::Array` (single variable = element, two variables = index + element) or `Object::Hash` (key + value pairs).

**Function** — captures the current environment by reference (`Rc<RefCell<Environment>>`), stores `parameters` and `body`, returns `Object::Function`.

**Call** — evaluates function expression and all arguments, calls `apply_function()`.

**Index** — evaluates left and index expressions:
- Array: bounds-checks (including negative index guard), returns element or error
- Hash: searches `Vec<(Object, Object)>` linearly for a matching key

**Member** — evaluates object, then:
- `Object::Module` → looks up member in module's HashMap, checks `pub_gated`
- `Object::StructInstance` → looks up field
- `Object::EnumType` → looks up variant by name, returns `Object::EnumVariant`
- Other types → error

**Switch** — evaluates subject, iterates arms calling `objects_equal()` for each pattern. If a match is found, evaluates that arm's body. If no arm matches and `default` is `Some`, evaluates the default body. Otherwise returns Null.

**Assignment (`eval_assignment`)** — called when an `Infix` with `=`/`+=`/etc. is found. Handles three target forms:
- `Ident` → calls `env.update(name, val)`. Errors if name not found (no silent creation).
- `Index` → evaluates the container, updates in place (arrays: bounds check; hashes: find-and-replace or insert)
- `Member` → updates a struct instance field

### `apply_function()` — function calls

Called for both user functions and builtin functions:

**User function (`Object::Function`):**
1. Check `call_depth >= MAX_CALL_DEPTH` → error
2. Check arg count vs required params (accounting for `self` in methods and default params)
3. Create new `Environment::new_enclosed(func_env)` — uses the closure's captured env as outer, not the call site's env
4. Bind each param to its arg (or default value if arg omitted)
5. `call_depth += 1` → eval body → `call_depth -= 1`
6. Unwrap `Object::Return` from body result

**Builtin (`Object::Builtin`):**
- Calls the function pointer directly with args and `CallInfo { line, column }`

**BuiltinHigherOrder (`Object::BuiltinHigherOrder`):**
- Calls the function pointer with args, `CallInfo`, and `&mut self` (so the builtin can call back into the evaluator to invoke user functions)

### Integer arithmetic safety

`eval_integer_infix()` uses checked arithmetic for `+`, `-`, `*`, `**`:
```
l.checked_add(r).map(Object::Integer).unwrap_or_else(|| Object::Error { "integer overflow" })
```

`/` and `%` check for zero before dividing.

`**` converts to f64, checks if result fits in `isize`, then converts back.

`//` (floor division) also checks for zero.

### Float safety

`eval_float_infix()` uses a `float_guard()` helper that checks for `NaN` or `±Infinity` after every float operation and returns an error instead of propagating the IEEE 754 special value.

---

## Stage 5 — Object System

**File:** `src/object/object.rs`

Every value in code-lang is an `Object`. The enum has 19 variants:

| Variant | Rust type | Displayed as |
|---|---|---|
| `Integer(isize)` | platform-sized int | `42` |
| `Float(f64)` | 64-bit float | `3.14` |
| `StringType(String)` | heap string | `"hello"` |
| `Char(char)` | Unicode scalar | `'a'` |
| `Bool(bool)` | boolean | `true` / `false` |
| `Null` | unit | `null` |
| `Array(Vec<Object>)` | ordered list | `[1, 2, 3]` |
| `Hash(Vec<(Object, Object)>)` | key-value pairs | `{a: 1}` |
| `Function { parameters, body, env }` | closure | `fn(x)` |
| `Builtin(fn(...) -> Object)` | Rust function pointer | `[Builtin]` |
| `BuiltinHigherOrder(fn(..., &mut dyn Evaluable) -> Object)` | Rust fn with evaluator access | `[Builtin]` |
| `StructType { name, default }` | struct definition | `struct Point` |
| `StructInstance { type_name, fields }` | struct value | `Point { x: 1, y: 2 }` |
| `EnumType { name, variants }` | enum definition | `Direction(North \| South)` |
| `EnumVariant { enum_name, variant }` | enum value | `Direction.North` |
| `Module { name, pub_gated, members }` | imported module | `[Module: fmt]` |
| `Return(Box<Object>)` | control flow signal | (internal) |
| `Break` | control flow signal | (internal) |
| `Continue` | control flow signal | (internal) |
| `Error { message, line, column }` | runtime error | `error: ...` |

`Return`, `Break`, `Continue`, and `Error` are not user-visible values — they are signals that propagate up through the evaluation stack and are consumed by the statement handlers that understand them (loops consume Break/Continue, `eval()` unwraps Return, error display consumes Error).

### `Hash` is a `Vec`, not a `HashMap`

`Object::Hash` stores pairs as `Vec<(Object, Object)>` rather than a `HashMap`. This is intentional — keys can be any `Object` (including non-hashable ones like `Array`), and the hash is small enough that linear scan is fine. Lookup is O(n) via `objects_equal()`.

### The `Evaluable` trait

```rust
pub trait Evaluable {
    fn call_function(&mut self, func: Object, args: Vec<Object>, info: CallInfo) -> Object;
}
```

`Evaluator` implements this trait. It is the interface through which `BuiltinHigherOrder` functions (like `arrays.map`) call back into the evaluator to invoke user-provided function values. Without this trait, stdlib functions would need a reference to `Evaluator`, which would create a circular dependency.

### `Environment`

Environments form a linked scope chain:

```
Root env (globals: is_error, imported modules)
  └─ Block env (for each { ... } block)
       └─ Function env (enclosed over the closure's defining scope)
```

Each `Environment` has:
- `store: HashMap<String, Object>` — bindings in this scope
- `consts: HashMap<String, bool>` — names that cannot be reassigned
- `pubs: HashSet<String>` — names visible to importers (used by `pub let`)
- `outer: Option<Rc<RefCell<Environment>>>` — parent scope

`get(name)` walks the chain upward until it finds the name or reaches the root.  
`update(name, val)` also walks upward, updating the first scope where the name exists. Returns `false` if not found (used by assignment to detect undeclared variables).  
`set(name, val)` always writes to the current (innermost) scope.

Environments are reference-counted (`Rc<RefCell<>>`). Functions capture a reference to their defining environment, enabling closures.

---

## Stage 6 — Standard Library

**Directory:** `src/std_lib/`

All 12 stdlib modules are written in Rust and registered in `Evaluator::preload_stdlib()`:

| Module | File | Contents |
|---|---|---|
| `arrays` | `array.rs` | push, pop, len, map, filter, reduce, find, any, all, sort, zip, flatten, unique, slice, contains, index_of, reverse, concat, chunk, dedupe, first, last |
| `strings` | `strings.rs` | to_upper, to_lower, split, join, contains, replace, trim, trim_left, trim_right, reverse, starts_with, ends_with, index_of, count, repeat, chars, from_chars, parse_int, parse_float, len, lines, is_empty, pad_left, pad_right |
| `math` | `math.rs` | PI, E, sqrt, abs, pow, floor, ceil, round, log, log2, sin, cos, tan, min, max, clamp, sign, gcd, lcm |
| `fmt` | `fmt.rs` | print, eprint, input, to_str, to_int, to_float, format, clear |
| `hash` | `hash.rs` | keys, values, entries, has_key, merge, delete, len, get |
| `fs` | `fs.rs` | read_file, write_file, append_file, read_lines, exists, list_dir, mkdir, copy, rename, remove |
| `path` | `path.rs` | join, basename, dirname, stem, extension, absolute, is_absolute |
| `os` | `os_mod.rs` | args, platform, arch, get_env, set_env, get_wd, hostname, exit |
| `time` | `time.rs` | now, unix, sleep, since, format, year, month, day, hour, minute, second |
| `json` | `json.rs` | parse, stringify |
| `rand` | `rand.rs` | int, float, choice, shuffle |
| `http` | `http.rs` | get, post, post_json |

Each module's `module()` function returns an `Object::Module` with a `HashMap<String, Object>` of its members. Functions are stored as `Object::Builtin` (or `Object::BuiltinHigherOrder` for map/filter/reduce/find/any/all).

### Import resolution

When `import "arrays"` is evaluated:

1. Check `module_cache` — if found, return it directly (no re-evaluation)
2. If not in cache (user-defined modules): read the `.cl` file from disk, lex/parse/eval it, store resulting environment as `Object::Module`
3. Bind the module to the import name in the current environment

Stdlib modules are always found in step 1 because `preload_stdlib()` populates the cache at startup.

---

## Error Display

**File:** `src/repl/repl.rs` — `show_error()`

When an `Object::Error` is returned (either from the evaluator or as a parse error):

```
error: cannot add INTEGER and STRING
  --> 3:14
   |
 3 | let x = 1 + "hello";
   |              ^
hint: use fmt.to_str() to convert a number to string
```

The format:
1. `error: <message>` on its own line
2. `  --> line:column` with a gutter showing the line number width
3. The source line
4. A caret `^` pointing to the error column
5. An optional `hint:` line from `get_hint()` — a lookup table matching common error message substrings to fix suggestions

In REPL mode: errors print to stderr, the REPL continues.  
In script mode: errors print to stderr, process exits with code 1.

---

## code-lang-fmt

**Directory:** `crates/fmt/`

A separate binary (`code-lang-fmt`) that shares the same lexer and parser via the `code-lang` library crate. It does not use the evaluator at all.

Three subcommands:

**`check`** — parse each file, report parse errors, exit 1 if any found. Does not evaluate.

**`lint`** — parse each file, run lint rules over the AST, report findings. With `--fix`, applies auto-fixable rules.

Lint rules are in `crates/fmt/src/lint_rules.rs`. Each rule implements the `Visitor` trait from `crates/fmt/src/visitor.rs`, which provides default `visit_*` methods that recurse into child nodes. Rules override only the methods they care about.

Current rules: `UnusedImport`, `ShadowedBinding`, `DeadCode`, `EmptyBlock`, `UnusedVariable`, `UndefinedVariable`, `ConstReassignment`.

**`format`** — not yet implemented (planned, see better-tools.md).

---

## Data Flow: A Single Expression

Tracing `let x = 1 + 2;` from text to result:

```
Source:  "let x = 1 + 2;"

Lexer produces:
  Token(Let, 1:1)
  Token(Ident("x"), 1:5)
  Token(Assign, 1:7)
  Token(Int(1), 1:9)
  Token(Plus, 1:11)
  Token(Int(2), 1:13)
  Token(Semicolon, 1:14)
  Token(EOF, 1:15)

Parser produces:
  Statement::Let {
    pattern: LetPattern::Ident("x"),
    value: Expression::Infix {
      left: Expression::Int { value: 1 },
      op: Token(Plus),
      right: Expression::Int { value: 2 },
    },
    line: 1, column: 1,
  }

Evaluator:
  eval_statement(Let { pattern: Ident("x"), value: Infix { ... } })
    eval_expression(Infix { left: Int(1), op: Plus, right: Int(2) })
      eval_expression(Int { value: 1 }) → Object::Integer(1)
      eval_expression(Int { value: 2 }) → Object::Integer(2)
      eval_integer_infix(Plus, 1, 2)
        1_isize.checked_add(2) → Some(3)
        → Object::Integer(3)
    env.set("x", Object::Integer(3))
    → Object::Null

REPL: Null is suppressed (not printed)
```

---

## Key Design Decisions

**Tree-walking, not bytecode.** Simpler to build and modify. The AST is evaluated directly, which is slower but means each AST node type has one clear evaluation rule. Good for a language at this stage.

**Errors as values, not exceptions.** `Object::Error` propagates up through the call stack rather than unwinding via panic or Rust's `?` operator. This means every function that can fail returns `Object` and every caller must check for `Object::Error { .. }`. The advantage is that errors can be caught with `is_error()` and stored in variables.

**`Rc<RefCell<Environment>>` for scopes.** Reference counting is used instead of arena allocation or a borrow-checker-friendly scope system because closures need to capture environments that outlive their defining scope. The `RefCell` allows mutation (variable assignment) through shared references.

**`Hash` as `Vec`, not `HashMap`.** Keys can be any `Object`. Since `Object` is not `Hash + Eq`, it cannot be used as a `HashMap` key without manual implementation. The linear scan cost is acceptable for the typical small hash sizes in a scripting context.

**Stdlib preloaded, not lazy.** All 12 modules are loaded at startup into `module_cache`. This means every program pays the initialization cost (~trivial in practice since stdlib functions are just Rust function pointers), but imports never do disk I/O or parsing.
