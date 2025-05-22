# Rust Compiler

This project is a simple Rust-based compiler that includes a lexer, parser, abstract syntax tree (AST), and code generation components. The goal of this compiler is to demonstrate the fundamental concepts of compiler design and implementation.

## Project Structure

- `src/main.rs`: Entry point of the compiler that initializes and orchestrates the compilation process.
- `src/lexer/mod.rs`: Defines the lexer module responsible for tokenizing the input source code.
- `src/parser/mod.rs`: Defines the parser module that parses tokens into an abstract syntax tree (AST).
- `src/ast/mod.rs`: Contains the definitions of various AST nodes and methods for traversing and manipulating the AST.
- `src/codegen/mod.rs`: Responsible for generating target code from the AST.
- `src/utils/mod.rs`: Contains utility functions and types used throughout the compiler.

## Building the Project

To build the project, ensure you have Rust and Cargo installed. Then, navigate to the project directory and run:

```
cargo build
```

## Running the Compiler

To run the compiler, use the following command:

```
cargo run <source_file>
```

Replace `<source_file>` with the path to the source code file you want to compile.

## Usage Instructions

1. Create a source code file with the desired code.
2. Use the `cargo run` command to compile the source code.
3. The compiler will output the generated code or any errors encountered during the compilation process.

## Contributing

Contributions are welcome! Please feel free to submit a pull request or open an issue for any suggestions or improvements.