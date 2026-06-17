import CopyButton from "./CopyButton";
import { highlight } from "../lib/highlight";

function escapeHtml(s: string): string {
  return s.replace(/&/g, "&amp;").replace(/</g, "&lt;").replace(/>/g, "&gt;");
}

export default function Pre({ children, lang = "cl" }: { children: string; lang?: string }) {
  const html = lang === "cl" ? highlight(children) : escapeHtml(children);
  return (
    <div className="pre-wrap">
      <CopyButton text={children} />
      <pre dangerouslySetInnerHTML={{ __html: `<code>${html}</code>` }} />
    </div>
  );
}
