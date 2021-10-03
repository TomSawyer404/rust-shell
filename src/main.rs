use std::io::*;
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

        if command == "exit" {
            return;
        }

        let child = Command::new(command).args(args).spawn();
        if let Err(i) = child {
            eprintln!("Unkown command -> {}", i);
            continue;
        } else {
            child.unwrap().wait().unwrap();
        }
    }
}
