use std::{env, fs, io};

mod lexer;
mod token;

fn main() -> Result<(), io::Error> {
    let args: Vec<String> = env::args().collect();
    let dbga = cfg!(debug_assertions);
    let path: &str;

    // Get path from args
    // TODO: Debug commands
    if args.len() >= 2 && dbga {
        path = &args[1];
    } else if args.len() >= 1 && !dbga {
        path = &args[0];
    } else {
        eprintln!("Specify a file path");
        std::process::exit(1);
    }

    let src = fs::read_to_string(&path)?;
    println!("Source file path: {path}");

    let mut lexer = lexer::Lexer::new(src.as_str());
    let tokens = lexer.scan();
    dbg!(&tokens);

    Ok(())
}
