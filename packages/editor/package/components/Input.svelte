<script lang="ts">
	import CodeMirror, { EditorConfiguration, Editor } from "codemirror";
	import CodeMirrorComponent from "codemirror-svelte";
	import "codemirror/lib/codemirror.css";
	import "codemirror/theme/ayu-mirage.css";
	import "codemirror/mode/markdown/markdown";

	const editorOptions: EditorConfiguration = {
		mode: {
			name: "markdown",
			highlightFormatting: true,
		},
		theme: "ayu-mirage",
		lineNumbers: true,
	};

	// initial content
	let value = "";

	// event registration
	const editorOnChange = (e: { detail: Editor }) => {};
	const editorOnScroll = (e: { detail: Editor }) => {};
</script>

<div id="input" class="flex-1 flex flex-col">
	<h1 class="text-3xl underline underline-offset-2 mb-4">Input</h1>

	<CodeMirrorComponent
		{CodeMirror}
		{value}
		options={editorOptions}
		on:change={editorOnChange}
		on:scroll={editorOnScroll}
	/>
</div>
