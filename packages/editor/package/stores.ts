import type { Writable } from "svelte/store";

declare global {
	interface Window {
		output: Writable<string>;
	}
}

import { writable } from "svelte/store";
window.output = writable("");
