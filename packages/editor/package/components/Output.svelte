<script lang="ts">
	let output: string[] = [];

	window.output.subscribe(
		(value) => (output = value.length === 0 ? output : [...output, value])
	);

	$: console.log(output);
</script>

<div class="flex-1 flex flex-col">
	<h1 class="text-3xl underline underline-offset-2 mb-4">Output</h1>
	<textarea
		class="flex-1 rounded-md border-solid border-4 border-dark-400 bg-dark-500 p-5"
		>{output.join("\n")}
	</textarea>
</div>
