use std::{env, fs};

mod token;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() >= 2 {
        let path = &args[1];
        let src = fs::read_to_string(&path).expect("That file path does not exist");
        println!("Source file path: {}", &path);

    } else {
        println!("Please specify a file path");
        std::process::exit(1);
    }
}