#[allow(unused_imports)]
use std::io::{self, Write};
use std::process;

fn main() {
    loop  {
        // Uncomment this block to pass the first stage
        print!("$ ");
        io::stdout().flush().unwrap();

        // Wait for user input
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let input_trimmed = input.trim();
        if input_trimmed.eq("exit 0") {
            process::exit(0);
        } else if input_trimmed.starts_with("echo ") {
            println!("{}", input_trimmed.replace("echo ", ""));
        }
        else {
            eprintln!("{}: command not found", input_trimmed);
        }
    }
}
