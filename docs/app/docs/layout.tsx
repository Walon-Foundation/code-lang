import DocsSidebar from "../components/DocsSidebar";
import TableOfContents from "../components/TableOfContents";

export default function DocsLayout({ children }: { children: React.ReactNode }) {
  return (
    <div className="docs-layout">
      <DocsSidebar />
      <article className="docs-content docs-article">
        {children}
      </article>
      <TableOfContents />
    </div>
  );
}
