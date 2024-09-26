use std::fs;
use std::env;
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let file = &args[1];
        let content = fs::read_to_string(file)
            .expect("Failed to read the file");
        println!("\n{}", content);
        println!(": End of file\n")
    }
    else {
        println!("\nUsage: rat [FILE]");
        println!("Example: rat example.txt\n")
    }
}