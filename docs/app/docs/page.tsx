import type { Metadata } from "next";

export const metadata: Metadata = { title: "Getting started" };

export default function GettingStarted() {
  return (
    <>
      <h1>Getting started</h1>

      <h2>Prerequisites</h2>
      <p>
        You need <a href="https://rustup.rs" target="_blank" rel="noopener noreferrer">Rust</a> (stable) installed.
      </p>

      <h2>Build from source</h2>
      <pre><code>{`git clone https://github.com/Walon-Foundation/code-lang
cd code-lang
cargo build --release`}</code></pre>
      <p>The binary lands at <code>target/release/code-lang</code>.</p>

      <h2>Run the REPL</h2>
      <pre><code>./target/release/code-lang</code></pre>
      <p>
        The REPL keeps history across sessions in <code>~/.code_lang_history</code>.
        Type <code>exit</code> or press <kbd>Ctrl-C</kbd> to quit.
      </p>
      <pre><code>{`>> let x = 10;
>> x * 2;
20
>> exit`}</code></pre>

      <h2>Run a script</h2>
      <p>Scripts must use the <code>.cl</code> extension.</p>
      <pre><code>./target/release/code-lang hello.cl</code></pre>

      <h2>Hello, world</h2>
      <p>Create <code>hello.cl</code>:</p>
      <pre><code>{`import "fmt";

fmt.print("Hello, world!");`}</code></pre>
      <pre><code>./target/release/code-lang hello.cl
Hello, world!</code></pre>

      <h2>Errors</h2>
      <p>
        Errors show the line, column, and the offending source line with a caret so you know exactly where the problem is:
      </p>
      <pre><code>{`error: identifier not found: foo
 --> 3:9
  |
3 | let x = foo + 1;
  |         ^`}</code></pre>
    </>
  );
}
