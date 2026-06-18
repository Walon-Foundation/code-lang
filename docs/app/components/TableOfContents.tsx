"use client";

import { useEffect, useState } from "react";
import { usePathname } from "next/navigation";

type Heading = { id: string; label: string };

export default function TableOfContents() {
  const pathname = usePathname();
  const [headings, setHeadings] = useState<Heading[]>([]);
  const [activeId, setActiveId] = useState("");

  // Re-read headings whenever the page changes
  useEffect(() => {
    const els = Array.from(document.querySelectorAll("h2[id]")) as HTMLElement[];
    setHeadings(els.map((el) => ({ id: el.id, label: el.innerText })));
    setActiveId("");
  }, [pathname]);

  // Scroll-spy
  useEffect(() => {
    if (headings.length === 0) return;

    const observer = new IntersectionObserver(
      (entries) => {
        for (const entry of entries) {
          if (entry.isIntersecting) {
            setActiveId(entry.target.id);
            break;
          }
        }
      },
      { rootMargin: "-10% 0px -80% 0px" }
    );

    headings.forEach(({ id }) => {
      const el = document.getElementById(id);
      if (el) observer.observe(el);
    });

    return () => observer.disconnect();
  }, [headings]);

  if (headings.length === 0) return null;

  return (
    <aside className="docs-toc-col">
      <nav>
        <p style={{ fontSize: "0.6875rem", fontWeight: 700, letterSpacing: "0.08em", textTransform: "uppercase", color: "#52525b", marginBottom: "0.625rem" }}>
          On this page
        </p>
        <div style={{ display: "flex", flexDirection: "column", gap: "0.0625rem" }}>
          {headings.map(({ id, label }) => {
            const isActive = activeId === id;
            return (
              <a
                key={id}
                href={`#${id}`}
                style={{
                  fontSize: "0.8125rem",
                  color: isActive ? "var(--text)" : "#52525b",
                  textDecoration: "none",
                  padding: "0.2rem 0.5rem",
                  marginLeft: "-0.5rem",
                  borderRadius: "4px",
                  fontWeight: isActive ? 500 : 400,
                  background: isActive ? "rgba(129,140,248,0.07)" : "transparent",
                  transition: "color 0.1s, background 0.1s",
                  display: "block",
                  lineHeight: 1.5,
                }}
              >
                {label}
              </a>
            );
          })}
        </div>
      </nav>
    </aside>
  );
}
