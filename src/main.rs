use std::env;
use std::fs::File;
use std::io::Read;
use std::process;
use lexer::lexer::Lexer;

mod lexer;

fn main() {
    let args: Vec<String> = env::args().collect();

    // Determine file and CLI arguments
    if args.len() >= 2 {
        let mut file = File::open(&args[1]).expect("Error reading file!");
        let mut buffer = String::new();

        file.read_to_string(&mut buffer).expect("Error");

        let lexer = Lexer::new();
        let tokens = lexer.scan(buffer);

        if args.len() == 3 && &args[2] == "--tokens" {
            for _ in &tokens {
                // let msg = c.display();
                // println!("{msg}");
                println!("{:#?}", tokens);
            }
        }

        process::exit(0);
    } else {
        eprint!("\nSource code file path not specified!");
        eprint!("\nUsage: cargo run <path> <--flags>\n");
        
        process::exit(1);
    }
}
