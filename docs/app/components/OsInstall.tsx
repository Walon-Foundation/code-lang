"use client";

import { useState, useEffect } from "react";

type OS = "linux" | "macos" | "windows";

function detectOS(): OS {
  if (typeof navigator === "undefined") return "linux";
  const ua = navigator.userAgent;
  if (ua.includes("Windows")) return "windows";
  if (ua.includes("Mac")) return "macos";
  return "linux";
}

function CopyIconButton({ text }: { text: string }) {
  const [copied, setCopied] = useState(false);
  const copy = () =>
    navigator.clipboard.writeText(text).then(() => {
      setCopied(true);
      setTimeout(() => setCopied(false), 2000);
    });
  return (
    <button
      onClick={copy}
      aria-label="Copy"
      title={copied ? "Copied!" : "Copy"}
      style={{
        display: "flex",
        alignItems: "center",
        justifyContent: "center",
        width: 28,
        height: 28,
        border: "1px solid var(--border)",
        borderRadius: 6,
        background: "transparent",
        color: copied ? "#22d3ee" : "var(--muted)",
        cursor: "pointer",
        flexShrink: 0,
        transition: "color 0.15s, border-color 0.15s",
        padding: 0,
      }}
    >
      {copied ? (
        <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2.5" strokeLinecap="round" strokeLinejoin="round">
          <polyline points="20 6 9 17 4 12" />
        </svg>
      ) : (
        <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
          <rect x="9" y="9" width="13" height="13" rx="2" ry="2" />
          <path d="M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1" />
        </svg>
      )}
    </button>
  );
}

function CodeCard({ title, tag, code }: { title: string; tag: string; code: string }) {
  return (
    <div
      className="install-card"
      style={{
        border: "1px solid var(--border)",
        borderRadius: 12,
        marginBottom: "1rem",
        background: "var(--surface)",
        minWidth: 0,
      }}
    >
      {/* Header — rounded top corners match the card */}
      <div
        className="install-card-head"
        style={{
          padding: "0.75rem 1.25rem",
          borderBottom: "1px solid var(--border)",
          display: "flex",
          alignItems: "center",
          justifyContent: "space-between",
          borderRadius: "12px 12px 0 0",
          gap: "0.75rem",
        }}
      >
        <span style={{ fontSize: "0.8125rem", fontWeight: 600, color: "var(--text)", minWidth: 0 }}>{title}</span>
        <div style={{ display: "flex", alignItems: "center", gap: "0.625rem", flexShrink: 0 }}>
          <span className="install-tag" style={{ fontSize: "0.75rem", color: "#52525b", whiteSpace: "nowrap" }}>{tag}</span>
          <CopyIconButton text={code} />
        </div>
      </div>
      {/* Pre — scrolls horizontally; rounded bottom corners */}
      <pre
        style={{
          margin: 0,
          padding: "1.25rem",
          fontFamily: "var(--font-mono)",
          fontSize: "0.875rem",
          color: "#d4d4d8",
          lineHeight: 1.75,
          overflowX: "auto",
          background: "transparent",
          borderRadius: "0 0 12px 12px",
        }}
      >
        <code>{code}</code>
      </pre>
    </div>
  );
}

function Code({ children }: { children: React.ReactNode }) {
  return <code style={{ fontFamily: "var(--font-mono)", fontSize: "0.85em", color: "#86efac" }}>{children}</code>;
}

function Step({ n, title, children }: { n: number; title: string; children: React.ReactNode }) {
  return (
    <div style={{ display: "flex", gap: "1rem", marginBottom: "2rem" }}>
      <div style={{
        width: 28, height: 28, borderRadius: "50%",
        background: "rgba(129,140,248,0.12)", border: "1px solid rgba(129,140,248,0.25)",
        color: "#818cf8", fontSize: "0.8125rem", fontWeight: 700,
        display: "flex", alignItems: "center", justifyContent: "center",
        flexShrink: 0, marginTop: "0.125rem",
      }}>
        {n}
      </div>
      <div style={{ flex: 1, minWidth: 0 }}>
        <p style={{ fontWeight: 600, color: "var(--text)", fontSize: "0.9375rem", marginBottom: "0.625rem" }}>{title}</p>
        {children}
      </div>
    </div>
  );
}

const LINUX_CMD  = "curl -fsSL https://raw.githubusercontent.com/Walon-Foundation/code-lang/main/install.sh | sh";
const MACOS_CMD  = "curl -fsSL https://raw.githubusercontent.com/Walon-Foundation/code-lang/main/install.sh | sh";
const WINDOWS_CMD = "irm https://raw.githubusercontent.com/Walon-Foundation/code-lang/main/install.ps1 | iex";

const OS_META: Record<OS, { label: string; note: React.ReactNode; card: React.ReactNode }> = {
  linux: {
    label: "Linux",
    note: <p style={{ fontSize: "0.875rem", color: "#52525b", lineHeight: 1.65, marginBottom: "1rem" }}>Installs to <Code>~/.local/bin</Code>. The script will tell you if that directory needs adding to your PATH.</p>,
    card: <CodeCard title="Shell" tag="bash / zsh / sh" code={LINUX_CMD} />,
  },
  macos: {
    label: "macOS",
    note: <p style={{ fontSize: "0.875rem", color: "#52525b", lineHeight: 1.65, marginBottom: "1rem" }}>Installs to <Code>~/.local/bin</Code>. The script will tell you if that directory needs adding to your PATH.</p>,
    card: <CodeCard title="Shell" tag="bash / zsh" code={MACOS_CMD} />,
  },
  windows: {
    label: "Windows",
    note: <p style={{ fontSize: "0.875rem", color: "#52525b", lineHeight: 1.65, marginBottom: "1rem" }}>Installs to <Code>%LOCALAPPDATA%\code-lang\bin</Code> and adds it to your user PATH. Restart your terminal after install.</p>,
    card: <CodeCard title="PowerShell" tag="Windows 10/11" code={WINDOWS_CMD} />,
  },
};

const OTHER_OS: Record<OS, OS[]> = {
  linux:   ["macos", "windows"],
  macos:   ["linux", "windows"],
  windows: ["linux", "macos"],
};

function Chevron({ open }: { open: boolean }) {
  return (
    <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2.5" strokeLinecap="round" strokeLinejoin="round"
      style={{ transform: open ? "rotate(90deg)" : "none", transition: "transform 0.15s", flexShrink: 0 }}>
      <polyline points="9 18 15 12 9 6" />
    </svg>
  );
}

function DisclosureBtn({ open, onClick, children }: { open: boolean; onClick: () => void; children: React.ReactNode }) {
  return (
    <button
      onClick={onClick}
      style={{
        display: "flex", alignItems: "center", gap: "0.4rem",
        fontSize: "0.8125rem", color: "#52525b",
        background: "none", border: "none", cursor: "pointer",
        padding: 0, marginTop: "1.25rem",
        marginBottom: open ? "1.25rem" : 0,
      }}
    >
      <Chevron open={open} />
      {children}
    </button>
  );
}

export default function OsInstall() {
  const [os, setOs] = useState<OS>("linux");
  const [showOther, setShowOther] = useState(false);
  const [showSource, setShowSource] = useState(false);

  useEffect(() => { setOs(detectOS()); }, []);

  const meta = OS_META[os];

  return (
    <div style={{ minWidth: 0, width: "100%" }}>
      {/* Detected OS label */}
      <div style={{ display: "flex", alignItems: "center", gap: "0.5rem", marginBottom: "0.875rem", flexWrap: "wrap" }}>
        <p style={{ fontSize: "0.6875rem", fontWeight: 700, letterSpacing: "0.1em", textTransform: "uppercase", color: "#52525b", margin: 0 }}>
          {meta.label}
        </p>
        <span style={{ fontSize: "0.6875rem", color: "#3f3f46", background: "rgba(129,140,248,0.08)", border: "1px solid rgba(129,140,248,0.15)", borderRadius: 100, padding: "0.1rem 0.45rem" }}>
          detected
        </span>
      </div>

      {meta.note}
      {meta.card}

      {/* Other OS disclosure */}
      <DisclosureBtn open={showOther} onClick={() => setShowOther((v) => !v)}>
        Install on a different OS
      </DisclosureBtn>

      {showOther && (
        <div style={{ display: "flex", flexDirection: "column", gap: "1.25rem" }}>
          {OTHER_OS[os].map((other) => (
            <div key={other}>
              <p style={{ fontSize: "0.75rem", fontWeight: 600, color: "#52525b", marginBottom: "0.5rem", textTransform: "capitalize" }}>
                {OS_META[other].label}
              </p>
              {OS_META[other].card}
            </div>
          ))}
        </div>
      )}

      {/* Build from source disclosure */}
      <DisclosureBtn open={showSource} onClick={() => setShowSource((v) => !v)}>
        Build from source
      </DisclosureBtn>

      {showSource && (
        <div style={{ marginTop: "0.25rem", minWidth: 0 }}>
          <p style={{ fontSize: "0.875rem", color: "#52525b", lineHeight: 1.65, marginBottom: "1.5rem" }}>
            Requires{" "}
            <a href="https://rustup.rs" target="_blank" rel="noopener noreferrer" style={{ color: "#818cf8" }}>Rust (stable)</a>.
            Gives you the latest unreleased changes.
          </p>

          <Step n={1} title="Install Rust">
            <CodeCard title="Install Rust" tag="rustup.rs" code="curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh" />
          </Step>
          <Step n={2} title="Clone the repository">
            <CodeCard title="Clone" tag="git" code="git clone https://github.com/Walon-Foundation/code-lang.git" />
          </Step>
          <Step n={3} title="Build in release mode">
            <CodeCard title="Build" tag="cargo" code={"cd code-lang\ncargo build --release"} />
            <p style={{ fontSize: "0.875rem", color: "#52525b" }}>Binary is at <Code>target/release/code-lang</Code>.</p>
          </Step>
          <Step n={4} title="(Optional) Add to PATH">
            <CodeCard title="Linux / macOS" tag="sh" code="sudo cp target/release/code-lang /usr/local/bin/" />
          </Step>
        </div>
      )}
    </div>
  );
}
