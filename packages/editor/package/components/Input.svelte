<script lang="ts">
	import { print_parse } from "@ocr-compiler/compiler";

	import CodeMirror, { EditorConfiguration, Editor } from "codemirror";
	import CodeMirrorComponent from "codemirror-svelte";
	import "codemirror/lib/codemirror.css";
	import "codemirror/theme/ayu-mirage.css";

	const editorOptions: EditorConfiguration = {
		theme: "ayu-mirage",
		lineNumbers: true,
	};

	// initial content
	let value = "";

	$: console.log(print_parse(value));

	// event registration
	const editorOnChange = (e: { detail: Editor }) => {
		value = e.detail.getValue();
	};
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
