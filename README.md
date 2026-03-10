# Code-Lang 🚀

Code-Lang is a modern, interpreted programming language written in Go. It began as an implementation following the excellent book **"Writing An Interpreter In Go"** by [Thorsten Ball](https://interpreterbook.com/), and has since evolved with additional features and custom extensions.

> [!IMPORTANT]
> **Status:** Code-Lang is a **passion project** and is currently under active development. While the core language is functional, it is not production-ready. If you intend to use this in a production environment, significant work, security audits, and optimizations are required.
> 
> 🚀 **Note:** This project is actively evolving and will soon be moved to the [Walon-Foundation](https://github.com/Walon-Foundation)!

---

## ✨ Features

- **Rich Type System:**
  - Integers and Floats
  - Strings and Characters
  - Booleans
  - Arrays (e.g., `[1, 2, 3]`)
  - Hashes/Dictionaries (e.g., `{"name": "Code-Lang"}`)
  - **Structs:** Custom data structures with default values and member access.
- **First-Class Functions:** Function literals, closures, and higher-order functions.
- **Control Flow:**
  - `if-elseif-else` expressions (everything is an expression!).
  - `while` loops for simple iteration.
  - `for` loops for structured iteration.
  - `break` and `continue` inside loops.
- **Static Analysis:**
  - **Symbol Table:** Tracks variable scopes, identifier resolution, and constant enforcement.
  - **Pre-execution Checks:** Catches undefined variables and illegal reassignments before running code.
- **Support for Comments:** Single-line (`#`) and multi-line (`/* */`).
- **Standard Operators:**
  - Arithmetic: `+`, `-`, `*`, `/`, `%` (Modulo)
  - Advanced: `**` (Power), `//` (Floor Division)
  - Comparison: `==`, `!=`, `<`, `>`, `<=`, `>=`
  - Logical: `&&` (AND), `||` (OR) — **with short-circuit evaluation** — and `!` (Negation)
  - Compound Assignment: `+=`, `-=`, `*=`, `/=`, `%=`, `**=`, `//=`
- **Built-in Functions:** `print`, `printf`, `typeof`, `len`, `push`, and more.
- **Module System:** Import other `.cl` files or built-in modules using `import "module"`.
- **Member Access:** Dot notation (`obj.prop`) for Hashes, Modules, Structs, and Servers.
- **Networking:** Built-in `http` client (GET, POST, etc.) and `net.server` for creating web servers.
- **JSON Support:** Built-in `json.parse()` and `json.stringify()`.
- **Standard Library:** Go-backed modules for `math`, `strings`, `time`, `hash`, `os`, `json`, and `net`.
- **REPL:** Interactive shell with persistent history and precise line/column error tracking.
- **File Execution:** Run scripts with the `.cl` extension.
- **Bytecode VM (WIP):** A compiler + VM backend now powers the REPL and file execution.
  - Supports arithmetic, `%`, `**`, `//`, comparisons (`==`, `!=`, `>`, `<`, `>=`, `<=`) with float support.
  - Supports assignment and updates: `=`, `+=`, `-=`, `*=`, `/=`, `%=`, `**=`, `//=`, `++`, `--`.
  - Current limitation: VM uses a global-only store (locals/scopes are still WIP in the VM).
- **Language Server Protocol (LSP):** Built-in Language Server providing IDE-like features:
  - Auto-completion, Hover previews, and live Diagnostics.
  - Go to Definition / Declaration / Implementation.
  - Find References, Rename variables, and Document Symbols.
  - Quickfix Code Actions (e.g., fixing undefined variables).

---

## 🚀 Getting Started

### Prerequisites

- [Go](https://golang.org/dl/) (version 1.21 or higher recommended)

### Installation

#### Option 1: Using `go install` (Recommended)
You can install the **code-lang** binary directly to your `$GOPATH/bin`:

```bash
go install github.com/walonCode/code-lang@latest
```

#### Option 2: Pre-built Binaries
Head over to the [Releases](https://github.com/walonCode/code-lang/releases) section to download path-ready binaries for Windows, macOS, and Linux.

#### Option 3: From Source
Clone and build manually:

```bash
git clone https://github.com/walonCode/code-lang.git
cd code-lang
go build -o code-lang main.go
```

### Running the REPL

Start the interactive shell by running:

```bash
go run main.go
```

### Running a Script

You can execute a Code-Lang script by passing the filename as an argument:

```bash
go run main.go hello.cl
```

### Running the Language Server (LSP)

The project now includes an LSP server executable that provides robust IDE features for Code-Lang! You can build the Language Server using the provided build script:

```bash
# Build the LSP using the script
./build_lsp

# Run the LSP (operates over stdin / stdout for editors like VS Code)
./lsp
```

### Visual Studio Code Extension

A dedicated VS Code extension is currently in the works to provide syntax highlighting and deep integration with the Code-Lang Language Server! You can find the repository and follow its development here:

👉 [Walon-Foundation/vscode_code-lang](https://github.com/Walon-Foundation/vscode_code-lang)

---

## 📖 Language Syntax at a Glance

### Variables & Functions

```rust
let age = 25;
let name = "Developer";
let isLearning = true;

let add = fn(a, b) {
    return a + b;
};

const PI = 3.14159;
# PI = 3.14; # Error: cannot reassign to const

print(add(10, 15)); # Output: 25
```

### Arrays and Hashes

```rust
let fibonacci = [0, 1, 1, 2, 3, 5, 8];
print(fibonacci[3]); # 2

let person = {"name": "Alice", "age": 30};
print(person.name); # Alice
```

### Structs

```rust
struct User {
    name: "Guest",
    role: "User",
}

let u = User { name: "Walon", role: "Admin" };
let guest = User {}; # Uses default values

print(u.name);     # Walon
print(guest.name); # Guest
```

### Conditionals

```rust
let x = 10;
let result = if (x > 10) {
    "Greater"
} elseif (x == 10) {
    "Equal"
} else {
    "Smaller"
};
```

### Logical Operators (`&&` / `||`)

`&&` and `||` use **short-circuit evaluation** — the right side is only evaluated when necessary.

```rust
let a = true;
let b = false;

print(a && b);  # false
print(a || b);  # true
print(!a);      # false

# Short-circuit: the right side is never evaluated when
# the result is already known from the left side.
let x = false && someUndefinedFn(); # safe — right side skipped
let y = true  || someUndefinedFn(); # safe — right side skipped

# Combine with comparisons
let age = 20;
let hasId = true;
if (age >= 18 && hasId) {
    print("Access granted");
};
```

### Loops

```rust
# While loop
let i = 0;
while (i < 5) {
    print(i);
    i += 1;
};

# For loop with break and continue
for (let j = 0; j < 10; j += 1) {
    if (j == 2) { continue; };
    if (j == 6) { break; };
    print(j);
};
```

### Standard Library Examples

#### Networking & JSON
```rust
import "http";
import "json";

let res = http.get("https://jsonplaceholder.typicode.com/todos/1");
let data = json.parse(res.body);
print(data.title);
```

#### Math & Time
```rust
import "math";
import "time";

let radius = 10;
let area = math.PI * math.pow(radius, 2);
print("Area:", math.round(area));

let start = time.now();
time.sleep(100);
print("Elapsed (ms):", time.since(start));
```

#### Strings & Hashes
```rust
import "strings";
import "hash";

let s = "  hello world  ";
print(strings.trim(strings.to_upper(s))); # HELLO WORLD

let user = {"name": "walon", "age": 25};
if (hash.has_key(user, "name")) {
    print("User keys:", hash.keys(user));
};
```

#### OS & Environment
```rust
import "os";

print("Platform:", os.platform);
print("API Key:", os.get_env("API_KEY"));
os.exit(0);
```

### Miscellaneous

```rust
# Single-line comment
/* 
   Multi-line comment 
*/

# Formatted print
let name = "Alice";
printf("Hello, %s!\n", name);

# Type checking
print(typeof(10));   # INTEGER
print(typeof("hi")); # STRING
print(typeof([]));   # ARRAY
```

---

## 🗺 Roadmap

| Feature | Status |
|---|---|
| Better Error Reporting (line & column tracking) | ✅ Done |
| Comments (single & multi-line) | ✅ Done |
| `while` and `for` loops with `break`/`continue` | ✅ Done |
| Logical Operators `&&` / `||` with short-circuiting | ✅ Done |
| Standard Library (`math`, `strings`, `time`, `hash`, `os`, `json`, `net`) | ✅ Done |
| Import System (`.cl` files) | ✅ Done |
| Member Access (dot notation) | ✅ Done |
| Compound Assignment (`+=`, `-=`, etc.) | ✅ Done |
| Structs (define custom types & create instances) | ✅ Done |
| Constants (`const`) | ✅ Done |
| Static Analysis (Symbol Table & Scope Awareness) | ✅ Done |
| Web Server (request/response handling) | 🚧 WIP |
| Struct Methods | 🔜 Planned |
| `fs` module (file system access) | 🔜 Planned |
| REPL Multi-line Support | 🔜 Planned |
| VSCode Extension (syntax highlighting) | 🚧 WIP |
| LSP (Language Server Protocol) | 🚧 WIP |

---

## 📜 License

This project is licensed under the **MIT License**. See the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- **Thorsten Ball** for the foundational guide [Writing An Interpreter In Go](https://interpreterbook.com/).
- The Go community for providing an incredible ecosystem for language development.
