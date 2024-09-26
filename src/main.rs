use std::fs;
use std::env;
use std::io;
use std::io::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut pipe = String::new();
    let help = "--help";

    if args[1] == help {
        println!("\nUsage: rat [FILE]\n");
        println!("    --help    prints this help page");
        println!("\nExample: rat example.txt");
        println!("Example: ls | rat\n");
    }

    else    if args.len() >= 2 {
        let file = &args[1];
        let content = fs::read_to_string(file)
            .expect("Failed to read the file");
        println!("\n{}", content);
    }
    else if let Ok(_) = io::stdin().read_to_string(&mut pipe) {
        println!("{}",pipe)
    }

}
