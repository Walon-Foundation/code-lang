import type { Metadata } from "next";
import Pre from "../../components/Pre";

export const metadata: Metadata = { title: "Standard library" };

const MODULES = [
  {
    name: "fmt",
    desc: "Output, input, and type conversion.",
    fns: [
      ["print(...args)", "Print args space-separated to stdout with newline."],
      ["eprint(...args)", "Same as print but to stderr."],
      ["input(prompt)", "Print prompt and read a line from stdin. Returns STRING."],
      ["typeof(x)", "Return the type name of x as a STRING."],
      ["to_int(x)", "Convert STRING, FLOAT, or BOOL to INTEGER."],
      ["to_float(x)", "Convert STRING or INTEGER to FLOAT."],
      ["to_str(x)", "Convert any value to its STRING representation."],
      ["clear()", "Clear the terminal screen."],
    ],
  },
  {
    name: "math",
    desc: "Mathematical functions and constants.",
    fns: [
      ["PI", "3.141592… (constant)"],
      ["E", "2.718281… (constant)"],
      ["sqrt(n)", "Square root."],
      ["abs(n)", "Absolute value. Returns same type as input."],
      ["pow(base, exp)", "base raised to exp. Returns FLOAT."],
      ["floor(n) / ceil(n) / round(n) / trunc(n)", "Rounding variants."],
      ["log(n)", "Natural logarithm (ln)."],
      ["log10(n)", "Base-10 logarithm."],
      ["exp(n)", "e raised to n."],
      ["sin(n) / cos(n) / tan(n)", "Trigonometric functions (radians)."],
      ["min(a, b, …) / max(a, b, …)", "Minimum / maximum of one or more numbers."],
      ["clamp(x, lo, hi)", "Clamp x to the range [lo, hi]."],
    ],
  },
  {
    name: "strings",
    desc: "String manipulation.",
    fns: [
      ["to_upper(s) / to_lower(s)", "Case conversion."],
      ["trim(s) / trim_left(s) / trim_right(s)", "Remove whitespace."],
      ["split(s, sep)", "Split string into ARRAY of strings."],
      ["join(arr, sep)", "Join array into a string."],
      ["contains(s, sub)", "BOOL — whether s contains sub."],
      ["starts_with(s, prefix) / ends_with(s, suffix)", "BOOL prefix/suffix check."],
      ["replace(s, old, new)", "Replace all occurrences of old with new."],
      ["index(s, sub)", "First index of sub in s, or -1."],
      ["count(s, sub)", "Number of non-overlapping occurrences of sub."],
      ["repeat(s, n)", "Repeat s n times."],
      ["reverse(s)", "Reverse the string."],
      ["to_chars(s)", "ARRAY of CHAR values."],
      ["from_chars(arr)", "Build a STRING from an array of CHARs."],
      ["parse_int(s) / parse_float(s)", "Parse string to number."],
    ],
  },
  {
    name: "arrays",
    desc: "Array operations. All functions return new arrays — no mutation.",
    fns: [
      ["len(arr)", "Length of array (also works on STRING)."],
      ["first(arr) / last(arr)", "First / last element, or null if empty."],
      ["rest(arr)", "Array without the first element."],
      ["pop(arr)", "Array without the last element."],
      ["push(arr, x) / prepend(arr, x)", "Return new array with x appended / prepended."],
      ["concat(a, b)", "Concatenate two arrays."],
      ["reverse(arr)", "Return reversed array."],
      ["slice(arr, start, end)", "Subarray from start (inclusive) to end (exclusive)."],
      ["contains(arr, x)", "BOOL — whether arr contains x."],
      ["index_of(arr, x)", "First index of x, or -1."],
      ["join(arr, sep)", "Join elements into a STRING."],
      ["sum(arr)", "Sum of numeric elements."],
      ["min(arr) / max(arr)", "Min / max of numeric array."],
      ["sort(arr)", "Sorted copy (numbers and strings)."],
      ["unique(arr)", "Remove duplicates, preserve order."],
      ["flatten(arr)", "Flatten one level of nesting."],
      ["zip(a, b)", "Array of [a[i], b[i]] pairs, stops at shorter."],
    ],
  },
  {
    name: "hash",
    desc: "Hash (dictionary) operations.",
    fns: [
      ["keys(h)", "ARRAY of keys."],
      ["values(h)", "ARRAY of values."],
      ["entries(h)", "ARRAY of [key, value] pairs."],
      ["has_key(h, k)", "BOOL — whether key k exists."],
      ["len(h)", "Number of key-value pairs."],
      ["merge(h1, h2)", "New hash with both; h2 overwrites h1 on conflicts."],
      ["delete(h, k)", "New hash without key k."],
    ],
  },
  {
    name: "fs",
    desc: "File system I/O.",
    fns: [
      ["read_file(path)", "Read file contents as STRING."],
      ["write_file(path, content)", "Write STRING to file (overwrite). Returns BOOL."],
      ["append_file(path, content)", "Append STRING to file (creates if missing). Returns BOOL."],
      ["read_lines(path)", "Read file into ARRAY of strings (one per line)."],
      ["exists(path)", "BOOL — path exists."],
      ["is_file(path) / is_dir(path)", "BOOL — type check."],
      ["list_dir(path)", "ARRAY of filenames in directory."],
      ["mkdir(path) / mkdir_all(path)", "Create directory / all intermediate directories."],
      ["remove(path)", "Delete a file."],
      ["remove_dir(path)", "Delete a directory and all its contents."],
      ["copy(src, dst) / rename(src, dst)", "Copy or rename a file."],
    ],
  },
  {
    name: "path",
    desc: "Path string manipulation — no filesystem access except absolute().",
    fns: [
      ["join(a, b, …)", "Join path segments with the OS separator."],
      ["basename(p)", "Filename with extension."],
      ["dirname(p)", "Parent directory."],
      ["stem(p)", "Filename without extension."],
      ["extension(p)", "Extension without the leading dot."],
      ["absolute(p)", "Canonicalized absolute path (hits filesystem)."],
      ["is_absolute(p)", "BOOL — whether path is absolute."],
    ],
  },
  {
    name: "os",
    desc: "Operating system interface.",
    fns: [
      ["args", "ARRAY of command-line arguments (value, not function)."],
      ["platform", "OS name string, e.g. \"linux\", \"macos\", \"windows\" (value)."],
      ["arch", "CPU architecture string (value)."],
      ["get_env(key)", "Read an environment variable. Returns STRING (empty if unset)."],
      ["set_env(key, val)", "Set an environment variable."],
      ["get_wd()", "Current working directory as STRING."],
      ["hostname()", "Machine hostname as STRING."],
      ["exit(code?)", "Exit the process with optional integer code (default 0)."],
    ],
  },
  {
    name: "time",
    desc: "Date and time. Timestamps are unix milliseconds (INTEGER).",
    fns: [
      ["now()", "Current time as unix milliseconds."],
      ["unix()", "Current time as unix seconds."],
      ["sleep(ms)", "Pause execution for ms milliseconds."],
      ["since(start_ms)", "Milliseconds elapsed since start_ms."],
      ["format(ms, layout)", "Format timestamp using a strftime-style layout string."],
      ["year(ms) / month(ms) / day(ms)", "Date components (UTC). Month is 1–12."],
      ["hour(ms) / minute(ms) / second(ms)", "Time components (UTC)."],
      ["RFC3339", "\"%Y-%m-%dT%H:%M:%S%z\" layout constant."],
      ["Kitchen", "\"%I:%M %p\" layout constant."],
    ],
  },
  {
    name: "json",
    desc: "JSON serialisation.",
    fns: [
      ["parse(s)", "Parse a JSON string into code-lang values. JSON objects become HASH."],
      ["stringify(x)", "Serialize a value to a JSON STRING."],
    ],
  },
  {
    name: "rand",
    desc: "Random number generation.",
    fns: [
      ["int(min, max)", "Random INTEGER in [min, max] inclusive."],
      ["float()", "Random FLOAT in [0.0, 1.0)."],
      ["choice(arr)", "Random element from array."],
      ["shuffle(arr)", "Return a new shuffled copy of the array (Fisher-Yates)."],
    ],
  },
  {
    name: "http",
    desc: "Blocking HTTP client. All functions return a HASH with status (INTEGER), body (STRING), and ok (BOOL).",
    fns: [
      ["get(url)", "HTTP GET."],
      ["get(url, headers)", "HTTP GET with custom headers (HASH of STRING→STRING)."],
      ["post(url, body)", "HTTP POST with a plain string body."],
      ["post(url, body, headers)", "HTTP POST with custom headers."],
      ["post_json(url, obj)", "HTTP POST with JSON body (sets Content-Type automatically)."],
    ],
  },
];

export default function StdlibReference() {
  return (
    <>
      <h1>Standard library</h1>
      <p>
        All stdlib modules are preloaded — no installation needed. Import a module by name and use dot notation to call its functions.
      </p>
      <Pre>{`import "math";
import "strings";

math.sqrt(9);               # 3.0
strings.to_upper("hello");  # HELLO`}</Pre>

      {MODULES.map((mod) => (
        <section key={mod.name}>
          <h2>
            <code>{mod.name}</code>
          </h2>
          <p>{mod.desc}</p>
          <div className="tbl"><table>
            <thead>
              <tr>
                <th>Function / value</th>
                <th>Description</th>
              </tr>
            </thead>
            <tbody>
              {mod.fns.map(([sig, desc]) => (
                <tr key={sig}>
                  <td><code>{sig}</code></td>
                  <td>{desc}</td>
                </tr>
              ))}
            </tbody>
          </table></div>
        </section>
      ))}
    </>
  );
}
