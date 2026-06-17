import Link from "next/link";

const EXAMPLE = `import "fmt";
import "math";

let greet = fn(name) {
    fmt.print("Hello, " + name + "!");
};

struct Point {
    x: 0,
    y: 0,
}

let p = Point { x: 3, y: 4 };
let dist = math.sqrt(p.x ** 2 + p.y ** 2);

greet("world");
fmt.print("distance:", dist);`;

const FEATURES = [
  { title: "Familiar syntax", body: "C-style braces, fn for functions, let/const for variables. Readable from day one." },
  { title: "First-class functions", body: "Functions are values. Pass them around, return them, close over variables." },
  { title: "Structs", body: "Define custom types with default field values and dot-notation access." },
  { title: "Rich standard library", body: "math, strings, arrays, hash, fs, os, time, json, http, rand, path — all built in." },
  { title: "Module system", body: "Import stdlib modules or your own .cl files. Simple, flat, no namespacing headaches." },
  { title: "Helpful errors", body: "Line, column, and source context on every error — so you know exactly where to look." },
];

export default function Home() {
  return (
    <main className="flex-1">
      {/* Hero */}
      <section className="max-w-5xl mx-auto px-6 py-24">
        <div className="max-w-2xl">
          <div className="inline-block text-xs font-medium px-2.5 py-1 rounded-full bg-zinc-100 dark:bg-zinc-800 text-zinc-600 dark:text-zinc-400 mb-6">
            Active development · v0.1
          </div>
          <h1 className="text-5xl font-semibold tracking-tight leading-tight mb-6">
            A clean, expressive<br />language in Rust.
          </h1>
          <p className="text-lg text-zinc-600 dark:text-zinc-400 mb-10 leading-relaxed">
            code-lang is a general-purpose interpreted language with first-class functions,
            structs, a growing standard library, and errors that actually point you to the problem.
          </p>
          <div className="flex gap-3">
            <Link
              href="/docs"
              className="h-10 px-5 bg-zinc-900 dark:bg-zinc-50 text-white dark:text-zinc-900 rounded-lg text-sm font-medium flex items-center hover:bg-zinc-700 dark:hover:bg-zinc-200 transition-colors"
            >
              Get started
            </Link>
            <Link
              href="/docs/language"
              className="h-10 px-5 border border-zinc-200 dark:border-zinc-700 rounded-lg text-sm font-medium flex items-center hover:bg-zinc-50 dark:hover:bg-zinc-900 transition-colors"
            >
              Language reference
            </Link>
          </div>
        </div>
      </section>

      {/* Code example */}
      <section className="bg-zinc-950 border-y border-zinc-800">
        <div className="max-w-5xl mx-auto px-6 py-12">
          <pre className="font-mono text-sm text-zinc-300 leading-relaxed overflow-x-auto">
            <code>{EXAMPLE}</code>
          </pre>
        </div>
      </section>

      {/* Features */}
      <section className="max-w-5xl mx-auto px-6 py-20">
        <h2 className="text-2xl font-semibold tracking-tight mb-12">Why code-lang?</h2>
        <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-8">
          {FEATURES.map((f) => (
            <div key={f.title}>
              <h3 className="font-medium mb-2">{f.title}</h3>
              <p className="text-sm text-zinc-600 dark:text-zinc-400 leading-relaxed">{f.body}</p>
            </div>
          ))}
        </div>
      </section>

      {/* Quick install */}
      <section className="bg-zinc-50 dark:bg-zinc-900 border-t border-zinc-200 dark:border-zinc-800">
        <div className="max-w-5xl mx-auto px-6 py-16">
          <h2 className="text-2xl font-semibold tracking-tight mb-4">Install</h2>
          <p className="text-zinc-600 dark:text-zinc-400 mb-6">Build from source with Cargo:</p>
          <pre className="font-mono text-sm bg-zinc-950 text-zinc-200 rounded-lg px-5 py-4 inline-block">
            <code>{"git clone https://github.com/Walon-Foundation/code-lang\ncd code-lang && cargo build --release"}</code>
          </pre>
          <p className="mt-6 text-sm text-zinc-600 dark:text-zinc-400">
            Then run the REPL with <code className="font-mono">./target/release/code-lang</code> or a script with{" "}
            <code className="font-mono">./target/release/code-lang hello.cl</code>.
          </p>
        </div>
      </section>
    </main>
  );
}
