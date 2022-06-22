<script lang="ts">
	//@ts-ignore
	import init, { print_parse } from "@ocr-compiler/compiler";

	init(import.meta.env.DEV ? "compiler_bg.wasm" : undefined).then(() => {
		console.log("init wasm-pack");
		console.log(print_parse("from"));
	});
</script>

<h1>Test</h1>
