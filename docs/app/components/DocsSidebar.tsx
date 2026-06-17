"use client";

import Link from "next/link";
import { usePathname } from "next/navigation";

const NAV = [
  {
    group: "Learn",
    items: [
      { label: "Getting started", href: "/docs" },
      { label: "Language reference", href: "/docs/language" },
    ],
  },
  {
    group: "Reference",
    items: [
      { label: "Standard library", href: "/docs/stdlib" },
      { label: "Install", href: "/install" },
      { label: "Changelog", href: "/changelog" },
    ],
  },
];

export default function DocsSidebar() {
  const pathname = usePathname();

  return (
    <aside className="docs-sidebar-col">
      <nav style={{ position: "sticky", top: "72px", display: "flex", flexDirection: "column", gap: "0.125rem" }}>
        {NAV.map((group) => (
          <div key={group.group} style={{ marginBottom: "1.25rem" }}>
            <p style={{ fontSize: "0.6875rem", fontWeight: 700, letterSpacing: "0.08em", textTransform: "uppercase", color: "#52525b", marginBottom: "0.375rem" }}>
              {group.group}
            </p>
            {group.items.map((item) => {
              const active = pathname === item.href;
              return (
                <Link
                  key={item.href}
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
              );
            })}
          </div>
        ))}
      </nav>
    </aside>
  );
}
