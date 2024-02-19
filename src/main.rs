use std::env;
use std::fs::File;
use std::io::Read;
use std::process;


fn main() {
    let args: Vec<String> = env::args().collect();

    // Determine file and CLI arguments
    if args.len() == 2 {
        let mut file = File::open(&args[1]).expect("Error reading file!");
        
        process::exit(0);
    } else {
        eprint!("\nSource code file path not specified!");
        eprint!("\nUsage: cargo run <path>\n");
        
        process::exit(1);
    }
}
