use std::env;
use std::fs::File;
use std::io::Read;
use std::process;
//use error::error::{DarcyError, ErrorKind};
use lexer::lexer::Lexer;
use lexer::tokens::{Token, TokenKind};
use error::{error_kind::ErrorKind, darcy_error::DarcyError};

mod lexer;
mod error;
mod ast;
mod scope;

fn main() {
    let args: Vec<String> = env::args().collect();

    // Determine file and CLI arguments
    if args.len() >= 2 {
        // Get file and make buffer
        let mut file = File::open(&args[1]).expect("Error reading file!");
        let mut buffer = String::new();

        // Read file as string into buffer
        file.read_to_string(&mut buffer).expect("Error");

        // Get lines content
        let mut lines = Vec::<String>::new();
        let lines_iter = buffer.chars();

        // Iterate over each character, making new strings when \n found
        let mut buf = String::new();
        for c in lines_iter {
            if c == '\n' {
                lines.push(buf);
                buf = String::new();

            } else {
                buf.push(c);
            }
        }

        println!("{:#?}", lines);

        // Create lexer and iterator
        let iterator = buffer.chars().peekable();
        let mut lexer = Lexer::new(iterator, lines);


        // Scan iterator for tokens
        let (tokens, errors) = lexer.scan();

        // Clean tokens
        let mut tokenized: Vec<&Token> = Vec::new();
        for token in tokens {
            if token.kind != TokenKind::Empty {
                tokenized.push(token);
            }
        }

        // Print errors
        for err in errors {
            err.report();
        }

        // Match CLI args for flags
        if args.len() == 3 {
            match args[2].as_str() {
                "--debug" => {
                    println!("debugging...");
                    for token in tokenized {
                        println!("{:#?}", token);
                    }
                },
                "--source" => {
                    println!("{:?}", buffer);
                },
                _ => {},
            }
        }

        process::exit(0);
    } else {
        eprint!("\nSource code file path not specified!");
        eprint!("\nUsage: cargo run <path> <--flags>\n");
        
        process::exit(1);
    }
}
