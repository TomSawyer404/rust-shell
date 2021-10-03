use std::env;
use std::io::*;
use std::path::Path;
use std::process::Command;

const RED: &str = "\x1b[31m";
const RESET: &str = "\x1b[0m";

fn main() {
    loop {
        print!("{}rshell> {}", RED, RESET);
        stdout().flush().unwrap();

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        // everything after the first whitespace character
        // is interpreted as args to the command
        let mut parts = input.trim().split_whitespace();
        let command = parts.next().unwrap();
        let args = parts;

        match command {
            "cd" => {
                // default to `/` as new directory if one was not provided
                let new_dir = args.peekable().peek().map_or("/", |x| *x);
                let root = Path::new(new_dir);
                if let Err(e) = env::set_current_dir(&root) {
                    eprintln!("{}", e);
                }
            }
            "exit" => return,
            _ => {
                let child = Command::new(command).args(args).spawn();
                if let Err(e) = child {
                    eprintln!("Unkown command -> {}", e);
                    continue;
                } else {
                    child.unwrap().wait().unwrap();
                }
            }
        }
    }
}
