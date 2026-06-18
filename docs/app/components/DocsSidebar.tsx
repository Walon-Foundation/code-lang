"use client";

import Link from "next/link";
import { usePathname } from "next/navigation";
import { useEffect, useState } from "react";

type Section = { label: string; hash: string };

const NAV: { group: string; items: { label: string; href: string; sections?: Section[] }[] }[] = [
  {
    group: "Learn",
    items: [
      {
        label: "Getting started",
        href: "/docs",
        sections: [
          { label: "Install", hash: "install" },
          { label: "The REPL", hash: "the-repl" },
          { label: "Your first script", hash: "your-first-script" },
          { label: "Importing modules", hash: "importing-modules" },
          { label: "Error format", hash: "error-format" },
          { label: "Next steps", hash: "next-steps" },
        ],
      },
      {
        label: "Language reference",
        href: "/docs/language",
        sections: [
          { label: "Comments", hash: "comments" },
          { label: "Variables", hash: "variables" },
          { label: "Types", hash: "types" },
          { label: "Operators", hash: "operators" },
          { label: "Control flow", hash: "control-flow" },
          { label: "Functions", hash: "functions" },
          { label: "Arrays", hash: "arrays" },
          { label: "Hashes", hash: "hashes" },
          { label: "Structs", hash: "structs" },
          { label: "Modules", hash: "modules" },
        ],
      },
    ],
  },
  {
    group: "Reference",
    items: [
      {
        label: "Standard library",
        href: "/docs/stdlib",
        sections: [
          { label: "fmt", hash: "fmt" },
          { label: "math", hash: "math" },
          { label: "strings", hash: "strings" },
          { label: "arrays", hash: "arrays" },
          { label: "hash", hash: "hash" },
          { label: "fs", hash: "fs" },
          { label: "path", hash: "path" },
          { label: "os", hash: "os" },
          { label: "time", hash: "time" },
          { label: "json", hash: "json" },
          { label: "rand", hash: "rand" },
          { label: "http", hash: "http" },
        ],
      },
      { label: "Install", href: "/install" },
      { label: "Changelog", href: "/changelog" },
    ],
  },
];

export default function DocsSidebar() {
  const pathname = usePathname();
  const [activeHash, setActiveHash] = useState("");

  useEffect(() => {
    const headings = Array.from(document.querySelectorAll("h2[id]")) as HTMLElement[];
    if (headings.length === 0) return;

    const observer = new IntersectionObserver(
      (entries) => {
        for (const entry of entries) {
          if (entry.isIntersecting) {
            setActiveHash(entry.target.id);
            break;
          }
        }
      },
      { rootMargin: "-10% 0px -80% 0px" }
    );

    headings.forEach((el) => observer.observe(el));
    return () => observer.disconnect();
  }, [pathname]);

  return (
    <aside className="docs-sidebar-col">
      <nav style={{ display: "flex", flexDirection: "column", gap: "0.125rem" }}>
        {NAV.map((group) => (
          <div key={group.group} style={{ marginBottom: "1.25rem" }}>
            <p style={{ fontSize: "0.6875rem", fontWeight: 700, letterSpacing: "0.08em", textTransform: "uppercase", color: "#52525b", marginBottom: "0.375rem" }}>
              {group.group}
            </p>

            {group.items.map((item) => {
              const active = pathname === item.href;
              return (
                <div key={item.href}>
                  <Link
                    href={item.href}
                    style={{
                      display: "block",
                      fontSize: "0.875rem",
                      color: active ? "var(--text)" : "var(--muted)",
                      textDecoration: "none",
                      padding: "0.25rem 0.5rem",
                      marginLeft: "-0.5rem",
                      borderRadius: "5px",
                      fontWeight: active ? 500 : 400,
                      background: active ? "rgba(129,140,248,0.08)" : "transparent",
                    }}
                  >
                    {item.label}
                  </Link>

                  {/* Section sub-links — only shown on the active page */}
                  {active && item.sections && (
                    <div style={{ marginLeft: "0.5rem", marginTop: "0.125rem", marginBottom: "0.375rem", borderLeft: "1px solid var(--border)", paddingLeft: "0.75rem", display: "flex", flexDirection: "column", gap: "0.0625rem" }}>
                      {item.sections.map((sec) => {
                        const isActiveSection = activeHash === sec.hash;
                        return (
                          <a
                            key={sec.hash}
                            href={`#${sec.hash}`}
                            style={{
                              display: "block",
                              fontSize: "0.8125rem",
                              color: isActiveSection ? "var(--text)" : "#52525b",
                              textDecoration: "none",
                              padding: "0.2rem 0.375rem",
                              borderRadius: "4px",
                              fontWeight: isActiveSection ? 500 : 400,
                              background: isActiveSection ? "rgba(129,140,248,0.07)" : "transparent",
                              transition: "color 0.1s, background 0.1s",
                            }}
                          >
                            {sec.label}
                          </a>
                        );
                      })}
                    </div>
                  )}
                </div>
              );
            })}
          </div>
        ))}
      </nav>
    </aside>
  );
}
