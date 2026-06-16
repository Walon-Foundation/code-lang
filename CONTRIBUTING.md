# Contributing to Code-Lang

Thank you for taking the time to contribute. All types of contributions are welcome — bug reports, documentation fixes, new standard library modules, and core language features.

---

## Development Setup

### Prerequisites

- [Rust](https://rustup.rs/) (stable toolchain)
- [Git](https://git-scm.com/)

### Getting started

```bash
git clone https://github.com/Walon-Foundation/code-lang.git
cd code-lang
cargo build
cargo test
```

---

## Project Structure

```
src/
├── token/      token definitions
├── lexer/      converts source text into tokens
├── ast/        AST node types
├── parser/     converts tokens into an AST
├── object/     runtime value types and environment
├── evaluator/  tree-walks the AST and executes it
└── repl/       interactive shell entry point

legacy/         original Go implementation (reference only)
docs/           language website (Next.js)
```

---

## Testing

Run all tests:

```bash
cargo test
```

Run tests for a specific module:

```bash
cargo test lexer
```

If you add a feature or fix a bug, include a test that covers it.

---

## Adding to the Standard Library

Standard library modules live in `src/std/` (coming soon). Each module is a Rust function that returns a `HashMap<String, Object>` of named builtins.

The general pattern:

```rust
pub fn module() -> HashMap<String, Object> {
    let mut m = HashMap::new();
    m.insert("hello".to_string(), Object::Builtin(|_args| {
        Object::StringType("world".to_string())
    }));
    m
}
```

Register it in the evaluator's builtin loader so `import "your_module"` works.

---

## Pull Request Process

1. Fork the repo and create a branch: `git checkout -b feat/your-feature`
2. Make your changes and add tests
3. Ensure `cargo build` and `cargo test` pass
4. Open a pull request with a clear description of what changed and why

Keep commits focused. One logical change per commit.

---

## Reporting Issues

Open an issue at [github.com/Walon-Foundation/code-lang/issues](https://github.com/Walon-Foundation/code-lang/issues).

For security vulnerabilities, see [SECURITY.md](SECURITY.md) — do not open a public issue.

---

## Communication

For larger changes or design questions, open a GitHub Discussion before writing code. It saves everyone time.
