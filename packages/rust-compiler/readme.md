# Transpiler

This package will be a rust library with WASM bindings that will accept a program as a string and run it through a pipeline to produce JavaScript that matches the code.

Desired features:

- Full transpilation pipeline including:
  - Parser, currently being built with pest.rs
  - Syntax error messages
  - Transpiler
- Access to each module seperately
