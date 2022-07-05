import Editor from "@ocr-compiler/editor";
import "@ocr-compiler/editor/app.css";

const app = new Editor({
	target: document.getElementById("app"),
});

export default app;
