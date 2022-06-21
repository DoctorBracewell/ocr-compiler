import { nextToken, Token } from "./tokenisers.js";

// Entry function
export function lex(data: string): Token[] {
  // Sanitise input by trimming end whitespace
  const sanitised = data
    .split("\n")
    .map((line) => line.trimEnd())
    .map((line) =>
      line.substring(
        0,
        line.indexOf("//") === -1 ? line.length : line.indexOf("//")
      )
    )
    .join("\n");

  let tokens: Token[] = [];
  let program = sanitised;

  while (program.length > 0) {
    // Get next token
    const { index, token } = nextToken(program);

    // Push token to token list and remove token from the program string
    tokens.push(token);
    program = program.slice(index);
  }

  return tokens;
}
