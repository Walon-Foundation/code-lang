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

        <footer className="site-footer">
          <div className="pw site-footer-inner">
            <div style={{ display: "flex", alignItems: "center", gap: "0.5rem" }}>
              <Image src="/logo.svg" alt="" width={18} height={18} style={{ opacity: 0.5 }} />
              <span>code-lang — MIT License</span>
            </div>
            <nav className="footer-links">
              <a href="https://github.com/Walon-Foundation/code-lang" target="_blank" rel="noopener noreferrer">GitHub</a>
              <Link href="/docs">Docs</Link>
              <Link href="/docs/stdlib">Stdlib</Link>
              <Link href="/changelog">Changelog</Link>
              <Link href="/install">Install</Link>
            </nav>
          </div>
        </footer>
      </body>
    </html>
  );
}
