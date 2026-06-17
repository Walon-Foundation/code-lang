import type { Metadata } from "next";
import { Geist, Geist_Mono } from "next/font/google";
import Image from "next/image";
import Link from "next/link";
import "./globals.css";

const geistSans = Geist({ variable: "--font-geist-sans", subsets: ["latin"] });
const geistMono = Geist_Mono({ variable: "--font-geist-mono", subsets: ["latin"] });

export const metadata: Metadata = {
  title: { default: "code-lang", template: "%s — code-lang" },
  description: "A general-purpose interpreted programming language written in Rust.",
  icons: { icon: "/icon.svg" },
};

const navLinks = [
  { label: "Docs", href: "/docs" },
  { label: "Language", href: "/docs/language" },
  { label: "Stdlib", href: "/docs/stdlib" },
];

export default function RootLayout({ children }: { children: React.ReactNode }) {
  return (
    <html lang="en" className={`${geistSans.variable} ${geistMono.variable}`}>
      <body>
        <header
          style={{
            borderBottom: "1px solid var(--border)",
            position: "sticky",
            top: 0,
            zIndex: 50,
            background: "rgba(9,9,11,0.85)",
            backdropFilter: "blur(12px)",
          }}
        >
          <div
            style={{
              maxWidth: "1100px",
              margin: "0 auto",
              padding: "0 1.5rem",
              height: "56px",
              display: "flex",
              alignItems: "center",
              justifyContent: "space-between",
            }}
          >
            <Link href="/" style={{ display: "flex", alignItems: "center", gap: "0.5rem", textDecoration: "none" }}>
              <Image src="/logo.svg" alt="" width={26} height={26} />
              <span style={{ fontWeight: 600, fontSize: "0.9375rem", color: "var(--text)", letterSpacing: "-0.015em" }}>
                code-lang
              </span>
            </Link>
            <nav style={{ display: "flex", alignItems: "center", gap: "2rem" }}>
              {navLinks.map((item) => (
                <Link
                  key={item.href}
                  href={item.href}
                  style={{ fontSize: "0.875rem", color: "var(--muted)", textDecoration: "none" }}
                >
                  {item.label}
                </Link>
              ))}
              <a
                href="https://github.com/Walon-Foundation/code-lang"
                target="_blank"
                rel="noopener noreferrer"
                style={{
                  fontSize: "0.8125rem",
                  fontWeight: 500,
                  color: "var(--text)",
                  textDecoration: "none",
                  background: "var(--surface)",
                  border: "1px solid var(--border)",
                  borderRadius: "6px",
                  padding: "0.3rem 0.875rem",
                }}
              >
                GitHub
              </a>
            </nav>
          </div>
        </header>

        <div style={{ flex: 1 }}>{children}</div>

        <footer style={{ borderTop: "1px solid var(--border)", marginTop: "6rem" }}>
          <div
            style={{
              maxWidth: "1100px",
              margin: "0 auto",
              padding: "1.75rem 1.5rem",
              display: "flex",
              alignItems: "center",
              justifyContent: "space-between",
              fontSize: "0.8125rem",
              color: "#52525b",
            }}
          >
            <div style={{ display: "flex", alignItems: "center", gap: "0.5rem" }}>
              <Image src="/logo.svg" alt="" width={18} height={18} style={{ opacity: 0.5 }} />
              <span>code-lang — MIT License</span>
            </div>
            <div style={{ display: "flex", gap: "1.5rem" }}>
              <a href="https://github.com/Walon-Foundation/code-lang" target="_blank" rel="noopener noreferrer" style={{ color: "#52525b", textDecoration: "none" }}>
                GitHub
              </a>
              <Link href="/docs" style={{ color: "#52525b", textDecoration: "none" }}>Docs</Link>
              <Link href="/docs/stdlib" style={{ color: "#52525b", textDecoration: "none" }}>Stdlib</Link>
            </div>
          </div>
        </footer>
      </body>
    </html>
  );
}
