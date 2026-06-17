import type { Metadata } from "next";

export const metadata: Metadata = { title: "Language reference" };

export default function LanguageReference() {
  return (
    <>
      <h1>Language reference</h1>

      <h2>Comments</h2>
      <pre><code>{`# single-line comment

/* multi-line
   comment */`}</code></pre>

      <h2>Variables</h2>
      <p><code>let</code> declares a mutable variable. <code>const</code> declares a constant — reassignment is an error.</p>
      <pre><code>{`let age = 25;
const MAX = 100;

age = 26;     # ok
MAX = 200;    # error: cannot reassign constant`}</code></pre>

      <h2>Types</h2>
      <table>
        <thead><tr><th>Type</th><th>Example</th></tr></thead>
        <tbody>
          <tr><td>Integer</td><td><code>42</code>, <code>-7</code></td></tr>
          <tr><td>Float</td><td><code>3.14</code>, <code>-0.5</code></td></tr>
          <tr><td>String</td><td><code>"hello"</code></td></tr>
          <tr><td>Char</td><td><code>'a'</code></td></tr>
          <tr><td>Boolean</td><td><code>true</code>, <code>false</code></td></tr>
          <tr><td>Null</td><td><code>null</code></td></tr>
          <tr><td>Array</td><td><code>[1, 2, 3]</code></td></tr>
          <tr><td>Hash</td><td><code>{"{ \"key\": \"value\" }"}</code></td></tr>
          <tr><td>Function</td><td><code>fn(x) {"{ x * 2 }"}</code></td></tr>
          <tr><td>Struct instance</td><td><code>Point {"{ x: 1, y: 2 }"}</code></td></tr>
        </tbody>
      </table>

      <h2>Operators</h2>
      <table>
        <thead><tr><th>Category</th><th>Operators</th></tr></thead>
        <tbody>
          <tr><td>Arithmetic</td><td><code>+ - * / %</code> and <code>**</code> (power), <code>//</code> (floor div)</td></tr>
          <tr><td>Comparison</td><td><code>== != &lt; &gt; &lt;= &gt;=</code></td></tr>
          <tr><td>Logical</td><td><code>&amp;&amp;</code> <code>||</code> <code>!</code> (short-circuit)</td></tr>
          <tr><td>Compound assign</td><td><code>+= -= *= /= %=</code></td></tr>
          <tr><td>Increment</td><td><code>++</code> <code>--</code> (prefix and postfix)</td></tr>
        </tbody>
      </table>

      <h2>Control flow</h2>

      <h3>if / elseif / else</h3>
      <pre><code>{`if (x > 10) {
    "greater"
} elseif (x == 10) {
    "equal"
} else {
    "less"
};`}</code></pre>

      <h3>while</h3>
      <pre><code>{`let i = 0;
while (i < 5) {
    i += 1;
};`}</code></pre>

      <h3>for</h3>
      <pre><code>{`for (let i = 0; i < 5; i++) {
    if (i == 3) { break; };
};`}</code></pre>

      <p><code>break</code> and <code>continue</code> work inside both <code>while</code> and <code>for</code>.</p>

      <h2>Functions</h2>
      <p>Functions are values. Assign them with <code>let</code> or <code>const</code>.</p>
      <pre><code>{`let add = fn(a, b) {
    return a + b;
};

add(3, 4);   # 7`}</code></pre>

      <h3>Closures</h3>
      <p>Functions capture the enclosing scope.</p>
      <pre><code>{`let make_counter = fn() {
    let n = 0;
    return fn() {
        n += 1;
        return n;
    };
};

let counter = make_counter();
counter();   # 1
counter();   # 2`}</code></pre>

      <h3>Recursion</h3>
      <pre><code>{`let fib = fn(n) {
    if (n <= 1) { return n; };
    return fib(n - 1) + fib(n - 2);
};

fib(10);   # 55`}</code></pre>

      <h2>Arrays</h2>
      <pre><code>{`let nums = [1, 2, 3, 4, 5];
nums[0];          # 1
nums[2] = 99;     # mutate in place

# arrays can hold mixed types
let mixed = [1, "hello", true, [2, 3]];`}</code></pre>

      <h2>Hashes</h2>
      <pre><code>{`let person = { "name": "Alice", "age": 30 };
person["name"];   # Alice
person.name;      # Alice — dot access also works
person["role"] = "admin";`}</code></pre>

      <h2>Structs</h2>
      <p>Structs define a type with default field values. Instances can override any field.</p>
      <pre><code>{`struct User {
    name: "Guest",
    role: "user",
    active: true,
}

let u = User { name: "Walon", role: "admin" };
let guest = User {};

u.name;        # Walon
guest.name;    # Guest
guest.active;  # true`}</code></pre>

      <h2>Modules</h2>

      <h3>Import stdlib</h3>
      <pre><code>{`import "math";
import "strings";

math.sqrt(16);              # 4.0
strings.to_upper("hello");  # HELLO`}</code></pre>

      <h3>Import a file</h3>
      <p>A <code>.cl</code> file is imported by its path without the extension. Everything defined at the top level becomes a member of the resulting module object.</p>
      <pre><code>{`# utils.cl
let double = fn(x) { x * 2 };

# main.cl
import "utils";
utils.double(5);   # 10`}</code></pre>
    </>
  );
}
