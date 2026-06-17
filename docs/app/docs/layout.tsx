import Link from "next/link";

const NAV = [
  { group: "Learn", items: [
    { label: "Getting started", href: "/docs" },
    { label: "Language reference", href: "/docs/language" },
  ]},
  { group: "Reference", items: [
    { label: "Standard library", href: "/docs/stdlib" },
    { label: "Install", href: "/install" },
    { label: "Changelog", href: "/changelog" },
  ]},
];

export default function DocsLayout({ children }: { children: React.ReactNode }) {
  return (
    <div className="docs-layout">
      <aside className="docs-sidebar-col">
        <nav style={{ position: "sticky", top: "72px", display: "flex", flexDirection: "column", gap: "0.125rem" }}>
          {NAV.map((group) => (
            <div key={group.group} style={{ marginBottom: "1.25rem" }}>
              <p style={{ fontSize: "0.6875rem", fontWeight: 700, letterSpacing: "0.08em", textTransform: "uppercase", color: "#52525b", marginBottom: "0.375rem" }}>
                {group.group}
              </p>
              {group.items.map((item) => (
                <Link
                  key={item.href}
                  href={item.href}
                  style={{ display: "block", fontSize: "0.875rem", color: "var(--muted)", textDecoration: "none", padding: "0.25rem 0" }}
                >
                  {item.label}
                </Link>
              ))}
            </div>
          ))}
        </nav>
      </aside>
      <article className="docs-content" style={{ flex: 1, minWidth: 0 }}>
        {children}
      </article>
    </div>
  );
}
