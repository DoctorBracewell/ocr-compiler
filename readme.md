# OCR Reference Language Compiler

Hello! This is a monorepo containing packages and apps for an implementation of the [OCR GCSE Reference Language](https://www.ocr.org.uk/Images/558027-specification-gcse-computer-science-j277.pdf#page=27).

**The majority of work is currently taking place on the [rust-compiler package](https://github.com/DoctorBracewell/ocr-compiler/tree/parser/packages/rust-compiler) on the `parser` branch, as I work on writing the parsing expression grammar and tests to go along with it.**

The project is currently in **very early stages**, no READMEs, documentation or codebases are complete whatsoever, but the end goal is for something like below:

- A rust library that implements a transpilation pipeline of `Reference Language -> JavaScript` with WebAssembly bindings.
- A stand-alone svelte component that implements a full editor for the language, including syntax highlighting, an input-output system and a file system.
- A [website](https://brace.dev/ocr-compiler) which contains project information and links as well an instance of the editor component, available for use in classrooms or individually.

Feel free to check out the monorepo directories to see what state the different projects are in, or even to try contributing (though I can't promise about how active I'll be in accepting PRs and the such).

Please get in touch through any of the links on my [website](https://brace.dev/) if you have any questions or ideas!

~ Brace
