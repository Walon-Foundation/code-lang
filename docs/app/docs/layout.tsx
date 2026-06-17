import type { Metadata } from "next";
import Link from "next/link";

export const metadata: Metadata = { title: "Docs" };

const NAV = [
  { label: "Getting started", href: "/docs" },
  { label: "Language reference", href: "/docs/language" },
  { label: "Standard library", href: "/docs/stdlib" },
];

export default function DocsLayout({ children }: { children: React.ReactNode }) {
  return (
    <div className="max-w-5xl mx-auto px-6 py-12 flex gap-12 flex-1">
      <aside className="w-44 shrink-0">
        <nav className="flex flex-col gap-1 sticky top-8">
          {NAV.map((item) => (
            <Link
              key={item.href}
              href={item.href}
              className="text-sm text-zinc-600 dark:text-zinc-400 hover:text-zinc-900 dark:hover:text-zinc-100 py-1 transition-colors"
            >
              {item.label}
            </Link>
          ))}
        </nav>
      </aside>
      <article className="docs-content flex-1 min-w-0 max-w-none">
        {children}
      </article>
    </div>
  );
}
