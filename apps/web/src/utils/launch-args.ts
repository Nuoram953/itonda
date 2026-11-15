/**
 * Split a command-line argument string into an array of arguments,
 * preserving quoted segments (single or double quotes).
 */
export function parseArgsString(input: string): string[] {
  const args: string[] = [];
  let current = "";
  let quote: '"' | "'" | null = null;

  for (let i = 0; i < input.length; i++) {
    const ch = input[i];
    if (quote) {
      if (ch === quote) {
        quote = null;
      } else {
        current += ch;
      }
    } else if (ch === '"' || ch === "'") {
      quote = ch;
    } else if (/\s/.test(ch)) {
      if (current.length > 0) {
        args.push(current);
        current = "";
      }
    } else {
      current += ch;
    }
  }

  if (current.length > 0) {
    args.push(current);
  }

  return args;
}

/**
 * Format an array of arguments into a single string, quoting arguments
 * that contain whitespace.
 */
export function formatArgsString(args: string[]): string {
  return args
    .map((arg) => (/\s/.test(arg) ? `"${arg}"` : arg))
    .join(" ");
}
