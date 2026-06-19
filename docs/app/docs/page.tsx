import type { Metadata } from "next";
import Link from "next/link";
import Pre from "../components/Pre";

export const metadata: Metadata = { title: "Getting started" };

export default function GettingStarted() {
  return (
    <>
      <h1>Getting started</h1>
      <p>
        code-lang is an interpreted language with a clean syntax and a complete standard library.
        This guide gets you from zero to running your first script.
      </p>

      <h2 id="install">Install</h2>
      <p>
        Build from source with Cargo. The repo is a Cargo workspace — the interpreter lives at the root.
      </p>
      <Pre lang="sh">{`git clone https://github.com/Walon-Foundation/code-lang
cd code-lang
cargo build --release`}</Pre>
      <p>
        The interpreter binary lands at <code>target/release/code-lang</code>. Add it to your PATH or run it directly.
      </p>
      <p>
        See the full <Link href="/install">install guide</Link> for more detail, including the upcoming
        formatter (<code>code-lang-fmt</code>) and language server (<code>code-lang-lsp</code>).
      </p>

      <h2 id="the-repl">The REPL</h2>
      <p>
        Run <code>code-lang</code> with no arguments to start the interactive shell. History is saved across sessions.
      </p>
      <Pre lang="text">{`>> let name = "world";
>> "Hello, " + name + "!";
Hello, world!
>> 2 ** 10;
1024
>> typeof 42;
integer
>> exit`}</Pre>
      <p>Exit with <code>exit</code>, <code>exit()</code>, or <kbd>Ctrl-C</kbd>.</p>

      <h2 id="your-first-script">Your first script</h2>
      <p>
        Scripts use the <code>.cl</code> extension. Create <code>hello.cl</code>:
      </p>
      <Pre>{`import "fmt";
import "math";

struct Point {
    x: 0,
    y: 0,
    distance: fn(self) {
        math.sqrt(self.x ** 2 + self.y ** 2)
    },
}

let greet = fn(name, msg = "Hello") {
    fmt.print(msg + ", " + name + "!");
};

let p = Point { x: 3, y: 4 };
greet("world");
fmt.print("distance:", p.distance());`}</Pre>
      <Pre lang="text">{`code-lang hello.cl
Hello, world!
distance: 5`}</Pre>

      <h2 id="importing-modules">Importing modules</h2>
      <p>
        All standard library modules are built in — no installation or setup needed.
        Import any module by name and call its functions with dot notation:
      </p>
      <Pre>{`import "strings";
import "arrays";
import "json";

strings.to_upper("hello");                       # HELLO
arrays.map([1, 2, 3], fn(x) { x * 2 });         # [2, 4, 6]
arrays.filter([1,2,3,4], fn(x) { x % 2 == 0 }); # [2, 4]
json.stringify({ "ok": true });                  # {"ok":true}`}</Pre>
      <p>
        See the <Link href="/docs/stdlib">standard library reference</Link> for all 12 modules.
      </p>

      <h2 id="language-features">Language features at a glance</h2>
      <p>A quick tour of the key language features. Read the <Link href="/docs/language">full reference</Link> for detail.</p>

      <h3>null and ??</h3>
      <Pre>{`let x;              # uninitialized — defaults to null
let y = x ?? 0;     # 0, because x is null
let z = 5 ?? 0;     # 5, because 5 is not null`}</Pre>

      <h3>Destructuring</h3>
      <Pre>{`let [a, b, c] = [1, 2, 3];
let { name, age } = { "name": "Walon", "age": 25 };`}</Pre>

      <h3>Enums and switch</h3>
      <Pre>{`enum Status { Ok, Pending, Err }

let s = Status.Ok;
switch (s) {
    Status.Ok      => fmt.print("all good"),
    Status.Pending => fmt.print("waiting"),
    Status.Err     => fmt.print("failed"),
};`}</Pre>

      <h3>typeof</h3>
      <Pre>{`typeof 42;       # "integer"
typeof "hello";  # "string"
typeof null;     # "null"
typeof [1,2];    # "array"`}</Pre>

      <h2 id="error-format">Error format</h2>
      <p>
        Errors include the source line, a caret pointing to the exact position, and a hint on how to fix it:
      </p>
      <Pre lang="text">{`error: identifier not found: foo
  --> 3:9
   |
 3 | let x = foo + 1;
   |         ^
hint: declare it first with 'let foo = value'`}</Pre>
      <p>Script mode exits with code <code>1</code> on any error.</p>

      <h2 id="next-steps">Next steps</h2>
      <p>
        Read the <Link href="/docs/language">language reference</Link> for a complete guide to syntax,
        types, functions, structs, enums, modules, and error handling.
      </p>
    </>
  );
}
