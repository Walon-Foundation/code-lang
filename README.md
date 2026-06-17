# code-lang

A general-purpose interpreted programming language written in Rust. Clean syntax, first-class functions, structs, a module system, and a growing standard library.

> **Status:** Active development — core language is functional, not production-ready.

## Install

Requires [Rust](https://rustup.rs/) (stable).

```bash
git clone https://github.com/Walon-Foundation/code-lang.git
cd code-lang
cargo build --release
```

## Use

```bash
# REPL
./target/release/code-lang

# Run a script
./target/release/code-lang hello.cl
```

## Quick example

```cl
import "fmt";
import "math";

let greet = fn(name) {
    fmt.print("Hello, " + name + "!");
};

greet("world");
fmt.print("sqrt(16) =", math.sqrt(16));
```

## Documentation

Full language reference and stdlib docs: **[docs/](./docs)**

## Project layout

```
code-lang/
├── src/        Rust source (lexer, parser, evaluator, stdlib, repl)
├── legacy/     Original Go implementation (reference)
├── docs/       Docs website (Next.js)
└── Cargo.toml
```

## License

MIT — see [LICENSE](LICENSE).
