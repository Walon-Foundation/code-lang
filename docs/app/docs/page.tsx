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

      <h2>Install</h2>
      <p>
        See the full <Link href="/install">install guide</Link> for step-by-step instructions.
        The short version — build from source with Cargo:
      </p>
      <Pre lang="sh">{`git clone https://github.com/Walon-Foundation/code-lang
cd code-lang
cargo build --release`}</Pre>
      <p>Binary lands at <code>target/release/code-lang</code>.</p>

      <h2>The REPL</h2>
      <p>
        Run <code>code-lang</code> (or <code>./target/release/code-lang</code>) with no arguments to start
        the interactive shell. History is saved across sessions.
      </p>
      <Pre lang="text">{`>> let name = "world";
>> "Hello, " + name + "!";
Hello, world!
>> 2 ** 10;
1024
>> exit`}</Pre>
      <p>Exit with <code>exit</code>, <code>exit()</code>, or <kbd>Ctrl-C</kbd>.</p>

      <h2>Your first script</h2>
      <p>
        Scripts use the <code>.cl</code> extension. Create <code>hello.cl</code>:
      </p>
      <Pre>{`import "fmt";
import "math";

let greet = fn(name) {
    fmt.print("Hello, " + name + "!");
};

greet("world");
fmt.print("pi ≈", math.PI);`}</Pre>
      <p>Run it:</p>
      <Pre lang="text">{`code-lang hello.cl
Hello, world!
pi ≈ 3.141592653589793`}</Pre>

      <h2>Importing modules</h2>
      <p>
        All standard library modules are built in — no installation or setup needed.
        Import any module by name and call its functions with dot notation:
      </p>
      <Pre>{`import "strings";
import "arrays";
import "json";

strings.to_upper("hello");        # HELLO
arrays.sort([3, 1, 4, 1, 5]);    # [1, 1, 3, 4, 5]
json.stringify({"ok": true});     # {"ok":true}`}</Pre>
      <p>
        See the <Link href="/docs/stdlib">standard library reference</Link> for all 12 modules.
      </p>

      <h2>Error format</h2>
      <p>
        Errors include the source line and a caret pointing to the exact position:
      </p>
      <Pre lang="text">{`error: identifier not found: foo
 --> 3:9
  |
3 | let x = foo + 1;
  |         ^`}</Pre>
      <p>Script mode exits with code <code>1</code> on any error so you can detect failures in shell scripts.</p>

      <h2>Next steps</h2>
      <p>
        Read the <Link href="/docs/language">language reference</Link> for a complete guide to syntax,
        types, functions, structs, and modules.
      </p>
    </>
  );
}
