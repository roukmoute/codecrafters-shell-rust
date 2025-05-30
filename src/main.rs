#[allow(unused_imports)]
use std::io::{self, Write};
use std::{env, process};
use std::path::Path;

fn main() {
    loop {
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
            println!("{}", input_trimmed.replace("echo ", ""))
        } else if input_trimmed.starts_with("type ") {
            let typed = input_trimmed.trim_start_matches("type ");

            let builtins = ["type", "echo", "exit"];

            if builtins.contains(&typed) {
                println!("{} is a shell builtin", typed)
            } else {
                is_executable_file(typed);
            }
        } else {
            eprintln!("{}: command not found", input_trimmed)
        }
    }
}

fn is_executable_file(typed: &str) {
    let mut valid = false;
    match env::var("PATH") {
        Ok(val) => {
            for path in env::split_paths(&val) {
                let binary_path = path.join(typed);
                if Path::new(&binary_path).exists() {
                    valid = true;
                    println!("{} is {}", typed, binary_path.display());
                    break;
                }
            }
        },
        Err(e) => eprintln!("{e}")
    }

    if valid == false {
        eprintln!("{}: not found", typed)
    }
}
