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
        Currently built from source with Cargo. Pre-built binaries and a one-line install script
        are coming soon.
      </p>

      {/* OS-aware install section */}
      <OsInstall />

      {/* Build from source */}
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
        Build from source
      </p>
      <p style={{ fontSize: "0.9375rem", color: "var(--muted)", marginBottom: "1rem", lineHeight: 1.7 }}>
        Requires <a href="https://rustup.rs" target="_blank" rel="noopener noreferrer" style={{ color: "#818cf8" }}>Rust</a> (stable, 1.80+).
        The repo is a Cargo workspace — all three binaries build together.
      </p>
      <div style={{ background: "var(--surface)", border: "1px solid var(--border)", borderRadius: 12, overflow: "hidden", marginBottom: "1rem" }}>
        <pre style={{ margin: 0, padding: "1.25rem", fontFamily: "var(--font-mono)", fontSize: "0.875rem", color: "#d4d4d8", lineHeight: 1.75, overflowX: "auto", background: "transparent" }}>
          <code>{`git clone https://github.com/Walon-Foundation/code-lang
cd code-lang
cargo build --release`}</code>
        </pre>
      </div>
      <p style={{ fontSize: "0.875rem", color: "#52525b", lineHeight: 1.65, marginBottom: "2rem" }}>
        Binaries land in <code style={{ fontFamily: "var(--font-mono)", fontSize: "0.85em", color: "#86efac" }}>target/release/</code>.
        Add that directory to your PATH or copy the binaries to <code style={{ fontFamily: "var(--font-mono)", fontSize: "0.85em", color: "#86efac" }}>/usr/local/bin/</code>.
      </p>

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
      <p style={{ fontSize: "0.875rem", color: "#52525b", lineHeight: 1.65, marginBottom: "2.5rem" }}>
        Scripts use the{" "}
        <code style={{ fontFamily: "var(--font-mono)", fontSize: "0.85em", color: "#86efac" }}>.cl</code>{" "}
        extension.
      </p>

      {/* Tooling — coming soon */}
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
        Tooling — coming soon
      </p>
      <p style={{ fontSize: "0.9375rem", color: "var(--muted)", marginBottom: "1.5rem", lineHeight: 1.7, maxWidth: "520px" }}>
        The workspace builds three binaries. The formatter and language server are currently skeleton
        stubs — they will be wired up in upcoming releases.
      </p>
      <div style={{ display: "flex", flexDirection: "column", gap: "0" }}>
        {[
          {
            bin: "code-lang",
            desc: "The interpreter. Run scripts or start the REPL.",
            status: "stable",
          },
          {
            bin: "code-lang-fmt",
            desc: "Formatter with check and lint subcommands. code-lang-fmt check file.cl exits 1 on parse errors.",
            status: "in progress",
          },
          {
            bin: "code-lang-lsp",
            desc: "Language server — parse diagnostics, completions, and hover in your editor.",
            status: "in progress",
          },
        ].map((item, i, arr) => {
          const isStable = item.status === "stable";
          const statusColor = isStable
            ? { color: "#86efac", bg: "rgba(134,239,172,0.08)", border: "rgba(134,239,172,0.2)" }
            : { color: "#818cf8", bg: "rgba(129,140,248,0.08)", border: "rgba(129,140,248,0.2)" };
          return (
            <div
              key={item.bin}
              style={{
                display: "flex",
                alignItems: "flex-start",
                gap: "1.25rem",
                padding: "1.125rem 0",
                borderBottom: i < arr.length - 1 ? "1px solid var(--border)" : "none",
              }}
            >
              <code
                style={{
                  fontFamily: "var(--font-mono)",
                  fontSize: "0.8125rem",
                  color: "#22d3ee",
                  background: "rgba(34,211,238,0.06)",
                  border: "1px solid rgba(34,211,238,0.15)",
                  borderRadius: "6px",
                  padding: "0.25rem 0.5rem",
                  flexShrink: 0,
                  whiteSpace: "nowrap" as const,
                }}
              >
                {item.bin}
              </code>
              <div style={{ flex: 1 }}>
                <p style={{ fontSize: "0.875rem", color: "var(--muted)", margin: 0, lineHeight: 1.6 }}>{item.desc}</p>
              </div>
              <span
                style={{
                  flexShrink: 0,
                  fontSize: "0.65rem",
                  fontWeight: 600,
                  letterSpacing: "0.06em",
                  textTransform: "uppercase" as const,
                  color: statusColor.color,
                  background: statusColor.bg,
                  border: `1px solid ${statusColor.border}`,
                  borderRadius: "4px",
                  padding: "0.2rem 0.45rem",
                }}
              >
                {item.status}
              </span>
            </div>
          );
        })}
      </div>

      <div
        style={{
          marginTop: "1.5rem",
          padding: "1rem 1.125rem",
          background: "rgba(129,140,248,0.06)",
          border: "1px solid rgba(129,140,248,0.18)",
          borderRadius: "8px",
          fontSize: "0.875rem",
          color: "#a1a1aa",
          lineHeight: 1.65,
        }}
      >
        An install script (<code style={{ fontFamily: "var(--font-mono)", fontSize: "0.85em", color: "#86efac" }}>curl -sSf https://… | sh</code>)
        will install all three binaries into{" "}
        <code style={{ fontFamily: "var(--font-mono)", fontSize: "0.85em", color: "#86efac" }}>~/.code-lang/bin/</code>{" "}
        so the VS Code extension can find them automatically — no PATH changes needed.
      </div>

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
