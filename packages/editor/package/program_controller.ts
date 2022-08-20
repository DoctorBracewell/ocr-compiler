import { print_parse } from "@ocr-compiler/compiler";

let i = 0;

function transpile(source: string) {
	// todo: transpilation with WASM
	// console.log(print_parse(source));
	console.log(source.length);

	return `window.output.set(${i++});`;
}

async function execute(program: string) {
	const scriptElement = document.createElement("script");
	scriptElement.setAttribute("type", "module");
	scriptElement.innerHTML = program;

	document.body.appendChild(scriptElement);
	document.body.removeChild(scriptElement);
}

export function run_program(source: string) {
	if (source.length === 0) return;

	const program = transpile(source);
	execute(program);
}
