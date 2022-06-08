/** This module contains a lot of abstracted logic to represent the tokens for lexing. */

/** An enum representation to represent the different types of tokens.
 *
 * From:  https://maxheiber.medium.com/alternatives-to-typescript-enums-50e4c16600b1
 */
export const TokenTypes = {
  AssignmentOperator: "assignment_operator",
  ComparisonOperator: "comparison_operator",
  MathematicalOperator: "maths_operator",
  LogicalOperator: "logical_operator",
  GlobalFunction: "global_function",
  BooleanLiteral: "boolean_literal",
  SquareBrackets: "square_brackets",
  RoundBrackets: "round_brackets",
  StringLiteral: "string_literal",
  NumberLiteral: "number_literal",
  DotOperator: "dot_operator",
  Whitespace: "whitespace",
  Identifier: "identifier",
  Operator: "operator",
  NewLine: "new_line",
  Keyword: "keyword",
  Comma: "comma",
} as const;

export type TokenType = typeof TokenTypes[keyof typeof TokenTypes];

/**
 * A representation of how to lex a specific token
 */
interface TokenData {
  /** The type of token */
  type: TokenType;
  /** A function that determines whether or not to continue taking characters when lexing the token */
  keepTaking: (char: string, acc: string) => boolean[];
  /** A function that determines whether or not to accept the token once characters have stopped being taken */
  isInvalid?: (acc: string) => boolean[];
}

/** === Builder keepTaking functions === */

/** A pre-built function that can take the place of a custom keepTaking functionm that continues taking any letter characters*/
const takeIfLetter = (char: string) => [/[a-z]/i.test(char)];
const arrayIntoRegex = (strArray: string[]) => (char: string) =>
  [
    new RegExp(
      `[${strArray
        .map((match) => (/[a-z]/i.test(match) ? match : `\\${match}`))
        .join("")}]`
    ).test(char),
  ];

/** === Builder isInvalid functions === */

/** A function that builds another function that can take the place of a custom isInvalid function, that returns inValid if the token value is not included in a provided array*/
const validIfIncluded = (strArray: string[]) => (acc: string) =>
  [!strArray.includes(acc)];

/**
 * A constant that contains all the token data
 */
export const tokenData: TokenData[] = [
  {
    type: TokenTypes.MathematicalOperator,
    keepTaking: arrayIntoRegex(["+", "-", "*", "/", "^", " MOD ", " DIV "]),
    isInvalid: validIfIncluded(["+", "-", "*", "/", "^", " MOD ", " DIV"]),
  },
  {
    type: TokenTypes.ComparisonOperator,
    keepTaking: arrayIntoRegex(["==", "!=", "<", ">", "<=", ">="]),
    isInvalid: validIfIncluded(["==", "!=", "<", ">", "<=", ">="]),
  },
  {
    type: TokenTypes.NewLine,
    keepTaking: (char) => [/\n/.test(char)],
  },
  {
    type: TokenTypes.Whitespace,
    keepTaking: (char) => [/\s/.test(char)],
  },
  {
    type: TokenTypes.RoundBrackets,
    keepTaking: (_, acc) => [["(", ")"].includes(acc)],
    isInvalid: validIfIncluded(["(", ")"]),
  },
  {
    type: TokenTypes.SquareBrackets,
    keepTaking: (_, acc) => [["[", "]"].includes(acc)],
    isInvalid: validIfIncluded(["[", "]"]),
  },
  {
    type: TokenTypes.DotOperator,
    keepTaking: (char) => [char === "."],
  },
  {
    type: TokenTypes.AssignmentOperator,
    keepTaking: (char) => [char === "="],
  },
  {
    type: TokenTypes.Comma,
    keepTaking: (char) => [char === ","],
  },
  {
    type: TokenTypes.LogicalOperator,
    keepTaking: takeIfLetter,
    isInvalid: validIfIncluded(["AND", "OR", "NOT"]),
  },
  {
    type: TokenTypes.Keyword,
    keepTaking: takeIfLetter,
    isInvalid: validIfIncluded([
      "while",
      "endwhile",
      "do",
      "until",
      "if",
      "then",
      "elseif",
      "endif",
      "const",
      "global",
      "array",
      "for",
      "to",
      "next",
      "step",
      "switch",
      "case",
      "default",
      "endswitch",
      "procedure",
      "endprocedure",
      "function",
      "endfunction",
      "return",
    ]),
  },
  {
    type: TokenTypes.BooleanLiteral,
    keepTaking: takeIfLetter,
    isInvalid: validIfIncluded(["true", "false"]),
  },
  {
    type: TokenTypes.GlobalFunction,
    keepTaking: takeIfLetter,
    isInvalid: validIfIncluded([
      "input",
      "print",
      "ASC",
      "CHR",
      "open",
      "newFile",
      "random",
      "str",
      "int",
      "float",
      "real",
      "bool",
    ]),
  },
  {
    type: TokenTypes.StringLiteral,
    keepTaking: (_, acc) => [acc === '"', acc.slice(1).slice(-2, -1) !== '"'],
    isInvalid: (acc) => [!(acc.startsWith('"') && acc.endsWith('"'))],
  },
  {
    type: TokenTypes.MathematicalOperator,
    keepTaking: (_, acc) => [acc === "'", acc.slice(1).slice(-2, -1) !== "'"],
    isInvalid: (acc) => [!(acc.startsWith("'") && acc.endsWith("'"))],
  },
  {
    type: TokenTypes.NumberLiteral,
    keepTaking: (char) => [/[0-9\.]/.test(char)],
  },
  {
    type: TokenTypes.Identifier,
    keepTaking: (char) => [/[a-z0-9]/i.test(char)],
  },
];
