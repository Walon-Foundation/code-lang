import type { Metadata } from "next";
import { Geist, Geist_Mono } from "next/font/google";
import Link from "next/link";
import "./globals.css";

const geistSans = Geist({ variable: "--font-geist-sans", subsets: ["latin"] });
const geistMono = Geist_Mono({ variable: "--font-geist-mono", subsets: ["latin"] });

export const metadata: Metadata = {
  title: { default: "code-lang", template: "%s — code-lang" },
  description: "A general-purpose interpreted programming language written in Rust.",
};

export default function RootLayout({ children }: { children: React.ReactNode }) {
  return (
    <html lang="en" className={`${geistSans.variable} ${geistMono.variable} h-full antialiased`}>
      <body className="min-h-full flex flex-col bg-white text-zinc-900 dark:bg-zinc-950 dark:text-zinc-100">
        <header className="border-b border-zinc-200 dark:border-zinc-800">
          <div className="max-w-5xl mx-auto px-6 h-14 flex items-center justify-between">
            <Link href="/" className="font-semibold text-sm tracking-tight">
              code-lang
            </Link>
            <nav className="flex items-center gap-6 text-sm text-zinc-600 dark:text-zinc-400">
              <Link href="/docs" className="hover:text-zinc-900 dark:hover:text-zinc-100 transition-colors">Docs</Link>
              <Link href="/docs/language" className="hover:text-zinc-900 dark:hover:text-zinc-100 transition-colors">Language</Link>
              <Link href="/docs/stdlib" className="hover:text-zinc-900 dark:hover:text-zinc-100 transition-colors">Stdlib</Link>
              <a
                href="https://github.com/Walon-Foundation/code-lang"
                target="_blank"
                rel="noopener noreferrer"
                className="hover:text-zinc-900 dark:hover:text-zinc-100 transition-colors"
              >
                GitHub
              </a>
            </nav>
          </div>
        </header>
        {children}
      </body>
    </html>
  );
}
