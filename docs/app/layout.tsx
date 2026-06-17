import type { Metadata } from "next";
import { Geist, Geist_Mono } from "next/font/google";
import Image from "next/image";
import Link from "next/link";
import Navbar from "./components/Navbar";
import "./globals.css";

const geistSans = Geist({ variable: "--font-geist-sans", subsets: ["latin"] });
const geistMono = Geist_Mono({ variable: "--font-geist-mono", subsets: ["latin"] });

export const metadata: Metadata = {
  title: { default: "code-lang", template: "%s — code-lang" },
  description: "A general-purpose interpreted programming language written in Rust.",
  icons: { icon: "/icon.svg" },
};

export default function RootLayout({ children }: { children: React.ReactNode }) {
  return (
    <html lang="en" className={`${geistSans.variable} ${geistMono.variable}`}>
      <body>
        <Navbar />

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
              <a href="https://github.com/Walon-Foundation/code-lang" target="_blank" rel="noopener noreferrer" style={{ color: "#52525b", textDecoration: "none" }}>GitHub</a>
              <Link href="/docs" style={{ color: "#52525b", textDecoration: "none" }}>Docs</Link>
              <Link href="/docs/stdlib" style={{ color: "#52525b", textDecoration: "none" }}>Stdlib</Link>
              <Link href="/changelog" style={{ color: "#52525b", textDecoration: "none" }}>Changelog</Link>
              <Link href="/install" style={{ color: "#52525b", textDecoration: "none" }}>Install</Link>
            </div>
          </div>
        </footer>
      </body>
    </html>
  );
}
