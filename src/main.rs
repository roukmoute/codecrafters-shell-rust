#[allow(unused_imports)]
use std::io::{self, Write};
use std::{env, fs, process};
use std::path::{Path, PathBuf};
use std::os::unix::fs::PermissionsExt;
use std::process::Command;

fn main() {
    loop {
        // Uncomment this block to pass the first stage
        print!("$ ");
        io::stdout().flush().unwrap();

        // Wait for user input
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();


        let input_trimmed = input.trim();
        let argv: Vec<&str> = input_trimmed.split_whitespace().collect();
        let program_name = argv.first().copied();
        match argv.as_slice() {
            ["exit", "0"] => process::exit(0),
            ["echo", args @ ..] => println!("{}", args.join(" ")),
            ["type", args @ ..] => {
                let builtins = ["type", "echo", "exit"];
                if let Some(arg) = args.first() {
                    if builtins.contains(&arg) {
                        println!("{} is a shell builtin", arg);
                    } else {
                        print_executable_path(arg);
                    }
                }
            }
            _ => {
                if let Some(name) = program_name {
                    if let Some(path) = find_in_path(name, |p| p.exists()) {
                        if is_executable(path) {
                            let output = Command::new(name)
                                .args(&argv[1..])
                                .output();

                            match output {
                                Ok(result) => print!("{}", String::from_utf8_lossy(&result.stdout)),
                                Err(_) => eprintln!("Execution error"),
                            }
                        }
                    } else {
                        eprintln!("{input_trimmed}: command not found")
                    }
                }
            }
        }
    }
}

fn print_executable_path<P: AsRef<Path>>(file_path: P) {
    let filename = file_path.as_ref();

    match find_in_path(filename, |p| p.exists() && is_executable(p)) {
        Some(path) => println!("{} is {}", filename.display(), path.display()),
        None => eprintln!("{}: not found", filename.display())
    }
}

fn find_in_path<P, F>(filename: P, predicate: F) -> Option<PathBuf>
where
    P: AsRef<Path>,
    F: Fn(&PathBuf) -> bool
{
    if let Some(path_var) = env::var_os("PATH") {
        for path in env::split_paths(&path_var) {
            let binary_path = path.join(filename.as_ref());
            if predicate(&binary_path) {
                return Some(binary_path);
            }
        }
    }
    None
}

fn is_executable<P: AsRef<Path>>(path: P) -> bool {
    if let Ok(metadata) = fs::metadata(path) {
        let mode = metadata.permissions().mode();

        mode & 0o111 != 0
    } else {
        eprintln!("Oopsie");
        false
    }
}