<script lang="ts">
	//@ts-ignore
	import init, { print_parse } from "compiler";

	init().then(() => {
		console.log("init wasm-pack");
		console.log(print_parse("from"));
	});
</script>

<h1>Test</h1>
