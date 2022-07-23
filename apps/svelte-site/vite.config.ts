import { defineConfig } from "vite";
import { svelte } from "@sveltejs/vite-plugin-svelte";
import wasmPack from "vite-plugin-wasm-pack";

// https://vitejs.dev/config/
export default defineConfig({
	plugins: [wasmPack([], ["@ocr-compiler/compiler"]), svelte()],
	base: "./",
});
