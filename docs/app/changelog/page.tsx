import type { Metadata } from "next";
import Link from "next/link";

export const metadata: Metadata = { title: "Changelog" };

const CHANGELOG: {
  version: string;
  date: string;
  tag: "release" | "dev";
  sections: { title: string; items: string[] }[];
}[] = [
  {
    version: "v0.2.2",
    date: "June 2026",
    tag: "release",
    sections: [
      {
        title: "Language",
        items: [
          "null keyword — write null as a literal value in any expression",
          "let x; — uninitialized declaration defaults to null (no more required initializer)",
          "?? null coalescing operator — 'a ?? b' returns b only when a is null",
          "typeof keyword — 'typeof expr' returns the type name as a string",
          "Array destructuring in let/const — 'let [a, b] = arr' binds elements by position",
          "Hash destructuring in let/const — 'let { x, y } = hash' extracts keys by name, 'let { x: alias } = hash' renames",
          "Default function parameters — 'fn(name, greeting = \"hi\")' uses the default when the argument is omitted",
        ],
      },
      {
        title: "Bug fixes",
        items: [
          "Struct self-methods with zero extra args no longer panic — 'point.get_x()' works correctly",
          "Array index assignment now bounds-checks — out-of-range index returns an error instead of silently doing nothing",
          "Importing a .cl file that fails at runtime now surfaces the error instead of swallowing it",
        ],
      },
      {
        title: "Safety",
        items: [
          "Float operations (sqrt, log, pow, trig) return a clean error on NaN or Infinity instead of propagating IEEE 754 special values",
          "Number parsing in the lexer no longer panics on malformed literals — emits ILLEGAL token instead",
        ],
      },
    ],
  },
  {
    version: "v0.2.1",
    date: "June 2026",
    tag: "release",
    sections: [
      {
        title: "Language",
        items: [
          "String interpolation — embed expressions directly in strings with ${...} syntax",
          "for-in loops — iterate arrays with 'for (i in arr)' and hashes with 'for (k, v in hash)'",
          "switch statement — pattern matching with 'switch (subject) { pattern => body }', compared with ==",
          "Enum types — define named variant sets with 'enum Direction { North, South, East, West }' and access via 'Direction.North'",
        ],
      },
      {
        title: "Error handling",
        items: [
          "is_error(val) global builtin — test whether a value is an error without importing anything",
          "Errors stored in let/const are recoverable values — only bare expression statements propagate errors",
          "Module member errors now name the module: 'fmt has no member x', 'utils has no public member x'",
        ],
      },
    ],
  },
  {
    version: "v0.2.0",
    date: "June 2026",
    tag: "release",
    sections: [
      {
        title: "Error quality",
        items: [
          "All errors now show the source line with a caret pointing to the exact column",
          "break and continue outside a loop now report the correct line/column",
          "import errors now point to the import statement location",
          "Hint messages added for common mistakes — type mismatches, undefined variables, arity errors, and more",
          "REPL now checks for parse errors before evaluating, preventing confusing partial-AST results",
          "Parse errors in imported .cl files are surfaced with the file name",
        ],
      },
      {
        title: "Safety",
        items: [
          "Recursion depth limit of 500 — infinite recursion now gives a clean error instead of a segfault",
          "Integer arithmetic (+ - * **) now uses checked operations — overflow produces a clear error",
          "Float operations that produce NaN or Infinity now return an error",
          "Function calls enforce arity — wrong argument count is an error, not silent truncation",
        ],
      },
      {
        title: "Standard library",
        items: [
          "arrays.map, filter, reduce, find, any, all — higher-order functions that accept user-defined functions",
          "fmt.format(template, ...args) — printf-style string formatting with %s %d %f %%",
          "math.log2, math.sign, math.gcd, math.lcm",
          "strings.lines, strings.is_empty, strings.pad_left, strings.pad_right",
          "hash.get(h, key, default) — safe key access with a fallback value",
        ],
      },
    ],
  },
  {
    version: "v0.1.0-dev",
    date: "June 2026",
    tag: "dev",
    sections: [
      {
        title: "Language",
        items: [
          "Tree-walking interpreter rewritten from Go to Rust",
          "First-class functions, closures, and recursion",
          "Structs with default field values and dot-notation access",
          "Module and import system for stdlib and .cl files",
          "Control flow: if / elseif / else, while, for, break, continue",
          "Operators: arithmetic (**  //), comparison, logical (&&  ||), compound assignment, prefix/postfix ++/--",
          "Types: Integer, Float, String, Char, Boolean, Array, Hash, Null, Function, Struct, Module",
        ],
      },
      {
        title: "Standard library",
        items: [
          "fmt — print, eprint, input, typeof, to_int, to_float, to_str, clear",
          "math — PI, E, sqrt, abs, pow, floor, ceil, round, log, sin, cos, tan, min, max, clamp",
          "strings — to_upper, to_lower, split, join, contains, replace, trim, reverse, to_chars, from_chars, parse_int, parse_float",
          "arrays — 20 functions including push, pop, slice, sort, zip, flatten, unique",
          "hash — keys, values, entries, has_key, merge, delete, len",
          "fs — read_file, write_file, append_file, read_lines, exists, list_dir, mkdir, copy, rename, remove",
          "path — join, basename, dirname, stem, extension, absolute, is_absolute",
          "os — args, platform, arch, get_env, set_env, get_wd, hostname, exit",
          "time — now, unix, sleep, since, format, year/month/day/hour/minute/second",
          "json — parse, stringify",
          "rand — int, float, choice, shuffle",
          "http — get, post, post_json (blocking, returns status/body/ok)",
        ],
      },
      {
        title: "Errors",
        items: [
          "All errors carry line and column from the call site",
          "Error output shows source line with caret pointer",
          "Parse errors surface to the user before evaluation begins",
          "Non-zero exit code on any error in script mode",
        ],
      },
      {
        title: "REPL",
        items: [
          "Persistent history across sessions (~/.code_lang_history)",
          "exit, exit(), and quit all exit cleanly",
          "Errors print to stderr without crashing the session",
        ],
      },
    ],
  },
];

function TagBadge({ tag }: { tag: "release" | "dev" }) {
  const styles: React.CSSProperties =
    tag === "release"
      ? {
          background: "rgba(34,211,238,0.08)",
          border: "1px solid rgba(34,211,238,0.2)",
          color: "#22d3ee",
        }
      : {
          background: "rgba(251,191,36,0.08)",
          border: "1px solid rgba(251,191,36,0.2)",
          color: "#fbbf24",
        };
  return (
    <span
      style={{
        fontSize: "0.6875rem",
        fontWeight: 600,
        letterSpacing: "0.05em",
        textTransform: "uppercase",
        borderRadius: "4px",
        padding: "0.2rem 0.5rem",
        ...styles,
      }}
    >
      {tag === "dev" ? "In development" : "Release"}
    </span>
  );
}

export default function ChangelogPage() {
  return (
    <div className="pw s-page">
      {/* Header */}
      <div
        style={{
          display: "inline-block",
          fontSize: "0.75rem",
          fontWeight: 500,
          color: "#818cf8",
          background: "rgba(129,140,248,0.08)",
          border: "1px solid rgba(129,140,248,0.18)",
          borderRadius: "100px",
          padding: "0.2rem 0.65rem",
          marginBottom: "1.5rem",
        }}
      >
        Changelog
      </div>
      <h1
        style={{
          fontSize: "clamp(2rem, 5vw, 2.75rem)",
          fontWeight: 700,
          letterSpacing: "-0.04em",
          color: "var(--text)",
          marginBottom: "1rem",
          lineHeight: 1.15,
        }}
      >
        What&apos;s changed
      </h1>
      <p style={{ fontSize: "1.0625rem", color: "var(--muted)", lineHeight: 1.7, maxWidth: "520px", marginBottom: "4rem" }}>
        A record of all notable changes to code-lang. The project follows a rolling development model
        — breaking changes will be noted explicitly once v1.0 stabilizes.
      </p>

      {/* Entries */}
      <div style={{ display: "flex", flexDirection: "column", gap: "4rem" }}>
        {CHANGELOG.map((entry) => (
          <div key={entry.version} className="log-entry">
            {/* Sidebar */}
            <div className="log-sidebar">
              <p
                style={{
                  fontFamily: "var(--font-mono)",
                  fontSize: "0.9375rem",
                  fontWeight: 700,
                  color: "var(--text)",
                  marginBottom: "0.375rem",
                  letterSpacing: "-0.02em",
                }}
              >
                {entry.version}
              </p>
              <p style={{ fontSize: "0.8125rem", color: "#52525b", marginBottom: "0.625rem" }}>{entry.date}</p>
              <TagBadge tag={entry.tag} />
            </div>

            {/* Content */}
            <div style={{ flex: 1, minWidth: 0 }}>
              {entry.sections.map((section) => (
                <div key={section.title} style={{ marginBottom: "2rem" }}>
                  <p
                    style={{
                      fontSize: "0.6875rem",
                      fontWeight: 700,
                      letterSpacing: "0.1em",
                      textTransform: "uppercase",
                      color: "#52525b",
                      marginBottom: "0.75rem",
                    }}
                  >
                    {section.title}
                  </p>
                  <ul style={{ margin: 0, padding: 0, listStyle: "none", display: "flex", flexDirection: "column", gap: "0.5rem" }}>
                    {section.items.map((item) => (
                      <li key={item} style={{ display: "flex", gap: "0.625rem", alignItems: "flex-start" }}>
                        <span style={{ color: "#818cf8", flexShrink: 0, marginTop: "0.1rem", fontSize: "0.75rem" }}>+</span>
                        <span style={{ fontSize: "0.9rem", color: "var(--muted)", lineHeight: 1.6 }}>{item}</span>
                      </li>
                    ))}
                  </ul>
                </div>
              ))}
            </div>
          </div>
        ))}
      </div>

      {/* Footer note */}
      <div
        style={{
          marginTop: "4rem",
          paddingTop: "2rem",
          borderTop: "1px solid var(--border)",
          display: "flex",
          alignItems: "center",
          justifyContent: "space-between",
          flexWrap: "wrap",
          gap: "1rem",
        }}
      >
        <p style={{ fontSize: "0.875rem", color: "#52525b" }}>
          Subscribe to releases on{" "}
          <a href="https://github.com/Walon-Foundation/code-lang/releases" target="_blank" rel="noopener noreferrer" style={{ color: "#818cf8" }}>
            GitHub
          </a>
          .
        </p>
        <Link
          href="/install"
          style={{
            fontSize: "0.875rem",
            fontWeight: 500,
            color: "var(--text)",
            background: "var(--surface)",
            border: "1px solid var(--border)",
            borderRadius: "7px",
            padding: "0.4rem 0.875rem",
            textDecoration: "none",
          }}
        >
          Install guide →
        </Link>
      </div>
    </div>
  );
}
