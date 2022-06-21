import { tokenData, TokenType } from "./tokenData.js";

export class Token {
  constructor(public tokenType: TokenType, public value: string) {
    this.tokenType = tokenType;
    this.value = value;
  }
}

export function nextToken(data: string) {
  for (const { type, keepTaking, isInvalid } of tokenData) {
    let acc = "";

    const { token, index } = takeWhile(data, (char) => {
      const keepTakingChars = keepTaking(char, acc + char);

      if (keepTakingChars.some(Boolean)) {
        acc += char;
        return true;
      }

      return false;
    });

    if (acc) console.log(type, acc);

    if (index === 0) continue;
    if (isInvalid ? isInvalid(acc).some(Boolean) : false) continue;

    return {
      token: new Token(type, token),
      index,
    };
  }

  throw new Error("Oops!");
}

function takeWhile(data: string, predicate: (char: string) => boolean) {
  let index = 0;

  for (const character of data) {
    if (!predicate(character)) break;

    index++;
  }

  return {
    index,
    token: data.slice(0, index),
  };
}
