<script lang="ts">
	import { addFile } from "../files.js";

	console.log(addFile);

	let value = "test";

	$: console.log(value);

	try {
		eval('addFile("test")');
	} catch (e) {
		console.log(e);
	}
</script>

<div class="flex-1 flex flex-col">
	<h1 class="text-3xl underline underline-offset-2 mb-4">Output</h1>
	<textarea class="flex-1 rounded-md border-solid border-4 border-dark-400" />
</div>
