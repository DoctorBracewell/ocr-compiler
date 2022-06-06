import { readFile } from "fs/promises";
import { lex } from "./lexer/index.js";

const file = await readFile("test/index.ocr", "utf-8");
console.log(lex(file));
