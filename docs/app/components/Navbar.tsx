"use client";

import Image from "next/image";
import Link from "next/link";
import { useState } from "react";

const navLinks = [
  { label: "Docs", href: "/docs" },
  { label: "Language", href: "/docs/language" },
  { label: "Stdlib", href: "/docs/stdlib" },
  { label: "Changelog", href: "/changelog" },
  { label: "Install", href: "/install" },
];

export default function Navbar() {
  const [open, setOpen] = useState(false);

  return (
    <header className="nav-header">
      <div className="nav-container">
        {/* Logo */}
        <Link href="/" className="nav-logo" onClick={() => setOpen(false)}>
          <Image src="/logo.svg" alt="" width={26} height={26} />
          <span>code-lang</span>
        </Link>

        {/* Desktop links */}
        <nav className="nav-desktop">
          {navLinks.map((item) => (
            <Link key={item.href} href={item.href} className="nav-link">
              {item.label}
            </Link>
          ))}
          <a
            href="https://github.com/Walon-Foundation/code-lang"
            target="_blank"
            rel="noopener noreferrer"
            className="nav-github"
          >
            GitHub
          </a>
        </nav>

        {/* Hamburger */}
        <button
          className="nav-hamburger"
          onClick={() => setOpen((v) => !v)}
          aria-label={open ? "Close menu" : "Open menu"}
        >
          {open ? (
            <svg width="20" height="20" viewBox="0 0 20 20" fill="none" stroke="currentColor" strokeWidth="1.75" strokeLinecap="round">
              <path d="M4 4l12 12M16 4L4 16" />
            </svg>
          ) : (
            <svg width="20" height="20" viewBox="0 0 20 20" fill="none" stroke="currentColor" strokeWidth="1.75" strokeLinecap="round">
              <path d="M3 5h14M3 10h14M3 15h14" />
            </svg>
          )}
        </button>
      </div>

      {/* Mobile menu */}
      {open && (
        <div className="nav-mobile">
          {navLinks.map((item) => (
            <Link
              key={item.href}
              href={item.href}
              className="nav-mobile-link"
              onClick={() => setOpen(false)}
            >
              {item.label}
            </Link>
          ))}
          <a
            href="https://github.com/Walon-Foundation/code-lang"
            target="_blank"
            rel="noopener noreferrer"
            className="nav-mobile-link"
            onClick={() => setOpen(false)}
          >
            GitHub ↗
          </a>
        </div>
      )}
    </header>
  );
}
