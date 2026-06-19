import Link from "next/link";
import HomeInstall from "./components/HomeInstall";

const EXAMPLE = `import "fmt";
import "math";

struct Point {
    x: 0,
    y: 0,
    distance: fn(self) {
        math.sqrt(self.x ** 2 + self.y ** 2)
    },
}

enum Status { Ok, Pending, Err }

let p = Point { x: 3, y: 4 };
let status = Status.Ok;
let label = null ?? "no label";

let [a, b] = [p.x, p.y];
fmt.print("x:", a, "y:", b);
fmt.print("dist:", p.distance());
fmt.print("type:", typeof status);`;

const FEATURES = [
  {
    icon: "⟨ ⟩",
    title: "Familiar syntax",
    body: "C-style braces, fn, let/const. Reads like the languages you already know.",
  },
  {
    icon: "λ",
    title: "First-class functions",
    body: "Functions are values. Closures, higher-order, recursion, default parameters — all built in.",
  },
  {
    icon: "◈",
    title: "Structs with self-methods",
    body: "Define types with default fields and methods. Call point.distance() — self is injected automatically.",
  },
  {
    icon: "⊞",
    title: "Enums and switch",
    body: "Named variant sets with dot access. Switch dispatches on any value with ==.",
  },
  {
    icon: "⧉",
    title: "Destructuring",
    body: "Unpack arrays and hashes directly: let [a, b] = arr and let { x, y } = hash.",
  },
  {
    icon: "◻",
    title: "12 stdlib modules",
    body: "math, strings, arrays, fs, http, json, time, rand, os, hash, path, fmt — ready to import.",
  },
  {
    icon: "⌗",
    title: "Null safety",
    body: "null is a first-class value. let x; defaults to null. ?? returns the right side when the left is null.",
  },
  {
    icon: "↯",
    title: "Precise errors",
    body: "Every error shows the source line, a caret, and a hint on how to fix it.",
  },
];

const MODULES = ["fmt", "math", "strings", "arrays", "hash", "fs", "path", "os", "time", "json", "rand", "http"];

const ROADMAP = [
  {
    label: "VS Code extension",
    desc: "Syntax highlighting for .cl files in VS Code, Cursor, and all Electron-based editors.",
    status: "in progress",
  },
  {
    label: "code-lang-lsp",
    desc: "Language server with parse diagnostics — underlines errors in the editor as you type.",
    status: "in progress",
  },
  {
    label: "code-lang-fmt",
    desc: "Formatter with check and lint subcommands. code-lang-fmt check exits 1 on parse errors.",
    status: "in progress",
  },
  {
    label: "Install script",
    desc: "curl | sh that drops code-lang, code-lang-lsp, and code-lang-fmt into ~/.code-lang/bin/.",
    status: "planned",
  },
  {
    label: "Zed extension",
    desc: "Tree-sitter grammar and language server integration for Zed.",
    status: "planned",
  },
  {
    label: "Higher-order stdlib",
    desc: "arrays.map, filter, reduce, find, any, all with user-defined functions.",
    status: "done",
  },
];

export default function Home() {
  return (
    <main>
      {/* ── Hero ─────────────────────────────────────── */}
      <section className="pw s-hero">
        <div
          style={{
            display: "inline-flex",
            alignItems: "center",
            gap: "0.4rem",
            fontSize: "0.75rem",
            fontWeight: 500,
            color: "#818cf8",
            background: "rgba(129,140,248,0.08)",
            border: "1px solid rgba(129,140,248,0.2)",
            borderRadius: "100px",
            padding: "0.25rem 0.75rem",
            marginBottom: "1.75rem",
          }}
        >
          <span style={{ width: 6, height: 6, borderRadius: "50%", background: "#818cf8", display: "inline-block" }} />
          Active development · v0.2.2
        </div>

        <h1
          style={{
            fontSize: "clamp(2.5rem, 6vw, 4rem)",
            fontWeight: 700,
            letterSpacing: "-0.04em",
            lineHeight: 1.1,
            color: "var(--text)",
            maxWidth: "680px",
            marginBottom: "1.5rem",
          }}
        >
          Code that reads{" "}
          <span
            style={{
              background: "linear-gradient(135deg, #818cf8, #22d3ee)",
              WebkitBackgroundClip: "text",
              WebkitTextFillColor: "transparent",
              backgroundClip: "text",
            }}
          >
            like you think.
          </span>
        </h1>

        <p
          style={{
            fontSize: "1.125rem",
            color: "var(--muted)",
            lineHeight: 1.75,
            maxWidth: "520px",
            marginBottom: "2.5rem",
          }}
        >
          code-lang is a general-purpose interpreted language built in Rust — fast to learn,
          with first-class functions, structs, enums, destructuring, and a complete standard library.
        </p>

        <div className="cta-row">
          <Link
            href="/docs"
            style={{
              height: "2.5rem",
              padding: "0 1.25rem",
              background: "linear-gradient(135deg, #6366f1, #06b6d4)",
              color: "#fff",
              borderRadius: "8px",
              fontSize: "0.875rem",
              fontWeight: 600,
              textDecoration: "none",
              display: "inline-flex",
              alignItems: "center",
              letterSpacing: "-0.01em",
            }}
          >
            Get started →
          </Link>
          <a
            href="https://github.com/Walon-Foundation/code-lang"
            target="_blank"
            rel="noopener noreferrer"
            style={{
              height: "2.5rem",
              padding: "0 1.25rem",
              background: "var(--surface)",
              color: "var(--muted)",
              border: "1px solid var(--border)",
              borderRadius: "8px",
              fontSize: "0.875rem",
              fontWeight: 500,
              textDecoration: "none",
              display: "inline-flex",
              alignItems: "center",
              gap: "0.4rem",
            }}
          >
            View on GitHub
          </a>
        </div>
      </section>

      {/* ── Code showcase ────────────────────────────── */}
      <section style={{ borderTop: "1px solid var(--border)", borderBottom: "1px solid var(--border)", background: "var(--surface)" }}>
        <div className="pw s-md">
          <div
            style={{
              borderRadius: "10px",
              border: "1px solid var(--border)",
              overflow: "hidden",
              background: "#0d0d0f",
            }}
          >
            <div
              style={{
                padding: "0.6rem 1rem",
                borderBottom: "1px solid var(--border)",
                display: "flex",
                alignItems: "center",
                gap: "0.4rem",
              }}
            >
              <span style={{ width: 10, height: 10, borderRadius: "50%", background: "#ff5f57" }} />
              <span style={{ width: 10, height: 10, borderRadius: "50%", background: "#ffbd2e" }} />
              <span style={{ width: 10, height: 10, borderRadius: "50%", background: "#28ca42" }} />
              <span style={{ marginLeft: "0.5rem", fontSize: "0.75rem", color: "#52525b" }}>example.cl</span>
            </div>
            <pre
              style={{
                margin: 0,
                padding: "1.5rem",
                fontFamily: "var(--font-mono)",
                fontSize: "0.875rem",
                lineHeight: 1.75,
                color: "#d4d4d8",
                overflowX: "auto",
              }}
            >
              <code>{EXAMPLE}</code>
            </pre>
          </div>
        </div>
      </section>

      {/* ── Features ─────────────────────────────────── */}
      <section className="pw s-lg">
        <p style={{ fontSize: "0.75rem", fontWeight: 600, letterSpacing: "0.1em", textTransform: "uppercase", color: "#52525b", marginBottom: "1rem" }}>
          Why code-lang
        </p>
        <h2
          style={{
            fontSize: "1.875rem",
            fontWeight: 700,
            letterSpacing: "-0.03em",
            color: "var(--text)",
            marginBottom: "3rem",
            maxWidth: "480px",
            lineHeight: 1.2,
          }}
        >
          Everything you need, nothing you don&apos;t.
        </h2>
        <div
          style={{
            display: "grid",
            gridTemplateColumns: "repeat(auto-fill, minmax(280px, 1fr))",
            gap: "1px",
            background: "var(--border)",
            border: "1px solid var(--border)",
            borderRadius: "12px",
            overflow: "hidden",
          }}
        >
          {FEATURES.map((f) => (
            <div
              key={f.title}
              style={{ background: "var(--bg)", padding: "1.75rem" }}
            >
              <div
                style={{
                  fontFamily: "var(--font-mono)",
                  fontSize: "1.125rem",
                  color: "#818cf8",
                  marginBottom: "0.875rem",
                  lineHeight: 1,
                }}
              >
                {f.icon}
              </div>
              <h3 style={{ fontSize: "0.9375rem", fontWeight: 600, color: "var(--text)", marginBottom: "0.4rem" }}>
                {f.title}
              </h3>
              <p style={{ fontSize: "0.875rem", color: "var(--muted)", lineHeight: 1.65, margin: 0 }}>{f.body}</p>
            </div>
          ))}
        </div>
      </section>

      {/* ── Stdlib strip ─────────────────────────────── */}
      <section style={{ borderTop: "1px solid var(--border)", background: "var(--surface)" }}>
        <div className="pw s-md">
          <p style={{ fontSize: "0.875rem", color: "var(--muted)", marginBottom: "1.25rem" }}>
            12 built-in modules — import any of them with a single line:
          </p>
          <div style={{ display: "flex", flexWrap: "wrap", gap: "0.5rem" }}>
            {MODULES.map((m) => (
              <Link
                key={m}
                href="/docs/stdlib"
                style={{
                  fontFamily: "var(--font-mono)",
                  fontSize: "0.8125rem",
                  color: "#22d3ee",
                  background: "rgba(34,211,238,0.06)",
                  border: "1px solid rgba(34,211,238,0.15)",
                  borderRadius: "6px",
                  padding: "0.25rem 0.625rem",
                  textDecoration: "none",
                }}
              >
                {`import "${m}"`}
              </Link>
            ))}
          </div>
        </div>
      </section>

      {/* ── Roadmap ──────────────────────────────────── */}
      <section className="pw s-lg">
        <p style={{ fontSize: "0.75rem", fontWeight: 600, letterSpacing: "0.1em", textTransform: "uppercase", color: "#52525b", marginBottom: "1rem" }}>
          Roadmap
        </p>
        <h2
          style={{
            fontSize: "1.875rem",
            fontWeight: 700,
            letterSpacing: "-0.03em",
            color: "var(--text)",
            marginBottom: "0.75rem",
            lineHeight: 1.2,
          }}
        >
          What&apos;s coming next.
        </h2>
        <p style={{ fontSize: "0.9375rem", color: "var(--muted)", marginBottom: "2.5rem", maxWidth: "480px", lineHeight: 1.7 }}>
          The interpreter is stable. The next milestone is the toolchain — editor support, a formatter, and an install script.
        </p>
        <div style={{ display: "flex", flexDirection: "column", gap: "0" }}>
          {ROADMAP.map((item, i) => {
            const statusColor =
              item.status === "done"
                ? { color: "#86efac", bg: "rgba(134,239,172,0.08)", border: "rgba(134,239,172,0.2)" }
                : item.status === "in progress"
                ? { color: "#818cf8", bg: "rgba(129,140,248,0.08)", border: "rgba(129,140,248,0.2)" }
                : { color: "#52525b", bg: "rgba(82,82,91,0.08)", border: "rgba(82,82,91,0.2)" };
            return (
              <div
                key={item.label}
                style={{
                  display: "flex",
                  alignItems: "flex-start",
                  gap: "1.25rem",
                  padding: "1.25rem 0",
                  borderBottom: i < ROADMAP.length - 1 ? "1px solid var(--border)" : "none",
                }}
              >
                <span
                  style={{
                    flexShrink: 0,
                    marginTop: "0.15rem",
                    fontSize: "0.65rem",
                    fontWeight: 600,
                    letterSpacing: "0.06em",
                    textTransform: "uppercase",
                    color: statusColor.color,
                    background: statusColor.bg,
                    border: `1px solid ${statusColor.border}`,
                    borderRadius: "4px",
                    padding: "0.2rem 0.45rem",
                    minWidth: "80px",
                    textAlign: "center" as const,
                  }}
                >
                  {item.status}
                </span>
                <div>
                  <p style={{ fontWeight: 600, color: "var(--text)", fontSize: "0.9rem", marginBottom: "0.2rem" }}>{item.label}</p>
                  <p style={{ fontSize: "0.85rem", color: "var(--muted)", margin: 0, lineHeight: 1.6 }}>{item.desc}</p>
                </div>
              </div>
            );
          })}
        </div>
      </section>

      {/* ── Install ──────────────────────────────────── */}
      <section className="pw s-lg" style={{ borderTop: "1px solid var(--border)" }}>
        <h2 style={{ fontSize: "1.5rem", fontWeight: 700, letterSpacing: "-0.03em", color: "var(--text)", marginBottom: "0.625rem" }}>
          Install
        </h2>
        <p style={{ fontSize: "0.9375rem", color: "var(--muted)", marginBottom: "2rem" }}>
          Build from source with Cargo. Pre-built binaries and an install script are coming.
        </p>
        <div style={{ maxWidth: 560 }}>
          <HomeInstall />
        </div>
      </section>
    </main>
  );
}
