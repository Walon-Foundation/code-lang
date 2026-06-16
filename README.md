# Code-Lang

Code-Lang is a general-purpose interpreted programming language written in Rust. It features a clean syntax, first-class functions, structs, a module system, and a growing standard library.

> **Status:** Active development. The core language is functional but not production-ready.

---

## Features

- **Types:** integers, floats, strings, characters, booleans, arrays, hashes, structs
- **Functions:** first-class, closures, higher-order
- **Control flow:** `if / elseif / else`, `while`, `for`, `break`, `continue`
- **Operators:** arithmetic (`+`, `-`, `*`, `/`, `%`, `**`, `//`), comparison, logical (`&&`, `||`, `!`) with short-circuit evaluation, compound assignment (`+=`, `-=`, `*=`, `/=`, `%=`)
- **Structs:** custom types with default field values and dot-notation access
- **Module system:** import `.cl` files or built-in standard library modules
- **Standard library:** `fmt`, `math`, `strings`, `arrays`, `hash`, `json`, `fs`, `os`, `time`, `net`, `http`
- **REPL:** interactive shell with line/column error tracking
- **LSP:** language server providing auto-completion, hover, diagnostics, go-to-definition *(separate repo)*

---

## Getting Started

### Prerequisites

- [Rust](https://rustup.rs/) (stable)

### Build from source

```bash
git clone https://github.com/Walon-Foundation/code-lang.git
cd code-lang
cargo build --release
```

The binary is at `target/release/code-lang`.

### Run the REPL

```bash
cargo run
```

### Run a script

```bash
cargo run -- hello.cl
```

---

## Language Quick Reference

### Variables and functions

```cl
let age = 25;
const PI = 3.14159;

let add = fn(a, b) {
    return a + b;
};

add(10, 15);
```

### Arrays and hashes

```cl
let nums = [1, 2, 3, 5, 8];
nums[2];

let person = {"name": "Alice", "age": 30};
person.name;
```

### Structs

```cl
struct User {
    name: "Guest",
    role: "user",
}

let u = User { name: "Walon", role: "admin" };
let guest = User {};

u.name;     # Walon
guest.name; # Guest
```

### Control flow

```cl
let x = 10;

if (x > 10) {
    "greater"
} elseif (x == 10) {
    "equal"
} else {
    "less"
};

while (x > 0) {
    x -= 1;
};

for (let i = 0; i < 5; i++) {
    if (i == 3) { break; };
};
```

### Modules

```cl
import "math";
import "strings";

math.pow(2, 10);
strings.to_upper("hello");
```

---

## Roadmap

| Feature | Status |
|---|---|
| Lexer, Parser, AST | Done |
| Tree-walking evaluator | Done |
| Structs | Done |
| Module / import system | Done |
| Standard library (core modules) | In progress |
| Bytecode compiler | Planned |
| Bytecode VM | Planned |
| LSP (language server) | In progress |
| Formatter | Planned |
| VS Code extension | In progress |
| Docs site | In progress |

---

## Project Layout

```
code-lang/
├── src/            Rust source (lexer, parser, evaluator, repl)
├── legacy/         Original Go implementation (reference)
├── docs/           Language website (Next.js)
├── Cargo.toml
└── LICENSE
```

---

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md).

## Security

See [SECURITY.md](SECURITY.md).

## License

MIT — see [LICENSE](LICENSE).
