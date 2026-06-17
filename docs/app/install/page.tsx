import type { Metadata } from "next";
import Link from "next/link";

export const metadata: Metadata = { title: "Install" };

const S = {
  page: {
    maxWidth: "1100px",
    margin: "0 auto",
    padding: "4rem 1.5rem 6rem",
  } as React.CSSProperties,
  badge: {
    display: "inline-block",
    fontSize: "0.75rem",
    fontWeight: 500,
    color: "#818cf8",
    background: "rgba(129,140,248,0.08)",
    border: "1px solid rgba(129,140,248,0.18)",
    borderRadius: "100px",
    padding: "0.2rem 0.65rem",
    marginBottom: "1.5rem",
  } as React.CSSProperties,
  h1: {
    fontSize: "clamp(2rem, 5vw, 2.75rem)",
    fontWeight: 700,
    letterSpacing: "-0.04em",
    color: "var(--text)",
    marginBottom: "1rem",
    lineHeight: 1.15,
  } as React.CSSProperties,
  lead: {
    fontSize: "1.0625rem",
    color: "var(--muted)",
    lineHeight: 1.7,
    maxWidth: "520px",
    marginBottom: "3.5rem",
  } as React.CSSProperties,
  card: {
    background: "var(--surface)",
    border: "1px solid var(--border)",
    borderRadius: "12px",
    overflow: "hidden",
    marginBottom: "1.5rem",
  } as React.CSSProperties,
  cardHead: {
    padding: "0.75rem 1.25rem",
    borderBottom: "1px solid var(--border)",
    display: "flex",
    alignItems: "center",
    justifyContent: "space-between",
  } as React.CSSProperties,
  cardTitle: {
    fontSize: "0.8125rem",
    fontWeight: 600,
    color: "var(--text)",
  } as React.CSSProperties,
  pre: {
    margin: 0,
    padding: "1.25rem",
    fontFamily: "var(--font-mono)",
    fontSize: "0.875rem",
    color: "#d4d4d8",
    lineHeight: 1.75,
    overflowX: "auto" as const,
    background: "transparent",
  } as React.CSSProperties,
  sectionLabel: {
    fontSize: "0.6875rem",
    fontWeight: 700,
    letterSpacing: "0.1em",
    textTransform: "uppercase" as const,
    color: "#52525b",
    marginBottom: "1rem",
    marginTop: "3rem",
  } as React.CSSProperties,
  h2: {
    fontSize: "1.25rem",
    fontWeight: 700,
    letterSpacing: "-0.025em",
    color: "var(--text)",
    marginBottom: "0.75rem",
  } as React.CSSProperties,
  note: {
    fontSize: "0.875rem",
    color: "#52525b",
    lineHeight: 1.65,
  } as React.CSSProperties,
};

function Step({ n, title, children }: { n: number; title: string; children: React.ReactNode }) {
  return (
    <div style={{ display: "flex", gap: "1.25rem", marginBottom: "2rem" }}>
      <div
        style={{
          width: 28,
          height: 28,
          borderRadius: "50%",
          background: "rgba(129,140,248,0.12)",
          border: "1px solid rgba(129,140,248,0.25)",
          color: "#818cf8",
          fontSize: "0.8125rem",
          fontWeight: 700,
          display: "flex",
          alignItems: "center",
          justifyContent: "center",
          flexShrink: 0,
          marginTop: "0.125rem",
        }}
      >
        {n}
      </div>
      <div style={{ flex: 1 }}>
        <p style={{ fontWeight: 600, color: "var(--text)", fontSize: "0.9375rem", marginBottom: "0.625rem" }}>{title}</p>
        {children}
      </div>
    </div>
  );
}

export default function InstallPage() {
  return (
    <div style={S.page}>
      <div style={S.badge}>Installation guide</div>
      <h1 style={S.h1}>Install code-lang</h1>
      <p style={S.lead}>
        code-lang is currently distributed as source. Build it from source with Cargo in under a minute.
        Pre-built binaries are coming.
      </p>

      {/* ── Build from source ── */}
      <p style={S.sectionLabel}>Build from source</p>
      <h2 style={S.h2}>Prerequisites</h2>
      <p style={{ ...S.note, marginBottom: "1.5rem" }}>
        You need{" "}
        <a href="https://rustup.rs" target="_blank" rel="noopener noreferrer" style={{ color: "#818cf8" }}>
          Rust (stable)
        </a>{" "}
        installed. If you don&apos;t have it:
      </p>
      <div style={S.card}>
        <div style={S.cardHead}>
          <span style={S.cardTitle}>Install Rust</span>
          <span style={{ fontSize: "0.75rem", color: "#52525b" }}>rustup.rs</span>
        </div>
        <pre style={S.pre}>
          <code>curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh</code>
        </pre>
      </div>

      <h2 style={{ ...S.h2, marginTop: "2.5rem" }}>Steps</h2>

      <Step n={1} title="Clone the repository">
        <div style={S.card}>
          <pre style={S.pre}>
            <code>git clone https://github.com/Walon-Foundation/code-lang.git</code>
          </pre>
        </div>
      </Step>

      <Step n={2} title="Build in release mode">
        <div style={S.card}>
          <pre style={S.pre}>
            <code>{`cd code-lang
cargo build --release`}</code>
          </pre>
        </div>
        <p style={S.note}>This produces a single binary at <code style={{ fontFamily: "var(--font-mono)", fontSize: "0.85em", color: "#86efac" }}>target/release/code-lang</code>.</p>
      </Step>

      <Step n={3} title="(Optional) Add to PATH">
        <div style={S.card}>
          <div style={S.cardHead}>
            <span style={S.cardTitle}>Linux / macOS</span>
          </div>
          <pre style={S.pre}>
            <code>{`# copy to a directory already on your PATH
sudo cp target/release/code-lang /usr/local/bin/

# or add the build output to PATH in your shell config
export PATH="$PATH:/path/to/code-lang/target/release"`}</code>
          </pre>
        </div>
      </Step>

      <Step n={4} title="Verify the install">
        <div style={S.card}>
          <pre style={S.pre}>
            <code>{`code-lang --version   # (if on PATH)
# or
./target/release/code-lang --version`}</code>
          </pre>
        </div>
      </Step>

      {/* ── Usage ── */}
      <p style={{ ...S.sectionLabel, marginTop: "3.5rem" }}>Usage</p>
      <h2 style={S.h2}>Run the REPL</h2>
      <div style={S.card}>
        <pre style={S.pre}>
          <code>{`code-lang
>> let x = 10;
>> x * x;
100
>> exit`}</code>
        </pre>
      </div>

      <h2 style={{ ...S.h2, marginTop: "2rem" }}>Run a script</h2>
      <div style={S.card}>
        <pre style={S.pre}>
          <code>code-lang hello.cl</code>
        </pre>
      </div>
      <p style={S.note}>Scripts must use the <code style={{ fontFamily: "var(--font-mono)", fontSize: "0.85em", color: "#86efac" }}>.cl</code> extension.</p>

      {/* ── Coming soon ── */}
      <div
        style={{
          marginTop: "4rem",
          background: "rgba(129,140,248,0.05)",
          border: "1px solid rgba(129,140,248,0.15)",
          borderRadius: "10px",
          padding: "1.5rem",
        }}
      >
        <p style={{ fontWeight: 600, color: "var(--text)", fontSize: "0.9375rem", marginBottom: "0.375rem" }}>
          Pre-built binaries — coming soon
        </p>
        <p style={{ fontSize: "0.875rem", color: "var(--muted)", lineHeight: 1.65, margin: 0 }}>
          We&apos;re working on distributing pre-built binaries for Linux, macOS, and Windows so you
          won&apos;t need Rust installed. Watch the{" "}
          <a href="https://github.com/Walon-Foundation/code-lang/releases" target="_blank" rel="noopener noreferrer" style={{ color: "#818cf8" }}>
            releases page
          </a>{" "}
          or the{" "}
          <Link href="/changelog" style={{ color: "#818cf8" }}>changelog</Link>{" "}
          for updates.
        </p>
      </div>

      {/* ── Next steps ── */}
      <p style={{ ...S.sectionLabel, marginTop: "3.5rem" }}>Next steps</p>
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
              borderRadius: "10px",
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
