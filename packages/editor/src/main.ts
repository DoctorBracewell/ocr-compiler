import "./app.css";
import Editor from "./components/Editor.svelte";

const app = new Editor({
	target: document.getElementById("app"),
});

export default app;
