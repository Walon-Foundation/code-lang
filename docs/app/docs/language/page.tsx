import type { Metadata } from "next";
import Link from "next/link";
import Pre from "../../components/Pre";

export const metadata: Metadata = { title: "Language reference" };

function Note({ children }: { children: React.ReactNode }) {
  return (
    <div
      style={{
        background: "rgba(129,140,248,0.06)",
        border: "1px solid rgba(129,140,248,0.18)",
        borderRadius: "8px",
        padding: "0.875rem 1rem",
        marginBottom: "1.25rem",
        fontSize: "0.875rem",
        color: "#a1a1aa",
        lineHeight: 1.65,
      }}
    >
      {children}
    </div>
  );
}

export default function LanguageReference() {
  return (
    <>
      <h1>Language reference</h1>
      <p>
        A complete guide to code-lang syntax and semantics. All examples can be run in the REPL or saved
        to a <code>.cl</code> file.
      </p>

      <h2 id="comments">Comments</h2>
      <Pre>{`# single-line comment

/* multi-line
   comment */`}</Pre>

      <h2 id="variables">Variables</h2>
      <p>
        <code>let</code> declares a mutable variable. <code>const</code> declares a constant —
        reassignment is a runtime error.
      </p>
      <Pre>{`let age = 25;
const PI = 3.14159;

age = 26;      # ok
PI  = 3;       # error: cannot reassign constant`}</Pre>

      <h2 id="types">Types</h2>
      <div className="tbl"><table>
        <thead><tr><th>Type</th><th>Literal example</th><th>Notes</th></tr></thead>
        <tbody>
          <tr><td>Integer</td><td><code>42</code>, <code>-7</code></td><td>64-bit signed</td></tr>
          <tr><td>Float</td><td><code>3.14</code>, <code>-0.5</code></td><td>64-bit IEEE 754</td></tr>
          <tr><td>String</td><td><code>"hello"</code>, <code>"hi ${name}"</code></td><td>UTF-8, double-quoted; supports interpolation</td></tr>
          <tr><td>Char</td><td><code>'a'</code></td><td>Single character, single-quoted</td></tr>
          <tr><td>Boolean</td><td><code>true</code>, <code>false</code></td><td></td></tr>
          <tr><td>Null</td><td><code>null</code></td><td>Absence of value</td></tr>
          <tr><td>Array</td><td><code>[1, "x", true]</code></td><td>Mixed types allowed</td></tr>
          <tr><td>Hash</td><td><code>{"{ \"k\": 1 }"}</code></td><td>Any type as key or value</td></tr>
          <tr><td>Function</td><td><code>fn(x) {"{ x * 2 }"}</code></td><td>First-class value</td></tr>
          <tr><td>Struct</td><td><code>Point {"{ x: 1 }"}</code></td><td>Typed object with defaults</td></tr>
        </tbody>
      </table></div>

      <h2 id="operators">Operators</h2>
      <div className="tbl"><table>
        <thead><tr><th>Category</th><th>Operators</th></tr></thead>
        <tbody>
          <tr><td>Arithmetic</td><td><code>+ - * / %</code> · <code>**</code> power · <code>//</code> floor division</td></tr>
          <tr><td>Comparison</td><td><code>== != &lt; &gt; &lt;= &gt;=</code></td></tr>
          <tr><td>Logical</td><td><code>&amp;&amp;</code> <code>||</code> <code>!</code> — short-circuit evaluation</td></tr>
          <tr><td>Compound assign</td><td><code>+= -= *= /= %=</code></td></tr>
          <tr><td>Increment / decrement</td><td><code>++</code> <code>--</code> prefix and postfix</td></tr>
          <tr><td>String concat</td><td><code>+</code> — works between strings</td></tr>
        </tbody>
      </table></div>
      <Pre>{`2 ** 8;       # 256
17 // 5;      # 3  (floor division)
10 % 3;       # 1

let n = 5;
n++;          # n is now 6
n += 10;      # n is now 16`}</Pre>

      <h2 id="string-interpolation">String interpolation</h2>
      <p>
        Embed any expression inside a string with <code>{"${...}"}</code>. The expression is evaluated
        and converted to a string automatically.
      </p>
      <Pre>{`let name = "Walon";
let age  = 25;

"Hello, \${name}!";          # Hello, Walon!
"In 5 years you'll be \${age + 5}.";   # In 5 years you'll be 30.
"pi ≈ \${math.round(3.14159, 2)}";`}</Pre>

      <h2 id="control-flow">Control flow</h2>

      <h3>if / elseif / else</h3>
      <Note>Branches are expressions — the last evaluated value is the result of the whole block.</Note>
      <Pre>{`let score = 85;

if (score >= 90) {
    "A"
} elseif (score >= 80) {
    "B"
} else {
    "C"
};`}</Pre>

      <h3>while</h3>
      <Pre>{`let i = 0;
while (i < 5) {
    i += 1;
};`}</Pre>

      <h3>for</h3>
      <Pre>{`for (let i = 0; i < 5; i++) {
    if (i == 2) { continue; };
    if (i == 4) { break; };
};`}</Pre>
      <p><code>break</code> and <code>continue</code> work in both <code>while</code> and <code>for</code>.</p>

      <h3>for-in</h3>
      <p>Iterate over arrays or hashes without a counter.</p>
      <Pre>{`let nums = [10, 20, 30];

for (n in nums) {
    fmt.print(n);
};

let scores = { "Alice": 95, "Bob": 87 };

for (name, score in scores) {
    fmt.print("\${name}: \${score}");
};`}</Pre>

      <h3>switch</h3>
      <p>
        Compare a subject against a series of patterns using <code>==</code>. The first matching arm runs.
        If no arm matches, the result is <code>null</code>.
      </p>
      <Pre>{`let direction = Direction.North;

switch (direction) {
    Direction.North => fmt.print("going north"),
    Direction.South => fmt.print("going south"),
    Direction.East  => fmt.print("going east"),
    Direction.West  => fmt.print("going west"),
};

# switch on any value
switch (score // 10) {
    10 => "A+",
    9  => "A",
    8  => "B",
};`}</Pre>

      <h2 id="functions">Functions</h2>
      <p>Functions are values. Assign them with <code>let</code> or <code>const</code>. Return early with <code>return</code> — the last expression in a block is also returned implicitly.</p>
      <Pre>{`let add = fn(a, b) {
    return a + b;
};

let square = fn(x) { x * x };   # implicit return

add(3, 4);      # 7
square(9);      # 81`}</Pre>

      <h3>Closures</h3>
      <p>Functions close over the enclosing scope and capture variables by reference.</p>
      <Pre>{`let make_adder = fn(n) {
    return fn(x) { x + n };
};

let add5 = make_adder(5);
add5(10);   # 15
add5(20);   # 25`}</Pre>

      <h3>Recursion</h3>
      <Pre>{`let fib = fn(n) {
    if (n <= 1) { return n; };
    return fib(n - 1) + fib(n - 2);
};

fib(10);   # 55`}</Pre>

      <h3>Higher-order functions</h3>
      <Pre>{`let apply = fn(f, x) { f(x) };
apply(fn(n) { n * 2 }, 7);   # 14

import "arrays";
let doubled = arrays.map([1, 2, 3], fn(x) { x * 2 });   # [2, 4, 6]
let evens   = arrays.filter([1,2,3,4], fn(x) { x % 2 == 0 });
let sum     = arrays.reduce([1,2,3,4], fn(acc, x) { acc + x }, 0);`}</Pre>

      <h2 id="arrays">Arrays</h2>
      <Pre>{`let nums = [1, 2, 3, 4, 5];

nums[0];          # 1
nums[2] = 99;     # mutate in place
nums[-1];         # last element (if supported)

let mixed = [1, "hello", true, [2, 3]];`}</Pre>
      <p>See the <Link href="/docs/stdlib">arrays module</Link> for slice, sort, zip, flatten, and 15 more operations.</p>

      <h2 id="hashes">Hashes</h2>
      <Pre>{`let person = { "name": "Alice", "age": 30 };

person["name"];        # Alice
person.name;           # same — dot access works too
person["role"] = "admin";   # add or update key`}</Pre>
      <p>Keys can be any type. See the <Link href="/docs/stdlib">hash module</Link> for <code>keys</code>, <code>values</code>, <code>merge</code>, and more.</p>

      <h2 id="structs">Structs</h2>
      <p>
        Structs define a named type with default field values. Instantiate with <code>TypeName {"{ fields }"}</code> —
        any field not provided gets its default.
      </p>
      <Pre>{`struct User {
    name: "Guest",
    role: "user",
    active: true,
}

let admin = User { name: "Walon", role: "admin" };
let guest = User {};

admin.name;    # Walon
guest.name;    # Guest
guest.active;  # true`}</Pre>

      <h2 id="enums">Enums</h2>
      <p>
        Enums define a named set of variants. Access variants with dot notation. Variants compare equal
        only to themselves, making them safe to use in <code>switch</code> arms.
      </p>
      <Pre>{`enum Direction { North, South, East, West }
enum Status    { Ok, Err, Pending }

let d = Direction.North;

d == Direction.North;   # true
d == Direction.South;   # false

switch (d) {
    Direction.North => "up",
    Direction.South => "down",
};`}</Pre>

      <h2 id="error-handling">Error handling</h2>
      <p>
        Errors in code-lang are values. When a stdlib function fails it returns an error object —
        it does <strong>not</strong> crash the program. Use <code>is_error(val)</code> to test it.
      </p>
      <Pre>{`import "fs";

let content = fs.read_file("maybe.txt");

if (is_error(content)) {
    fmt.print("file not found");
} else {
    fmt.print(content);
};`}</Pre>
      <Note>
        <code>is_error()</code> is a global builtin — no import needed. Errors stored in <code>let</code> or{" "}
        <code>const</code> are recoverable values. A bare error expression (not assigned) propagates and
        halts the current block.
      </Note>

      <h2 id="modules">Modules</h2>

      <h3>Import stdlib</h3>
      <Pre>{`import "math";
import "strings";

math.sqrt(16);              # 4.0
math.clamp(150, 0, 100);    # 100

strings.split("a,b,c", ",");   # ["a", "b", "c"]
strings.to_upper("hello");     # HELLO`}</Pre>

      <h3>Import a .cl file</h3>
      <p>
        Import any <code>.cl</code> file by its path (without the extension). Everything declared at the
        top level in that file becomes a field on the resulting module object.
      </p>
      <Pre>{`# utils.cl
const VERSION = "1.0";
let double = fn(x) { x * 2 };

# main.cl
import "utils";
utils.double(5);    # 10
utils.VERSION;      # 1.0`}</Pre>
      <p>
        Use <code>pub</code> to control what is exported. When any <code>pub</code> declaration exists,
        only those names are accessible from outside the module.
      </p>
      <Pre>{`# utils.cl
pub let greet = fn(name) { "Hello, \${name}!" };
let _secret   = 42;   # not exported

# main.cl
import "utils";
utils.greet("world");   # Hello, world!
utils._secret;          # error: utils has no public member '_secret'`}</Pre>

      <p>See all 12 built-in modules in the <Link href="/docs/stdlib">standard library reference</Link>.</p>
    </>
  );
}
