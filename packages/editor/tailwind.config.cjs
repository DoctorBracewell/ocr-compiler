const config = {
	content: [
		"./index.html",
		"./src/**/*.{html,js,svelte,ts}",
		"./package/**/*.{html,js,svelte,ts}",
	],
	presets: [
		require("./package/tailwind.config.cjs"),
		require("@ocr-compiler/tailwind-base"),
	],
};

module.exports = config;
