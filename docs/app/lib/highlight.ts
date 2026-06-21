const KEYWORDS = new Set([
  "let", "const", "fn", "return", "if", "elseif", "else",
  "while", "for", "break", "continue", "import", "struct",
  "true", "false", "null", "typeof", "switch", "enum",
  "pub", "in", "self",
]);

function esc(s: string): string {
  return s.replace(/&/g, "&amp;").replace(/</g, "&lt;").replace(/>/g, "&gt;");
}

export function highlight(code: string): string {
  const out: string[] = [];
  let i = 0;

  while (i < code.length) {
    const ch = code[i];

    // Single-line comment: # ...
    if (ch === "#") {
      const end = code.indexOf("\n", i);
      const slice = end === -1 ? code.slice(i) : code.slice(i, end);
      out.push(`<span class="hl-c">${esc(slice)}</span>`);
      i += slice.length;
      continue;
    }

    // Multi-line comment: /* ... */
    if (ch === "/" && code[i + 1] === "*") {
      const end = code.indexOf("*/", i + 2);
      const slice = end === -1 ? code.slice(i) : code.slice(i, end + 2);
      out.push(`<span class="hl-c">${esc(slice)}</span>`);
      i += slice.length;
      continue;
    }

    // String: "..."
    if (ch === '"') {
      let j = i + 1;
      while (j < code.length && code[j] !== '"') {
        if (code[j] === "\\") j++;
        j++;
      }
      const slice = code.slice(i, j + 1);
      out.push(`<span class="hl-s">${esc(slice)}</span>`);
      i = j + 1;
      continue;
    }

    // Char: '.'
    if (ch === "'") {
      let j = i + 1;
      if (j < code.length && code[j] === "\\") j++;
      if (j < code.length) j++;
      if (j < code.length && code[j] === "'") j++;
      const slice = code.slice(i, j);
      out.push(`<span class="hl-s">${esc(slice)}</span>`);
      i = j;
      continue;
    }

    // Number (integer or float)
    if (ch >= "0" && ch <= "9") {
      let j = i;
      while (j < code.length && (code[j] >= "0" && code[j] <= "9" || code[j] === ".")) j++;
      out.push(`<span class="hl-n">${esc(code.slice(i, j))}</span>`);
      i = j;
      continue;
    }

    // Identifier / keyword / function call
    if ((ch >= "a" && ch <= "z") || (ch >= "A" && ch <= "Z") || ch === "_") {
      let j = i;
      while (j < code.length && /[a-zA-Z0-9_]/.test(code[j])) j++;
      const word = code.slice(i, j);
      if (KEYWORDS.has(word)) {
        out.push(`<span class="hl-k">${esc(word)}</span>`);
      } else if (j < code.length && code[j] === "(") {
        out.push(`<span class="hl-f">${esc(word)}</span>`);
      } else {
        out.push(esc(word));
      }
      i = j;
      continue;
    }

    out.push(esc(ch));
    i++;
  }

  return out.join("");
}
