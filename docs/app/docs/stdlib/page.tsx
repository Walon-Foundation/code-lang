import type { Metadata } from "next";
import Pre from "../../components/Pre";

export const metadata: Metadata = { title: "Standard library" };

const MODULES: {
  name: string;
  desc: string;
  fns: [string, string, string][];
}[] = [
  {
    name: "fmt",
    desc: "Output, input, and type conversion.",
    fns: [
      ["print(...args)", "Print args space-separated to stdout with a trailing newline.", "null"],
      ["eprint(...args)", "Same as print but writes to stderr.", "null"],
      ["input(prompt)", "Print prompt and read a line from stdin.", "string"],
      ["to_int(x)", "Convert string, float, or bool to integer.", "integer"],
      ["to_float(x)", "Convert string or integer to float.", "float"],
      ["to_str(x)", "Convert any value to its string representation.", "string"],
      ["format(template, ...args)", "Printf-style formatting: %s string, %d integer, %f float, %% literal percent.", "string"],
      ["clear()", "Clear the terminal screen.", "null"],
    ],
  },
  {
    name: "math",
    desc: "Mathematical functions and constants.",
    fns: [
      ["PI", "3.141592… (constant, not a function)", "float"],
      ["E", "2.718281… (constant, not a function)", "float"],
      ["sqrt(n)", "Square root of n.", "float"],
      ["abs(n)", "Absolute value — preserves input type.", "integer or float"],
      ["pow(base, exp)", "base raised to exp.", "float"],
      ["floor(n)", "Round down to nearest integer.", "integer"],
      ["ceil(n)", "Round up to nearest integer.", "integer"],
      ["round(n)", "Round to nearest integer.", "integer"],
      ["trunc(n)", "Truncate decimal part toward zero.", "integer"],
      ["log(n)", "Natural logarithm (ln).", "float"],
      ["log10(n)", "Base-10 logarithm.", "float"],
      ["log2(n)", "Base-2 logarithm.", "float"],
      ["exp(n)", "e raised to n.", "float"],
      ["sin(n) / cos(n) / tan(n)", "Trigonometric functions — input in radians.", "float"],
      ["min(a, b) / max(a, b)", "Minimum or maximum of two numbers.", "integer or float"],
      ["clamp(x, lo, hi)", "Clamp x to the range [lo, hi].", "integer or float"],
      ["sign(n)", "Returns -1, 0, or 1 based on the sign of n.", "integer"],
      ["gcd(a, b)", "Greatest common divisor.", "integer"],
      ["lcm(a, b)", "Least common multiple.", "integer"],
    ],
  },
  {
    name: "strings",
    desc: "String manipulation.",
    fns: [
      ["to_upper(s) / to_lower(s)", "Case conversion.", "string"],
      ["trim(s)", "Remove leading and trailing whitespace.", "string"],
      ["trim_left(s) / trim_right(s)", "Remove whitespace from one side only.", "string"],
      ["split(s, sep)", "Split s on sep.", "array"],
      ["join(arr, sep)", "Join array elements into a string with sep between each.", "string"],
      ["contains(s, sub)", "Whether s contains sub.", "bool"],
      ["starts_with(s, prefix) / ends_with(s, suffix)", "Prefix or suffix check.", "bool"],
      ["replace(s, old, new)", "Replace all occurrences of old with new.", "string"],
      ["index(s, sub)", "First index of sub in s, or -1 if not found.", "integer"],
      ["count(s, sub)", "Number of non-overlapping occurrences of sub in s.", "integer"],
      ["repeat(s, n)", "Repeat s n times.", "string"],
      ["reverse(s)", "Reverse the characters of s.", "string"],
      ["len(s)", "Number of characters in s.", "integer"],
      ["to_chars(s)", "Split s into individual characters.", "array"],
      ["from_chars(arr)", "Build a string from an array of characters.", "string"],
      ["parse_int(s)", "Parse s as an integer.", "integer"],
      ["parse_float(s)", "Parse s as a float.", "float"],
      ["lines(s)", "Split s by newline.", "array"],
      ["is_empty(s)", "Whether s has zero characters.", "bool"],
      ["pad_left(s, n, ch)", "Left-pad s to width n using character ch.", "string"],
      ["pad_right(s, n, ch)", "Right-pad s to width n using character ch.", "string"],
    ],
  },
  {
    name: "arrays",
    desc: "Array operations. All functions return new arrays — no mutation.",
    fns: [
      ["len(arr)", "Number of elements.", "integer"],
      ["first(arr) / last(arr)", "First or last element, or null if the array is empty.", "any or null"],
      ["rest(arr)", "New array without the first element.", "array"],
      ["pop(arr)", "New array without the last element.", "array"],
      ["push(arr, x)", "New array with x appended.", "array"],
      ["prepend(arr, x)", "New array with x prepended.", "array"],
      ["concat(a, b)", "Concatenate two arrays.", "array"],
      ["reverse(arr)", "Reversed copy of arr.", "array"],
      ["slice(arr, start, end)", "Subarray from start (inclusive) to end (exclusive).", "array"],
      ["contains(arr, x)", "Whether arr contains x.", "bool"],
      ["index_of(arr, x)", "First index of x, or -1 if not found.", "integer"],
      ["join(arr, sep)", "Join elements into a string with sep between each.", "string"],
      ["sum(arr)", "Sum of all numeric elements.", "integer or float"],
      ["min(arr) / max(arr)", "Minimum or maximum element of a numeric array.", "integer or float"],
      ["sort(arr)", "Sorted copy — works on numbers and strings.", "array"],
      ["unique(arr)", "Copy with duplicates removed, order preserved.", "array"],
      ["flatten(arr)", "Flatten one level of nesting.", "array"],
      ["zip(a, b)", "Array of [a[i], b[i]] pairs, stops at the shorter array.", "array"],
      ["map(arr, fn)", "New array of fn(element) results.", "array"],
      ["filter(arr, fn)", "Elements where fn(element) is truthy.", "array"],
      ["reduce(arr, fn, init)", "Accumulate fn(acc, element) left-to-right, starting from init.", "any"],
      ["find(arr, fn)", "First element where fn(element) is truthy, or null.", "any or null"],
      ["any(arr, fn)", "Whether at least one element passes fn.", "bool"],
      ["all(arr, fn)", "Whether every element passes fn.", "bool"],
    ],
  },
  {
    name: "hash",
    desc: "Hash (dictionary) operations.",
    fns: [
      ["keys(h)", "All keys.", "array"],
      ["values(h)", "All values.", "array"],
      ["entries(h)", "All [key, value] pairs.", "array"],
      ["has_key(h, k)", "Whether key k exists in h.", "bool"],
      ["get(h, k, default)", "Value for key k, or default if the key is absent.", "any"],
      ["len(h)", "Number of key-value pairs.", "integer"],
      ["merge(h1, h2)", "New hash with entries from both; h2 wins on key conflicts.", "hash"],
      ["delete(h, k)", "New hash without key k.", "hash"],
    ],
  },
  {
    name: "fs",
    desc: "File system I/O.",
    fns: [
      ["read_file(path)", "Read file contents as a string.", "string or error"],
      ["write_file(path, content)", "Write string to file, overwriting if it exists.", "bool"],
      ["append_file(path, content)", "Append string to file, creating it if missing.", "bool"],
      ["read_lines(path)", "Read file into an array of strings, one per line.", "array or error"],
      ["exists(path)", "Whether path exists on disk.", "bool"],
      ["is_file(path) / is_dir(path)", "Type check — file or directory.", "bool"],
      ["list_dir(path)", "Filenames inside a directory.", "array or error"],
      ["mkdir(path)", "Create a directory.", "bool"],
      ["mkdir_all(path)", "Create a directory and all missing parents.", "bool"],
      ["remove(path)", "Delete a file.", "bool"],
      ["remove_dir(path)", "Delete a directory and all its contents.", "bool"],
      ["copy(src, dst) / rename(src, dst)", "Copy or rename a file.", "bool"],
    ],
  },
  {
    name: "path",
    desc: "Path string manipulation — no filesystem access except absolute().",
    fns: [
      ["join(a, b, …)", "Join path segments with the OS separator.", "string"],
      ["basename(p)", "Filename with extension.", "string"],
      ["dirname(p)", "Parent directory.", "string"],
      ["stem(p)", "Filename without extension.", "string"],
      ["extension(p)", "Extension without the leading dot.", "string"],
      ["absolute(p)", "Canonicalized absolute path — hits the filesystem.", "string"],
      ["is_absolute(p)", "Whether path is absolute.", "bool"],
    ],
  },
  {
    name: "os",
    desc: "Operating system interface.",
    fns: [
      ["args", "Command-line arguments passed to the script (value, not a function).", "array"],
      ["platform", "OS name — e.g. \"linux\", \"macos\", \"windows\" (value, not a function).", "string"],
      ["arch", "CPU architecture string (value, not a function).", "string"],
      ["get_env(key)", "Read an environment variable. Returns empty string if unset.", "string"],
      ["set_env(key, val)", "Set an environment variable for the current process.", "null"],
      ["get_wd()", "Current working directory.", "string"],
      ["hostname()", "Machine hostname.", "string"],
      ["exit(code?)", "Exit the process. Optional integer exit code, defaults to 0.", "—"],
    ],
  },
  {
    name: "time",
    desc: "Date and time. All timestamps are unix milliseconds (integer).",
    fns: [
      ["now()", "Current time as unix milliseconds.", "integer"],
      ["unix()", "Current time as unix seconds.", "integer"],
      ["sleep(ms)", "Pause execution for ms milliseconds.", "null"],
      ["since(start_ms)", "Milliseconds elapsed since start_ms.", "integer"],
      ["format(ms, layout)", "Format a timestamp using a strftime-style layout string.", "string"],
      ["year(ms) / month(ms) / day(ms)", "Date components (UTC). month() is 1–12.", "integer"],
      ["hour(ms) / minute(ms) / second(ms)", "Time components (UTC).", "integer"],
      ["RFC3339", "\"%Y-%m-%dT%H:%M:%S%z\" layout constant (value, not a function).", "string"],
      ["Kitchen", "\"%I:%M %p\" layout constant (value, not a function).", "string"],
    ],
  },
  {
    name: "json",
    desc: "JSON serialisation.",
    fns: [
      ["parse(s)", "Parse a JSON string. JSON objects become hash, arrays become array.", "hash, array, string, integer, float, bool, or null"],
      ["stringify(x)", "Serialize a value to a JSON string.", "string"],
    ],
  },
  {
    name: "rand",
    desc: "Random number generation.",
    fns: [
      ["int(min, max)", "Random integer in [min, max] inclusive.", "integer"],
      ["float()", "Random float in [0.0, 1.0).", "float"],
      ["choice(arr)", "Random element from array, or null if the array is empty.", "any or null"],
      ["shuffle(arr)", "Shuffled copy of the array (Fisher-Yates).", "array"],
    ],
  },
  {
    name: "http",
    desc: "Blocking HTTP client.",
    fns: [
      ["get(url)", "HTTP GET. Returns a hash with status (integer), body (string), and ok (bool).", "hash"],
      ["get(url, headers)", "HTTP GET with custom headers (hash of string → string).", "hash"],
      ["post(url, body)", "HTTP POST with a plain string body.", "hash"],
      ["post(url, body, headers)", "HTTP POST with custom headers.", "hash"],
      ["post_json(url, obj)", "HTTP POST with JSON body — sets Content-Type automatically.", "hash"],
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
          <h2 id={mod.name}>
            <code>{mod.name}</code>
          </h2>
          <p>{mod.desc}</p>
          <div className="tbl"><table>
            <thead>
              <tr>
                <th>Function / value</th>
                <th>Description</th>
                <th>Returns</th>
              </tr>
            </thead>
            <tbody>
              {mod.fns.map(([sig, desc, ret]) => (
                <tr key={sig}>
                  <td><code>{sig}</code></td>
                  <td>{desc}</td>
                  <td><code>{ret}</code></td>
                </tr>
              ))}
            </tbody>
          </table></div>
        </section>
      ))}
    </>
  );
}
