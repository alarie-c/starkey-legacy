use std::{env, fs};

use lexer::Lexer;

mod token;
mod lexer;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() >= 2 {
        let path = &args[1];
        let src = fs::read_to_string(&path).expect("That file path does not exist");
        println!("Source file path: {}", &path);

        let mut lexer = Lexer::new(&src);
        let tokens = lexer.scan();
        dbg!(&tokens);

    } else {
        println!("Please specify a file path");
        std::process::exit(1);
    }
}