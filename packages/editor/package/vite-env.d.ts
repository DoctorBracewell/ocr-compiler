/// <reference types="svelte" />
/// <reference types="vite/client" />

import type { Writable } from "svelte/store";
declare global {
	interface Window {
		files: Writable<number>;
	}
}
