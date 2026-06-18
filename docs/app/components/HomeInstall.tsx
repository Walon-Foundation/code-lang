"use client";

import { useState, useEffect } from "react";
import Link from "next/link";

type OS = "linux" | "macos" | "windows";

function detectOS(): OS {
  if (typeof navigator === "undefined") return "linux";
  const ua = navigator.userAgent;
  if (ua.includes("Windows")) return "windows";
  if (ua.includes("Mac")) return "macos";
  return "linux";
}

const CMDS: Record<OS, { label: string; cmd: string }> = {
  linux:   { label: "Linux",   cmd: "curl -fsSL https://raw.githubusercontent.com/Walon-Foundation/code-lang/main/install.sh | sh" },
  macos:   { label: "macOS",   cmd: "curl -fsSL https://raw.githubusercontent.com/Walon-Foundation/code-lang/main/install.sh | sh" },
  windows: { label: "Windows", cmd: "irm https://raw.githubusercontent.com/Walon-Foundation/code-lang/main/install.ps1 | iex" },
};

export default function HomeInstall() {
  const [os, setOs] = useState<OS>("linux");
  const [copied, setCopied] = useState(false);

  useEffect(() => { setOs(detectOS()); }, []);

  const { label, cmd } = CMDS[os];

  const copy = () =>
    navigator.clipboard.writeText(cmd).then(() => {
      setCopied(true);
      setTimeout(() => setCopied(false), 2000);
    });

  return (
    <div>
      <div style={{ background: "var(--surface)", border: "1px solid var(--border)", borderRadius: "10px" }}>
        <div style={{ padding: "0.625rem 1rem", borderBottom: "1px solid var(--border)", display: "flex", alignItems: "center", justifyContent: "space-between", gap: "0.75rem" }}>
          <span style={{ fontSize: "0.75rem", color: "#52525b", fontWeight: 500 }}>{label}</span>
          <button
            onClick={copy}
            aria-label="Copy"
            title={copied ? "Copied!" : "Copy"}
            style={{ display: "flex", alignItems: "center", justifyContent: "center", width: 26, height: 26, border: "1px solid var(--border)", borderRadius: 6, background: "transparent", color: copied ? "#22d3ee" : "var(--muted)", cursor: "pointer", flexShrink: 0, transition: "color 0.15s", padding: 0 }}
          >
            {copied ? (
              <svg width="13" height="13" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2.5" strokeLinecap="round" strokeLinejoin="round">
                <polyline points="20 6 9 17 4 12" />
              </svg>
            ) : (
              <svg width="13" height="13" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
                <rect x="9" y="9" width="13" height="13" rx="2" ry="2" />
                <path d="M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1" />
              </svg>
            )}
          </button>
        </div>
        <pre style={{ margin: 0, padding: "1rem", fontFamily: "var(--font-mono)", fontSize: "0.8125rem", color: "#d4d4d8", lineHeight: 1.7, overflowX: "auto" }}>
          <code>{cmd}</code>
        </pre>
      </div>
      <p style={{ fontSize: "0.875rem", color: "#52525b", marginTop: "1rem" }}>
        <Link href="/install" style={{ color: "#818cf8" }}>Other platforms &amp; install guide →</Link>
      </p>
    </div>
  );
}
