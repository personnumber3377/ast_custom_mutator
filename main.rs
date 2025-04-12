use arbitrary::{Arbitrary, Unstructured};
use regex_syntax::ast::Ast;
use std::io::{self, Read};

fn main() {
    // Read bytes from stdin
    let mut buffer = Vec::new();
    io::stdin().read_to_end(&mut buffer).expect("Failed to read from stdin");

    // Convert bytes into an `Unstructured` instance
    let mut unstructured = Unstructured::new(&buffer);

    // Generate a random AST from the bytes
    match Ast::arbitrary(&mut unstructured) {
        Ok(ast) => {
            // Serialize AST to a string
            let ast_string = format!("{}", ast);
            println!("Generated AST: {}", ast_string);
        }
        Err(err) => {
            eprintln!("Failed to parse AST: {:?}", err);
        }
    }
}