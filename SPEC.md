# code-lang Language Specification

**Version:** 0.2.x  
**Status:** Living document — reflects the current interpreter implementation.

---

## Table of Contents

1. [Introduction](#1-introduction)
2. [Lexical Structure](#2-lexical-structure)
3. [Types](#3-types)
4. [Expressions](#4-expressions)
5. [Statements](#5-statements)
6. [Scoping and Environments](#6-scoping-and-environments)
7. [Functions](#7-functions)
8. [Modules](#8-modules)
9. [Error Model](#9-error-model)
10. [Standard Library](#10-standard-library)
11. [Grammar](#11-grammar)

---

## 1. Introduction

code-lang is a dynamically typed, expression-oriented, interpreted language. Source files carry the `.cl` extension and are UTF-8 encoded. The runtime is a tree-walking interpreter implemented in Rust with the pipeline:

```
Source text → Lexer → Token stream → Parser → AST → Evaluator → Object
```

Every value is an `Object`. There is no compilation step; evaluation happens directly on the AST.

---

## 2. Lexical Structure

### 2.1 Source encoding

Source files are decoded as UTF-8. Identifiers and string contents may contain any valid Unicode, but only ASCII letters and `_` are recognized in identifier positions by the lexer.

### 2.2 Whitespace

The characters space (U+0020), horizontal tab (U+0009), newline (U+000A), and carriage return (U+000D) are whitespace and are skipped between tokens. Newlines do not terminate statements (there is no implicit semicolon insertion).

### 2.3 Comments

```
# single-line comment — extends to end of line

/* multi-line comment
   spans multiple lines */
```

Comments are stripped by the lexer and produce no tokens. Multi-line comments do not nest.

### 2.4 Keywords

The following identifiers are reserved and cannot be used as variable names:

```
fn      let     const   pub     return
if      else    elseif  while   for
in      break   continue switch  import
struct  enum    typeof  null    true    false
```

### 2.5 Identifiers

```ebnf
identifier = ( letter | "_" ) { letter | digit | "_" } ;
letter     = "a"..."z" | "A"..."Z" | "_" ;
digit      = "0"..."9" ;
```

Identifiers are case-sensitive. `foo`, `Foo`, and `FOO` are distinct names.

### 2.6 Integer literals

```ebnf
integer_literal = digit { digit } ;
```

Parsed as `isize` (platform-native signed integer, minimum 64 bits). An integer that overflows `isize` produces `ILLEGAL` at the lexer level and a parse error.

Examples: `0`, `42`, `1000000`

### 2.7 Float literals

```ebnf
float_literal = digit { digit } "." digit { digit }
              | "." digit { digit } ;
```

Parsed as IEEE 754 `f64`. A float that overflows `f64` produces `ILLEGAL`.

Examples: `3.14`, `0.5`, `.75`

### 2.8 String literals

All string literals are interpolated strings. They are delimited by double quotes `"..."`.

```ebnf
string_literal = '"' { string_char | interpolation } '"' ;
interpolation  = "$" "{" expression_source "}" ;
```

Inside `${...}`, the expression source is re-lexed and re-parsed independently. The expression may itself contain nested `{}` (the depth counter tracks braces). There is no escape sequence mechanism — to include a literal `"` inside a string, you must use string concatenation.

Examples:
```cl
"hello"
"hello ${name}"
"${a + b} is the result"
"nested: ${ fn(x) { x + 1; }(5) }"
```

### 2.9 Char literals

```ebnf
char_literal = "'" char_char "'" ;
```

Exactly one character between single quotes. An empty `''` or multi-character `'ab'` produces `ILLEGAL`.

Examples: `'a'`, `'Z'`, `'5'`

### 2.10 Boolean literals

`true` and `false` are keywords, not identifiers.

### 2.11 Null literal

`null` is a keyword that produces the Null value.

### 2.12 Operators and punctuation

| Token | Symbol |
|---|---|
| `=` | assignment |
| `==` | equality |
| `!=` | inequality |
| `<` `>` `<=` `>=` | comparison |
| `+` `-` `*` `/` `%` | arithmetic |
| `**` | exponentiation (power) |
| `//` | floor division |
| `++` `--` | increment / decrement |
| `+=` `-=` `*=` `/=` `%=` | compound assignment |
| `&&` `\|\|` `!` | logical and, or, not |
| `??` | null coalescing |
| `=>` | fat arrow (switch arms) |
| `.` | member access |
| `[` `]` | index / array literal |
| `{` `}` | block / hash literal / struct body |
| `(` `)` | grouping / call |
| `,` | separator |
| `;` | statement terminator |
| `:` | key-value separator |

Note: `&` alone and `|` alone are `ILLEGAL`. Only `&&` and `||` are valid.

---

## 3. Types

All values are one of the following types at runtime. Types are not declared — they are inferred from the value.

### 3.1 Integer

A 64-bit signed integer (`i64`). Range: −9,223,372,036,854,775,808 to 9,223,372,036,854,775,807.

- **Creation:** integer literal `42`, `typeof` result, arithmetic on integers
- **Display:** decimal, e.g. `42`
- **type_name:** `"INTEGER"` (or `"integer"` from `typeof` keyword — see §4.13)

Arithmetic on integers that would overflow produces an `Error` (not silent wrapping).

### 3.2 Float

IEEE 754 double-precision (`f64`).

- **Creation:** float literal `3.14`, integer-float mixed arithmetic, explicit conversion
- **Display:** Rust's default `f64` display (e.g. `3.14`, `1` displays as `1`)
- **type_name:** `"FLOAT"`

Operations that would produce NaN or ±Infinity produce an `Error`.

### 3.3 String

An immutable UTF-8 string.

- **Creation:** string literal, string interpolation, `+` concatenation, `fmt.to_str()`
- **Display:** the string content directly (no quotes)
- **type_name:** `"STRING"`

Supported operations:
- `+` with another String: concatenation → String
- `+` with a Char: append char → String
- `==`, `!=`: structural equality

Indexing a string with an integer returns the character at that position (0-based) as a `Char`. Negative indices produce an error.

### 3.4 Char

A single Unicode scalar value.

- **Creation:** char literal `'a'`, string indexing, `strings.to_chars()`
- **Display:** the character itself
- **type_name:** `"CHAR"`

Supported operations:
- `+` with Char: concatenation → String
- `==`, `!=`: equality

### 3.5 Bool

`true` or `false`.

- **Creation:** boolean literal, comparison operators, logical operators
- **Display:** `true` or `false`
- **type_name:** `"BOOL"`

Supported operations: `==`, `!=`. `&&` and `||` accept any truthy value (see §4.7).

### 3.6 Null

The absence of a value.

- **Creation:** `null` keyword, uninitialized `let x;`, absent hash key lookup, absent array element, functions that return nothing
- **Display:** `null`
- **type_name:** `"NULL"`

`null` is falsy. `!null` is `true`. `null ?? expr` evaluates `expr`.

### 3.7 Array

An ordered, heterogeneous, mutable sequence.

- **Creation:** array literal `[1, "two", true]`
- **Display:** `[1, two, true]`
- **type_name:** `"ARRAY"`

Arrays are indexed by non-negative integers (0-based). Negative indices produce an error. Index access returns the element; out-of-bounds access produces an error. Arrays are passed by value (they are cloned when assigned or passed to functions).

### 3.8 Hash

An ordered sequence of key-value pairs. Keys may be any value that compares equal (using the evaluator's `objects_equal`). Order is insertion order.

- **Creation:** hash literal `{ "key": value, ... }`
- **Display:** `{key: value, ...}`
- **type_name:** `"HASH"`

Key access via `h["key"]` searches linearly for the first pair whose key is equal to the index value. Missing keys return an `Error` (not `null`). Keys are typically strings but may be any type.

### 3.9 Function

A first-class closure.

- **Creation:** function literal `fn(params) { body }`
- **Display:** `fn(param1, param2, ...)`
- **type_name:** `"FUNCTION"`

Functions capture their defining environment by reference (shared `Rc<RefCell<Env>>`). See §7.

### 3.10 StructType

A struct definition, not an instance.

- **Creation:** `struct Name { field: default, ... };`
- **Display:** `struct Name`
- **type_name:** `"STRUCT_TYPE"`

A `StructType` is used to create instances via struct literal syntax. It holds default values for each field.

### 3.11 StructInstance

An instance of a struct, holding named fields.

- **Creation:** `Name { field: value, ... }`
- **Display:** `Name { field1: val1, field2: val2, ... }`
- **type_name:** `"STRUCT_INSTANCE"`

Fields are accessed with `.` notation. Fields may be any type, including functions (enabling self-methods, see §7.4).

### 3.12 Module

A collection of named values, either from the standard library or from a `.cl` file.

- **Creation:** `import "name"`
- **Display:** `[Module: name]`
- **type_name:** `"MODULE"`

Modules are accessed via `.` notation. Accessing a missing member produces an error. Standard library modules are `pub_gated: false` (error: "has no member"). User modules that used `pub` declarations are `pub_gated: true` (error: "has no public member").

### 3.13 EnumType

A set of named variants.

- **Creation:** `enum Name { Variant1, Variant2 };`
- **Display:** `Name(Variant1 | Variant2 | ...)`
- **type_name:** `"ENUM_TYPE"`

### 3.14 EnumVariant

A specific variant of an enum.

- **Creation:** `EnumName.VariantName`
- **Display:** `EnumName.VariantName`
- **type_name:** `"ENUM_VARIANT"`

Enum variants can be compared with `==` and `!=` and used in switch arms.

### 3.15 Error

A runtime error value.

- **Creation:** runtime error conditions (type mismatch, missing identifier, etc.)
- **Display:** `error: message`
- **type_name:** `"ERROR"`

Errors carry `message: String`, `line: usize`, `column: usize`. They propagate up the call stack and short-circuit evaluation (an error returned from any sub-expression stops evaluation of the containing expression). Errors are NOT exceptions — they can be stored in variables and tested with `is_error()`. See §9.

### 3.16 Builtin / BuiltinHigherOrder

Native Rust functions exposed to the language.

- **Display:** `[Builtin]`
- **type_name:** `"BUILTIN"`

`BuiltinHigherOrder` functions can call back into the evaluator and accept function values as arguments (used by `arrays.map`, `arrays.filter`, `arrays.reduce`, etc.).

---

## 4. Expressions

Expressions produce a value. Every expression listed in this section is evaluated eagerly unless noted.

### 4.1 Literals

| Syntax | Type produced |
|---|---|
| `42` | Integer |
| `3.14` | Float |
| `"hello"` | String |
| `'a'` | Char |
| `true` / `false` | Bool |
| `null` | Null |
| `[1, 2, 3]` | Array |
| `{ "k": v }` | Hash |

### 4.2 Identifiers

An identifier is looked up in the current environment, walking outward through enclosing scopes until found or the top-level scope is exhausted.

```
identifier → env.get(name)
```

If not found: `Error { "identifier not found: name" }`.

### 4.3 String interpolation

```cl
"text ${expr} more text"
```

Each `${expr}` segment is evaluated. The result is converted to its string display form and concatenated with the surrounding literal parts. If any segment produces an `Error`, that error is returned.

### 4.4 Prefix operators

| Operator | Operand type | Result |
|---|---|---|
| `!` | Bool `true` | Bool `false` |
| `!` | Bool `false` | Bool `true` |
| `!` | Null | Bool `true` |
| `!` | any other | Bool `false` |
| `-` | Integer | Integer (negated) |
| `-` | Float | Float (negated) |
| `-` | other | Error |

### 4.5 Infix operators

**Precedence table** (lowest to highest):

| Level | Operators | Associativity |
|---|---|---|
| 1 (lowest) | `??` | right |
| 2 | `=` `+=` `-=` `*=` `/=` `%=` | right |
| 3 | `\|\|` | left |
| 4 | `&&` | left |
| 5 | `==` `!=` | left |
| 6 | `<` `>` `<=` `>=` | left |
| 7 | `+` `-` | left |
| 8 | `*` `/` `%` | left |
| 9 | (prefix) | — |
| 10 | (postfix `++` `--`) | — |
| 11 | `(` (call) | left |
| 12 | `[` (index) | left |
| 13 (highest) | `.` (member) | left |

**Arithmetic operators** (`+`, `-`, `*`, `/`, `%`, `**`, `//`):

- Integer OP Integer → Integer (overflow produces Error; division/modulo by zero produces Error)
- Float OP Float → Float (NaN or Infinity produces Error)
- Integer OP Float → Float (integer promoted)
- Float OP Integer → Float (integer promoted)
- String `+` String → String (concatenation)
- String `+` Char → String (append char)
- Char `+` Char → String (concatenation)
- All other combinations → type mismatch Error

`**` (exponentiation): computed as `(l as f64).powf(r as f64)`. Result is checked for overflow before converting back to i64 for Integer operands.

`//` (floor division): `floor(l / r)` as integer.

**Comparison operators** (`==`, `!=`, `<`, `>`, `<=`, `>=`):

- `<`, `>`, `<=`, `>=` are defined for Integer and Float (with implicit promotion).
- `==` and `!=` are defined for Integer, Float, String, Bool, Char, Null, EnumVariant. All other type combinations return `false` for `==` and `true` for `!=`.
- Comparing values of different types (other than Integer/Float) is always unequal.

**Logical operators** (`&&`, `||`):

Both are short-circuit. They return the actual value that determined the result, not necessarily a Bool.

- `&&`: evaluates left; if falsy, returns left value. Otherwise evaluates and returns right value.
- `||`: evaluates left; if truthy, returns left value. Otherwise evaluates and returns right value.

**Truthiness:** `false` and `null` are falsy. Everything else is truthy.

### 4.6 Compound assignment

`x += expr` is equivalent to `x = x + expr`, computed using the same infix rules. The left side must be an identifier, a member expression, or an index expression. The current value is retrieved, the infix applied, and the result stored back.

For plain `=`, the right side is evaluated and stored without reading the current value.

### 4.7 Increment and decrement

```cl
x++    # postfix: returns current value, then increments
x--    # postfix: returns current value, then decrements
++x    # prefix: increments, then returns new value
--x    # prefix: decrements, then returns new value
```

Valid on: Integer, Float. Other types produce Error.

Valid targets: identifiers, member expressions (`obj.field++`), index expressions (`arr[i]++`). Negative indices on arrays produce Error.

### 4.8 Null coalescing (`??`)

```cl
left ?? right
```

Evaluates `left`. If the result is `null`, evaluates and returns `right`. Otherwise returns the left value. Right side is not evaluated unless left is null.

### 4.9 `typeof`

```cl
typeof expr
```

Evaluates `expr` and returns a lowercase string naming its type:

| Value | `typeof` result |
|---|---|
| Integer | `"integer"` |
| Float | `"float"` |
| String | `"string"` |
| Char | `"char"` |
| Bool | `"bool"` |
| Null | `"null"` |
| Array | `"array"` |
| Hash | `"hash"` |
| Function | `"function"` |
| StructType | `"struct_type"` |
| StructInstance | `"struct_instance"` |
| Module | `"module"` |
| EnumType | `"enum_type"` |
| EnumVariant | `"enum_variant"` |
| Error | `"error"` |

Note: `fmt.typeof(val)` returns the same string but in uppercase.

### 4.10 Index expression

```cl
expr[index]
```

**Array indexed by Integer:** Returns element at 0-based position. Negative indices produce Error. Out-of-bounds produces Error `"index N out of range (len M)"`.

**String indexed by Integer:** Returns `Char` at position. Negative indices or out-of-bounds produce Error.

**Hash indexed by any value:** Searches linearly for the first key equal to `index`. Returns the value if found. If not found, returns Error `"key X not found in hash"`.

All other combinations produce Error `"index operator not supported: TYPE"`.

### 4.11 Member access

```cl
object.property
```

- **StructInstance:** returns value of named field. Missing field → Error `"unknown field F on TypeName"`.
- **Module:** returns named member. Missing member → Error `"name has no member 'F'"` (or `"has no public member"` for pub-gated user modules).
- **Hash:** searches for string key equal to property name. Missing → Error `"property not found: F"`.
- **EnumType:** if property is a valid variant name, returns `EnumVariant { enum_name, variant }`. Otherwise Error.
- **Other types:** Error `"cannot access property F on TYPE"`.

### 4.12 Function call

```cl
fn_expr(arg1, arg2, ...)
```

Evaluates `fn_expr`, then evaluates each argument left-to-right, then calls `apply_function`. See §7.

If `fn_expr` is a member expression (`obj.method`), it may trigger self-method injection (§7.4).

### 4.13 Array literal

```cl
[expr1, expr2, ...]
```

Each element is evaluated left-to-right. If any element produces an Error, evaluation stops and the error is returned.

### 4.14 Hash literal

```cl
{ key_expr: val_expr, ... }
```

Each key and value is evaluated left-to-right in pair order. Keys may be any value. If any key or value produces an Error, evaluation stops and the error is returned.

### 4.15 Struct literal

```cl
StructName { field1: expr1, field2: expr2, ... }
```

Looks up `StructName` in the environment. Must be a `StructType`; otherwise Error. Creates a `StructInstance` by starting from the struct's default field values, then applying the provided field overrides. Any field not listed in the literal takes its default value. Fields not defined on the struct can be added (the override dict is merged into the defaults).

### 4.16 If / elseif / else

```cl
if condition { body }
if condition { body } elseif cond2 { body2 } else { body3 }
```

Evaluates `condition`. If truthy, evaluates and returns the consequence block. Otherwise checks each `elseif` condition in order. If one is truthy, evaluates and returns its body. If none matched and an `else` block exists, evaluates and returns it. If no branch matched and there is no `else`, returns Null.

### 4.17 While loop

```cl
while condition { body }
```

Evaluates `condition` before each iteration. If falsy, the loop ends. `break` exits the loop (returns Null). `continue` skips to the next condition check. `return` exits the enclosing function. Errors propagate out. The loop itself returns the value of the last iteration body, or Null.

### 4.18 C-style for loop

```cl
for let i = 0; i < 10; i++ { body }
```

Creates a new scope. Evaluates the init statement once, then repeatedly: checks condition (exits if falsy), evaluates body, evaluates post statement. `break`, `continue`, `return`, errors behave as in while. The post statement is skipped when `continue` is hit.

### 4.19 For-in loop

```cl
for item in iterable { body }
for key, value in hash_or_iterable { body }
```

**Array:** iterates elements, binding each to `item`. One-variable form only.

**String:** iterates characters, binding each `Char` to `item`. One-variable form only.

**Hash:** requires two-variable form (`key, value`). Iterates pairs in insertion order. Using one-variable form on a hash produces Error `"hash iteration requires two variables"`.

Other types produce Error `"cannot iterate over TYPE"`.

`break`, `continue`, `return`, errors behave as in while.

### 4.20 Switch

```cl
switch subject {
    pattern1 => { body1 },
    pattern2 => { body2 },
    _ => { default_body },
}
```

Evaluates `subject` once. Then evaluates each arm's pattern in order and compares it to the subject using structural equality (`objects_equal`). The first match executes its body and the result is returned. Subsequent arms are not checked.

The default arm uses the identifier `_` as its pattern. `_` is looked up in the environment; if not bound, it evaluates to a missing identifier Error — so `_` must not be used if the variable is not defined. Convention: use `_` as the default arm and ensure the identifier `_` is not bound.

> **Implementation note:** The default arm works because the switch compares with `objects_equal`, and an identifier `_` that is not bound produces an Error, which compares false against anything. This means `_` in switch only works as a default when `_` is not bound in scope.

If no arm matches, the switch expression returns Null.

---

## 5. Statements

Statements produce a value (used internally for `return` and `break`/`continue` signal propagation) but their value is generally discarded except in the REPL.

### 5.1 Expression statement

```cl
expr;
```

Evaluates `expr`. The semicolon is optional in some positions but required after most statements. In the REPL, the result value is printed if non-Null.

### 5.2 `let`

```cl
let name = expr;
let name;                         # uninitialized — value is null
let [a, b, c] = array_expr;      # array destructuring
let { key, key2: alias } = hash_or_struct_expr;  # hash destructuring
```

Binds `name` in the current environment. If no `= expr` is given, the value is `null`.

**Array destructuring:** The right side must be an Array. Each name is bound to the element at the corresponding index. If the array is shorter than the names list, excess names are bound to `null`. Use `_` to skip a position.

**Hash destructuring:** The right side must be a Hash or StructInstance. Each `key` is extracted from the value; if `key: alias` is given, the alias is the bound name; otherwise the key name itself is used. Missing keys produce `null`.

A `let` binding in a block scope shadows outer bindings; it does not update them.

### 5.3 `const`

```cl
const NAME = expr;
const [a, b] = array_expr;
const { key } = hash_expr;
```

Same as `let` but marks the binding as immutable. Attempting to reassign a `const` name produces Error `"cannot reassign constant 'NAME'"`.

> **Note:** The immutability check is per-name; it is enforced at the point of assignment (`=`). Compound assignment on a const also triggers this error.

### 5.4 `pub`

```cl
pub let name = expr;
pub const NAME = expr;
```

Evaluates the inner `let` or `const` normally, then marks `name` in the environment as publicly exported. Only `LetPattern::Ident` (plain name) is pub-markable; destructured patterns silently skip the pub-marking.

`pub` has no effect in the REPL or top-level execution — it only matters when the file is imported as a module.

### 5.5 `return`

```cl
return expr;
```

Evaluates `expr` and signals function exit with that value. Propagates up through `eval_statement` until caught by `apply_function`. If used outside a function body, the value propagates to the top-level evaluator and becomes the program's result.

Alternatively, the last expression in a function body is implicitly returned (no `return` keyword needed).

### 5.6 `break` and `continue`

```cl
break;
continue;
```

Valid only inside loop bodies. Using either outside a loop produces Error `"break outside of loop"` / `"continue outside of loop"`.

- `break`: exits the innermost loop. The loop expression returns Null.
- `continue`: skips the rest of the loop body and proceeds to the next iteration (for-in loops), next condition check (while), or post statement (C-for).

### 5.7 `import`

```cl
import "module_name";
```

Loads a module and binds it in the current environment under the name `module_name`. Two sources:

1. **Standard library:** if `module_name` matches a preloaded stdlib name (`fmt`, `arrays`, `strings`, `math`, `fs`, `hash`, `os`, `time`, `json`, `rand`, `path`, `http`), the module object is loaded from the cache.

2. **File module:** reads `module_name.cl` from the current working directory, parses it, and evaluates it in a new enclosed environment. If the file cannot be read → Error. If there are parse errors → Error listing all parse messages. If evaluation produces an Error → that error is returned. Otherwise, a `Module` object is constructed from the evaluated environment.

   If the module file uses any `pub` declarations, only the `pub`-marked names are exported (pub-gated module). If no `pub` declarations exist, all top-level names are exported.

Modules are cached; importing the same name twice returns the cached version.

### 5.8 `struct`

```cl
struct Name {
    field1: default_expr1,
    field2: default_expr2,
};
```

Evaluates all default expressions in the current environment. Creates a `StructType` value and binds it to `Name` in the current environment. The trailing `;` after `}` is required.

Field names are unordered (stored in a HashMap internally). All fields must have default values in the definition.

### 5.9 `enum`

```cl
enum Name { Variant1, Variant2, Variant3 };
```

Creates an `EnumType` with the listed variants and binds it to `Name`. The trailing `;` is required. Variants are strings internally. Enum variants are accessed via `Name.Variant`.

### 5.10 Block

```cl
{ stmt1; stmt2; expr }
```

A block creates a new enclosed scope. Statements are evaluated in order. If any statement produces `Return`, `Error`, `Break`, or `Continue`, evaluation stops and that signal is returned immediately. The block's value is the value of its last statement.

---

## 6. Scoping and Environments

### 6.1 Lexical scope

code-lang uses lexical (static) scoping. Each block, function body, for loop init, and for-in iteration creates a new enclosed environment. The enclosed environment has a reference to its parent, forming a scope chain.

### 6.2 Name lookup

`env.get(name)` searches the current scope's store first. If not found, it walks to the outer scope recursively until the top-level environment. If still not found, returns `None`, and the evaluator produces Error `"identifier not found: name"`.

### 6.3 Name assignment

`env.update(name, value)` searches the scope chain for an existing binding and updates it in place. If no existing binding is found, the assignment falls through to `env.set`, which creates a new binding in the **current** scope. This means `x = expr` where `x` is not yet bound creates a new local binding.

> This is important: `x = 5` inside a function body where `x` is not a parameter or local does NOT update an outer `x`; it creates a new local `x` in the innermost scope that doesn't have it.

Wait — actually `update` does walk the chain and mutate if found. The fallback to `set` in the evaluator only occurs in specific code paths (e.g., `++`). For plain assignment via `eval_assignment`, if `update` returns false (not found), `set` is called on the current scope. So plain `x = 5` where `x` doesn't exist creates it in the current scope.

### 6.4 Const immutability

`const` bindings record their name in `env.consts`. When `eval_assignment` is called for a name that is marked const anywhere in the scope chain, it returns Error `"cannot reassign constant 'name'"`.

### 6.5 Closures

When a function is created (`Expression::Function`), the current environment (`Rc<RefCell<Environment>>`) is captured by reference in the `Object::Function`. This is a shared reference — mutations to variables in the captured environment are visible through the closure.

---

## 7. Functions

### 7.1 Function literals

```cl
let add = fn(a, b) { a + b; };
let square = fn(x) { x * x; };
```

Creates a `Function` object capturing the current environment. Functions are values and can be stored, passed, and returned.

### 7.2 Default parameters

```cl
let greet = fn(name, greeting = "hello") {
    "${greeting}, ${name}!";
};
greet("Walon");          # → "hello, Walon!"
greet("Walon", "hi");   # → "hi, Walon!"
```

Parameters with defaults must come after parameters without defaults (convention; not enforced by the parser, but calling with fewer arguments than required non-default parameters produces Error).

Default expressions are evaluated **at call time** in the function's extended environment (not at definition time).

### 7.3 Arity rules

Given a function with `N` total parameters and `R` required (no-default) parameters:

- `args.len() > N` → Error `"wrong number of arguments: expected N, got M"`
- `args.len() < R` → Error `"missing arguments: expected at least R, got M"`
- `R <= args.len() <= N` → OK; missing optional parameters use their defaults

### 7.4 Self-methods

If a function is stored as a field on a struct, and its first parameter is named `self`, calling it through member syntax injects the receiver as the first argument automatically:

```cl
struct Counter {
    count: 0,
    increment: fn(self) { self.count + 1; }
};
let c = Counter {};
c.increment();   # self = c is injected; returns 1
```

The arity check and error messages subtract 1 for `self`, so the user sees their parameter count (excluding self).

### 7.5 Recursion limit

The interpreter limits call depth to **500**. Exceeding this produces Error `"maximum call depth exceeded (500)"`.

### 7.6 Return values

A function returns:
1. The value of an explicit `return expr;` statement, or
2. The value of the last expression in the function body (the last statement's value, if it is an `Expression` statement)

If the body ends on a statement that produces Null (like `let`, `struct`, etc.), the function returns Null.

---

## 8. Modules

### 8.1 Standard library modules

Standard library modules are pre-built Rust objects preloaded at interpreter startup. They are not loaded from disk. They are accessed by importing their name:

```cl
import "fmt";
import "arrays";
```

Available stdlib modules: `fmt`, `arrays`, `strings`, `math`, `fs`, `hash`, `os`, `time`, `json`, `rand`, `path`, `http`.

All stdlib module members are publicly accessible (not pub-gated). The error for a missing member is `"modname has no member 'x'"`.

### 8.2 User modules

A `.cl` file can be imported as a module:

```cl
import "utils";   # loads ./utils.cl
```

The file is read from disk, parsed, and evaluated. The resulting environment becomes the module's member set. If any `pub` declarations exist in the file, only those names are exported:

```cl
# utils.cl
pub let version = "1.0";
pub fn helper() { "ok"; }
let _private = "hidden";   # not exported
```

Importing this file: `utils.version` works; `utils._private` → Error `"utils has no public member '_private'"`.

If no `pub` declarations are present, all top-level names are exported (backward-compatible behavior).

### 8.3 Module caching

Both stdlib and user modules are cached in the evaluator. Importing the same name a second time returns the cached module without re-reading or re-evaluating the file.

---

## 9. Error Model

code-lang uses **error-as-value** semantics, not exceptions.

### 9.1 Error values

`Object::Error { message, line, column }` is a runtime value. It carries:
- `message`: human-readable description
- `line` / `column`: source position of the error (1-based line, 1-based column)

Builtins emit errors with `line: 0, column: 0`; these are stamped with the call-site position by `apply_function`.

### 9.2 Error propagation

Every evaluator function checks its sub-expressions for errors and returns them immediately. This means an error in a deeply nested expression propagates back to the top level without any intermediate code running.

Example:
```cl
let x = 1 + undefined_var;  # error propagates — x is never bound
```

### 9.3 Catching errors

The global function `is_error(val)` returns `true` if `val` is an Error.

```cl
let result = some_function();
if is_error(result) {
    fmt.print("failed: ${result}");
} else {
    fmt.print(result);
}
```

Note: storing an error in a variable does **not** propagate it — errors only propagate when they are the return value of an expression that feeds into further computation. An error stored in a `let` binding is just a value.

### 9.4 Error display

The REPL and `execute` function display errors with:
1. `error: message`
2. A source excerpt with a caret pointing to the error column (if line > 0)
3. A `hint:` line if a fix hint is available for the error message pattern

---

## 10. Standard Library

All stdlib modules must be imported before use. Member access on an unimported module produces `"identifier not found"`.

---

### 10.1 `fmt`

Formatting, conversion, and I/O.

| Function | Signature | Description |
|---|---|---|
| `fmt.print` | `(val, ...)` | Print values to stdout separated by space, with newline |
| `fmt.eprint` | `(val, ...)` | Print to stderr with newline |
| `fmt.to_str` | `(val)` | Convert any value to its string display form |
| `fmt.to_int` | `(val)` | Convert Integer/Float/Bool/String → Integer. String is parsed. Error on failure. |
| `fmt.to_float` | `(val)` | Convert Integer/Float/String → Float. String is parsed. Error on failure. |
| `fmt.input` | `(prompt: String)` | Print prompt (no newline), read one line from stdin, return String (trimmed of trailing newline) |
| `fmt.clear` | `()` | Clear terminal (runs `clear` on Unix, `cls` on Windows) |
| `fmt.format` | `(template, ...)` | Printf-style formatting: `%s` any value, `%d` integer, `%f` float, `%%` literal `%` |
| `fmt.typeof` | `(val)` | Returns uppercase type name string (e.g. `"INTEGER"`) |

`fmt.print` accepts multiple arguments; all are formatted and joined by space. `fmt.to_int(true)` → `1`; `fmt.to_int(false)` → `0`.

---

### 10.2 `arrays`

Array operations. All functions return new arrays rather than mutating in place.

| Function | Signature | Description |
|---|---|---|
| `arrays.len` | `(arr)` | Length of array (also accepts String) |
| `arrays.first` | `(arr)` | First element, or null if empty |
| `arrays.last` | `(arr)` | Last element, or null if empty |
| `arrays.rest` | `(arr)` | All elements after the first; null if empty |
| `arrays.push` | `(arr, val)` | New array with `val` appended |
| `arrays.pop` | `(arr)` | New array with last element removed; null if empty |
| `arrays.prepend` | `(arr, val)` | New array with `val` prepended |
| `arrays.reverse` | `(arr)` | Reversed copy |
| `arrays.contains` | `(arr, val)` | Bool: true if `val` appears in array |
| `arrays.index_of` | `(arr, val)` | 0-based index of first occurrence, or -1 |
| `arrays.slice` | `(arr, start, end)` | Sub-array `[start, end)`. Indices clamped to bounds. |
| `arrays.join` | `(arr, sep: String)` | Concatenate all elements to string with separator |
| `arrays.concat` | `(arr1, arr2)` | Concatenate two arrays |
| `arrays.sum` | `(arr)` | Numeric sum of all elements |
| `arrays.min` | `(arr)` | Minimum numeric element |
| `arrays.max` | `(arr)` | Maximum numeric element |
| `arrays.flatten` | `(arr)` | Flatten one level of nested arrays |
| `arrays.sort` | `(arr)` | Sorted copy (numeric/string comparison) |
| `arrays.unique` | `(arr)` | Deduplicated copy (first occurrence kept) |
| `arrays.zip` | `(arr1, arr2)` | Array of `[a, b]` pairs, length of shorter |
| `arrays.map` | `(arr, fn)` | New array of `fn(element)` results |
| `arrays.filter` | `(arr, fn)` | Elements where `fn(element)` is truthy |
| `arrays.reduce` | `(arr, fn, init)` | Left fold: `fn(acc, element)` starting from `init` |
| `arrays.find` | `(arr, fn)` | First element where `fn(element)` is truthy, or null |
| `arrays.any` | `(arr, fn)` | Bool: true if any element passes |
| `arrays.all` | `(arr, fn)` | Bool: true if all elements pass |

---

### 10.3 `strings`

String manipulation. All functions return new values.

| Function | Signature | Description |
|---|---|---|
| `strings.len` | `(s)` | Character count (UTF-8 char count, not bytes) |
| `strings.to_upper` | `(s)` | Uppercase copy |
| `strings.to_lower` | `(s)` | Lowercase copy |
| `strings.split` | `(s, sep)` | Split by separator → Array of Strings |
| `strings.join` | `(arr, sep)` | Join array of strings with separator |
| `strings.contains` | `(s, sub)` | Bool |
| `strings.replace` | `(s, from, to)` | Replace first occurrence of `from` with `to` |
| `strings.trim` | `(s)` | Strip leading and trailing whitespace |
| `strings.trim_left` | `(s)` | Strip leading whitespace |
| `strings.trim_right` | `(s)` | Strip trailing whitespace |
| `strings.starts_with` | `(s, prefix)` | Bool |
| `strings.ends_with` | `(s, suffix)` | Bool |
| `strings.index` | `(s, sub)` | Byte index of first occurrence, or -1 |
| `strings.count` | `(s, sub)` | Count of non-overlapping occurrences |
| `strings.repeat` | `(s, n)` | Repeat string `n` times |
| `strings.reverse` | `(s)` | Reversed string |
| `strings.to_chars` | `(s)` | String → Array of Chars |
| `strings.from_chars` | `(arr)` | Array of Chars → String |
| `strings.parse_int` | `(s)` | Parse string → Integer. Error on failure. |
| `strings.parse_float` | `(s)` | Parse string → Float. Error on failure. |
| `strings.lines` | `(s)` | Split by newline → Array of Strings |
| `strings.is_empty` | `(s)` | Bool: true if length is 0 |
| `strings.pad_left` | `(s, n, ch)` | Left-pad to width `n` with char `ch` |
| `strings.pad_right` | `(s, n, ch)` | Right-pad to width `n` with char `ch` |

---

### 10.4 `math`

Mathematical functions. Most operate on floats; integer arguments are promoted.

| Member | Type | Description |
|---|---|---|
| `math.PI` | Float | π ≈ 3.14159265358979 |
| `math.E` | Float | e ≈ 2.71828182845905 |
| `math.sqrt(n)` | fn | Square root. Negative input → Error. |
| `math.abs(n)` | fn | Absolute value |
| `math.floor(n)` | fn | Floor |
| `math.ceil(n)` | fn | Ceiling |
| `math.round(n)` | fn | Round to nearest integer |
| `math.trunc(n)` | fn | Truncate towards zero |
| `math.pow(base, exp)` | fn | `base ** exp`. NaN/Infinity → Error. |
| `math.log(n)` | fn | Natural logarithm. `n <= 0` → Error. |
| `math.log10(n)` | fn | Log base 10. `n <= 0` → Error. |
| `math.log2(n)` | fn | Log base 2. `n <= 0` → Error. |
| `math.exp(n)` | fn | e^n. Overflow → Error. |
| `math.sin(n)` | fn | Sine (radians) |
| `math.cos(n)` | fn | Cosine (radians) |
| `math.tan(n)` | fn | Tangent (radians). Asymptotes → Error. |
| `math.min(a, b)` | fn | Minimum of two numeric values |
| `math.max(a, b)` | fn | Maximum of two numeric values |
| `math.clamp(n, lo, hi)` | fn | Clamp `n` to `[lo, hi]` |
| `math.sign(n)` | fn | -1, 0, or 1 |
| `math.gcd(a, b)` | fn | Greatest common divisor (Euclidean) |
| `math.lcm(a, b)` | fn | Least common multiple |

---

### 10.5 `hash`

Hash (dictionary) operations.

| Function | Signature | Description |
|---|---|---|
| `hash.keys` | `(h)` | Array of all keys |
| `hash.values` | `(h)` | Array of all values |
| `hash.entries` | `(h)` | Array of `[key, value]` pairs |
| `hash.has_key` | `(h, key)` | Bool |
| `hash.get` | `(h, key, default)` | Value for key, or `default` if missing |
| `hash.merge` | `(a, b)` | New hash with all pairs from `a` then `b` |
| `hash.delete` | `(h, key)` | New hash with `key` removed |
| `hash.len` | `(h)` | Number of key-value pairs |

---

### 10.6 `fs`

File system operations.

| Function | Signature | Description |
|---|---|---|
| `fs.read_file` | `(path)` | Read file → String. Error on I/O failure. |
| `fs.write_file` | `(path, content)` | Write string to file (truncates). Error on failure. |
| `fs.append_file` | `(path, content)` | Append string to file. Error on failure. |
| `fs.read_lines` | `(path)` | Read file → Array of Strings (one per line) |
| `fs.exists` | `(path)` | Bool |
| `fs.is_file` | `(path)` | Bool |
| `fs.is_dir` | `(path)` | Bool |
| `fs.list_dir` | `(path)` | Array of filenames in directory |
| `fs.mkdir` | `(path)` | Create directory |
| `fs.mkdir_all` | `(path)` | Create directory and all parents |
| `fs.remove` | `(path)` | Delete file |
| `fs.remove_dir` | `(path)` | Delete directory |
| `fs.copy` | `(src, dst)` | Copy file |
| `fs.rename` | `(src, dst)` | Rename or move file |

---

### 10.7 `os`

Operating system interface.

| Function | Signature | Description |
|---|---|---|
| `os.args` | `()` | Command-line arguments → Array of Strings |
| `os.platform` | `()` | OS name string (e.g. `"linux"`) |
| `os.arch` | `()` | CPU architecture (e.g. `"x86_64"`) |
| `os.get_env` | `(key)` | Read environment variable → String or null |
| `os.set_env` | `(key, val)` | Set environment variable |
| `os.get_wd` | `()` | Current working directory → String |
| `os.exit` | `(code)` | Exit process with integer code |
| `os.hostname` | `()` | Machine hostname → String |

---

### 10.8 `time`

Date and time.

| Function | Signature | Description |
|---|---|---|
| `time.now` | `()` | Current local time as formatted String |
| `time.unix` | `()` | Unix timestamp → Integer (seconds since epoch) |
| `time.sleep` | `(ms)` | Sleep for `ms` milliseconds |
| `time.since` | `(unix_ts)` | Seconds elapsed since `unix_ts` → Float |
| `time.format` | `(ts, fmt)` | Format timestamp string using format specifier |
| `time.year` | `()` | Current year → Integer |
| `time.month` | `()` | Current month → Integer (1–12) |
| `time.day` | `()` | Current day of month → Integer |
| `time.hour` | `()` | Current hour → Integer (0–23) |
| `time.minute` | `()` | Current minute → Integer |
| `time.second` | `()` | Current second → Integer |

---

### 10.9 `json`

| Function | Signature | Description |
|---|---|---|
| `json.parse` | `(s)` | Parse JSON string → Hash/Array/primitive. Error on malformed input. |
| `json.stringify` | `(val)` | Serialize value → JSON String. |

---

### 10.10 `rand`

| Function | Signature | Description |
|---|---|---|
| `rand.int` | `(min, max)` | Random integer in `[min, max]` |
| `rand.float` | `()` | Random Float in `[0.0, 1.0)` |
| `rand.choice` | `(arr)` | Random element from Array |
| `rand.shuffle` | `(arr)` | Shuffled copy of Array |

---

### 10.11 `path`

| Function | Signature | Description |
|---|---|---|
| `path.join` | `(a, b)` | Join path segments with OS separator |
| `path.basename` | `(p)` | Filename with extension |
| `path.dirname` | `(p)` | Parent directory |
| `path.extension` | `(p)` | File extension (e.g. `"cl"`) |
| `path.stem` | `(p)` | Filename without extension |
| `path.absolute` | `(p)` | Resolve to absolute path |
| `path.is_absolute` | `(p)` | Bool |

---

### 10.12 `http`

| Function | Signature | Description |
|---|---|---|
| `http.get` | `(url)` | HTTP GET → String body. Error on failure. |
| `http.post` | `(url, body)` | HTTP POST with string body → String response. |
| `http.post_json` | `(url, hash)` | HTTP POST with hash serialized as JSON → String response. |

---

### 10.13 Global functions

These are available without importing any module:

| Function | Signature | Description |
|---|---|---|
| `is_error` | `(val)` | Bool: true if `val` is an Error object |

---

## 11. Grammar

Full EBNF grammar derived from the parser. `{ x }` means zero or more, `[ x ]` means optional, `( x | y )` means one of.

```ebnf
program         = { statement } EOF ;

statement       = let_stmt
                | const_stmt
                | pub_stmt
                | return_stmt
                | import_stmt
                | struct_stmt
                | enum_stmt
                | expr_stmt
                | break_stmt
                | continue_stmt
                ;

let_stmt        = "let" let_pattern [ "=" expression ] ";" ;
const_stmt      = "const" let_pattern "=" expression ";" ;
pub_stmt        = "pub" ( let_stmt | const_stmt ) ;
return_stmt     = "return" expression ";" ;
import_stmt     = "import" string_literal ";" ;
struct_stmt     = "struct" IDENT "{" { IDENT ":" expression "," } "}" ";" ;
enum_stmt       = "enum" IDENT "{" IDENT { "," IDENT } "}" ";" ;
break_stmt      = "break" ";" ;
continue_stmt   = "continue" ";" ;
expr_stmt       = expression [ ";" ] ;

let_pattern     = IDENT
                | "[" IDENT { "," IDENT } "]"
                | "{" hash_pattern { "," hash_pattern } "}"
                ;
hash_pattern    = IDENT [ ":" IDENT ] ;

expression      = null_coalesce_expr ;

null_coalesce_expr = or_expr [ "??" null_coalesce_expr ] ;
or_expr         = and_expr { "||" and_expr } ;
and_expr        = eq_expr { "&&" eq_expr } ;
eq_expr         = cmp_expr { ( "==" | "!=" ) cmp_expr } ;
cmp_expr        = sum_expr { ( "<" | ">" | "<=" | ">=" ) sum_expr } ;
sum_expr        = prod_expr { ( "+" | "-" ) prod_expr } ;
prod_expr       = prefix_expr { ( "*" | "/" | "%" | "//" | "**" ) prefix_expr } ;
prefix_expr     = ( "!" | "-" | "++" | "--" ) prefix_expr
                | postfix_expr
                ;
postfix_expr    = call_expr { "++" | "--" } ;
call_expr       = index_expr { "(" [ arg_list ] ")" } ;
index_expr      = member_expr { "[" expression "]" } ;
member_expr     = primary { "." IDENT } ;

primary         = INTEGER
                | FLOAT
                | string_literal
                | CHAR
                | "true" | "false"
                | "null"
                | IDENT
                | "(" expression ")"
                | "[" [ expr_list ] "]"
                | "{" [ hash_pair { "," hash_pair } ] "}"
                | fn_literal
                | if_expr
                | while_expr
                | for_expr
                | for_in_expr
                | switch_expr
                | typeof_expr
                ;

fn_literal      = "fn" "(" [ param_list ] ")" block ;
param_list      = param { "," param } ;
param           = IDENT [ "=" expression ] ;

if_expr         = "if" expression block { "elseif" expression block } [ "else" block ] ;
while_expr      = "while" expression block ;
for_expr        = "for" ( let_stmt | const_stmt ) expression ";" ( expr_stmt | update_stmt ) block ;
for_in_expr     = "for" IDENT [ "," IDENT ] "in" expression block ;
switch_expr     = "switch" expression "{" { switch_arm } "}" ;
switch_arm      = expression "=>" block [ "," ] ;
typeof_expr     = "typeof" expression ;

block           = "{" { statement } "}" ;
arg_list        = expression { "," expression } ;
expr_list       = expression { "," expression } ;
hash_pair       = expression ":" expression ;

string_literal  = '"' { CHAR | "${" expression "}" } '"' ;

INTEGER         = digit { digit } ;
FLOAT           = digit { digit } "." digit { digit }
                | "." digit { digit } ;
CHAR            = "'" char "'" ;
IDENT           = letter { letter | digit } ;
```

> **Note on struct literal parsing:** A bare identifier followed by `{` is parsed as a struct literal (`Point { x: 1 }`), not as an expression followed by a block. This check occurs after the Pratt expression loop. Only a bare identifier (not a chained expression) triggers this path.

---

*End of specification.*
