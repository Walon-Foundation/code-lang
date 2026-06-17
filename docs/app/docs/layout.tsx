import Link from "next/link";

const NAV = [
  { label: "Getting started", href: "/docs" },
  { label: "Language reference", href: "/docs/language" },
  { label: "Standard library", href: "/docs/stdlib" },
];

export default function DocsLayout({ children }: { children: React.ReactNode }) {
  return (
    <div
      style={{
        maxWidth: "1100px",
        margin: "0 auto",
        padding: "3rem 1.5rem",
        display: "flex",
        gap: "4rem",
        alignItems: "flex-start",
      }}
    >
      <aside style={{ width: "180px", flexShrink: 0 }}>
        <nav style={{ position: "sticky", top: "72px", display: "flex", flexDirection: "column", gap: "0.125rem" }}>
          <p style={{ fontSize: "0.6875rem", fontWeight: 600, letterSpacing: "0.08em", textTransform: "uppercase", color: "#52525b", marginBottom: "0.5rem" }}>
            Docs
          </p>
          {NAV.map((item) => (
            <Link
              key={item.href}
              href={item.href}
              style={{ fontSize: "0.875rem", color: "var(--muted)", textDecoration: "none", padding: "0.3rem 0" }}
            >
              {item.label}
            </Link>
          ))}
        </nav>
      </aside>
      <article className="docs-content" style={{ flex: 1, minWidth: 0 }}>
        {children}
      </article>
    </div>
  );
}
