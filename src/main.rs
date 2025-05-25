// This file is the entry point of the compiler. It initializes the compiler components, such as the lexer, parser, and code generator, and orchestrates the compilation process.

mod lexer;
mod parser;
mod ast;
mod codegen;
mod utils;

fn main() {
    // Initialize the lexer, parser, and code generator
    let input = "1 + (2 * 3)".to_string();
    let mut lexer = lexer::Lexer::new(input);
    let tokens = match lexer.next_token() {
        Some(value) => value,
        None => {
            println!("No tokens found");
            return;
        }
    };

    let parser = parser::Parser::new(tokens);
    let ast = match parser.parse() {
        Ok(ast) => ast, // Successfully parsed AST
        Err(err) => {
            println!("Parsing error: {}", err);
            return;
        }
    }; // Parse tokens into an AST

    let codegen = codegen::Codegen::new();
    let target_code = codegen.generate_code(&ast); // Generate target code

    // Output the generated code
    println!("{}", target_code);
}