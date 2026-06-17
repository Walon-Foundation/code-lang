import type { Metadata, Viewport } from "next";
import { Geist, Geist_Mono } from "next/font/google";
import Image from "next/image";
import Link from "next/link";
import Navbar from "./components/Navbar";
import "./globals.css";

const geistSans = Geist({ variable: "--font-geist-sans", subsets: ["latin"] });
const geistMono = Geist_Mono({ variable: "--font-geist-mono", subsets: ["latin"] });

const SITE_URL = "https://code-lang.walonfoundation.com";
const DESCRIPTION = "A general-purpose interpreted programming language written in Rust.";

export const viewport: Viewport = {
  width: "device-width",
  initialScale: 1,
  themeColor: "#09090b",
};

export const metadata: Metadata = {
  metadataBase: new URL(SITE_URL),
  title: { default: "code-lang", template: "%s — code-lang" },
  description: DESCRIPTION,
  keywords: [
    "programming language",
    "interpreter",
    "scripting language",
    "rust",
    "code-lang",
    ".cl files",
    "open source",
  ],
  authors: [{ name: "Walon Foundation", url: "https://github.com/Walon-Foundation" }],
  creator: "Walon Foundation",
  icons: { icon: "/icon.svg" },
  manifest: "/manifest.webmanifest",
  openGraph: {
    type: "website",
    locale: "en_US",
    url: SITE_URL,
    siteName: "code-lang",
    title: "code-lang",
    description: DESCRIPTION,
    images: [{ url: "/opengraph-image", width: 1200, height: 630, alt: "code-lang" }],
  },
  twitter: {
    card: "summary_large_image",
    title: "code-lang",
    description: DESCRIPTION,
    images: ["/opengraph-image"],
  },
  alternates: { canonical: SITE_URL },
  robots: {
    index: true,
    follow: true,
    googleBot: { index: true, follow: true, "max-image-preview": "large" },
  },
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
