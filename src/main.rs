use std::{env, fs};

use lexer::Lexer;
use parser::Parser;

mod ast;
mod lexer;
mod parser;
mod token;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() >= 2 {
        // Get file path
        let path = &args[1];
        let src = fs::read_to_string(&path).expect("That file path does not exist");
        println!("Source file path: {}", &path);

        // Tokenize
        let mut lexer = Lexer::new(&src);
        let tokens = lexer.scan();
        dbg!(&tokens);

        // Parse + AstGen
        let mut parser = Parser::new(&tokens);
        parser.parse();
        dbg!(&parser.ast);
    } else {
        // File path was likely not specified
        println!("Please specify a file path");
        std::process::exit(1);
    }
}
