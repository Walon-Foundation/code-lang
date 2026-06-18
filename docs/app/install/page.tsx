import type { Metadata } from "next";
import Link from "next/link";
import OsInstall from "../components/OsInstall";

export const metadata: Metadata = { title: "Install" };

export default function InstallPage() {
  return (
    <div className="pw s-page">
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
        Installation guide
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
        Install code-lang
      </h1>

      <p
        style={{
          fontSize: "1.0625rem",
          color: "var(--muted)",
          lineHeight: 1.7,
          maxWidth: "520px",
          marginBottom: "3rem",
        }}
      >
        Pre-built binaries for Linux, macOS, and Windows. No Rust required.
      </p>

      {/* OS-aware install section */}
      <OsInstall />

      {/* Usage */}
      <p
        style={{
          fontSize: "0.6875rem",
          fontWeight: 700,
          letterSpacing: "0.1em",
          textTransform: "uppercase",
          color: "#52525b",
          marginBottom: "1rem",
          marginTop: "3.5rem",
        }}
      >
        Usage
      </p>

      <p style={{ fontSize: "1.125rem", fontWeight: 700, letterSpacing: "-0.025em", color: "var(--text)", marginBottom: "0.75rem" }}>
        Run the REPL
      </p>
      <div style={{ background: "var(--surface)", border: "1px solid var(--border)", borderRadius: 12, overflow: "hidden", marginBottom: "1.5rem" }}>
        <pre style={{ margin: 0, padding: "1.25rem", fontFamily: "var(--font-mono)", fontSize: "0.875rem", color: "#d4d4d8", lineHeight: 1.75, overflowX: "auto", background: "transparent" }}>
          <code>{`code-lang\n>> let x = 10;\n>> x * x;\n100\n>> exit`}</code>
        </pre>
      </div>

      <p style={{ fontSize: "1.125rem", fontWeight: 700, letterSpacing: "-0.025em", color: "var(--text)", marginBottom: "0.75rem" }}>
        Run a script
      </p>
      <div style={{ background: "var(--surface)", border: "1px solid var(--border)", borderRadius: 12, overflow: "hidden", marginBottom: "0.75rem" }}>
        <pre style={{ margin: 0, padding: "1.25rem", fontFamily: "var(--font-mono)", fontSize: "0.875rem", color: "#d4d4d8", lineHeight: 1.75, overflowX: "auto", background: "transparent" }}>
          <code>code-lang hello.cl</code>
        </pre>
      </div>
      <p style={{ fontSize: "0.875rem", color: "#52525b", lineHeight: 1.65 }}>
        Scripts use the{" "}
        <code style={{ fontFamily: "var(--font-mono)", fontSize: "0.85em", color: "#86efac" }}>.cl</code>{" "}
        extension.
      </p>

      {/* Next steps */}
      <p
        style={{
          fontSize: "0.6875rem",
          fontWeight: 700,
          letterSpacing: "0.1em",
          textTransform: "uppercase",
          color: "#52525b",
          marginBottom: "1rem",
          marginTop: "3.5rem",
        }}
      >
        Next steps
      </p>
      <div style={{ display: "flex", gap: "1rem", flexWrap: "wrap" }}>
        {[
          { label: "Getting started", href: "/docs", desc: "Write your first script." },
          { label: "Language reference", href: "/docs/language", desc: "Full syntax guide." },
          { label: "Standard library", href: "/docs/stdlib", desc: "All 12 built-in modules." },
        ].map((item) => (
          <Link
            key={item.href}
            href={item.href}
            style={{
              flex: "1 1 200px",
              background: "var(--surface)",
              border: "1px solid var(--border)",
              borderRadius: 10,
              padding: "1.125rem 1.25rem",
              textDecoration: "none",
              display: "block",
            }}
          >
            <p style={{ fontWeight: 600, color: "var(--text)", fontSize: "0.9rem", marginBottom: "0.25rem" }}>{item.label} →</p>
            <p style={{ fontSize: "0.8125rem", color: "var(--muted)", margin: 0 }}>{item.desc}</p>
          </Link>
        ))}
      </div>
    </div>
  );
}
