<script>
	import { Editor } from "@ocr-compiler/svelte-editor";
</script>

<main class="w-full h-screen flex flex-col">
	<div class="flex flex-col m-auto mt-[20%] max-w-[70%]">
		<h1 class="mx-auto mb-7 text-5xl">Hello!</h1>
		<p>
			This site is a <b>work-in-progress</b>, and will eventually hold the documentation and sandbox
			for an implementation of the
			<a
				href="https://www.ocr.org.uk/Images/558027-specification-gcse-computer-science-j277.pdf#page=27"
				>OCR GCSE reference language</a
			>.
		</p>
		<p>
			Right now development on the parser and transpiler is happening with <b>rust</b> on the
			<a href="https://github.com/DoctorBracewell/ocr-compiler">Github</a> monorepo, feel free to pop
			in and take a look or even contribute!
		</p>
		<p>
			Work on this site will likely begin... when I get bored of doing compilery-stuff and want
			write some svelte code, so keep an eye out!
		</p>
		<p>~ Brace</p>
	</div>

	<Editor />
</main>
