<script lang="ts">
	import "../styles/app.css";

	import Input from "./Input.svelte";
	import Output from "./Output.svelte";
	import FileManager from "./FileManager/FileManager.svelte";

	import init from "@ocr-compiler/compiler";
</script>

<div class="w-full flex-1 flex text-neutral-100">
	{#await init(import.meta.env.DEV ? "compiler_bg.wasm" : undefined) then}
		<div
			class="flex-1 m-2 grid grid-cols-12 grid-rows-2 md:grid-rows-1 bg-dark-200 gap-[8px] border-solid border-[8px] border-dark-200 rounded-md"
		>
			<div class="flex col-span-4 row-span-2 md:col-span-3 p-5 bg-dark-400">
				<FileManager />
			</div>
			<div
				class="flex col-span-8 row-span-1 md:col-span-5 md:row-span-2 p-5 bg-dark-400"
			>
				<Input />
			</div>
			<div
				class="flex col-span-8 row-span-1 md:col-span-4 md:row-span-2 p-5 bg-dark-400"
			>
				<Output />
			</div>
		</div>
	{/await}
</div>
