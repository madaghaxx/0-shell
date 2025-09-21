use std::io::{ self, BufRead, Write };
mod command;
fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();
        let stdin = io::stdin();
        let mut input = String::new();
        let mut handle = stdin.lock();
        handle.read_line(&mut input).unwrap();
        let input = input.trim();
        match input {
            "exit" => {
                break;
            }
            "" => {
                continue;
            }
            _ => {
                let parts: Vec<&str> = input.split_whitespace().collect();
                if !parts.is_empty() {
                    let name = parts[0];
                    let args = &parts[1..];
                    if !crate::command::dispatch(name, args) {
                        println!("Command not found '{name}'");
                    }
                }
            }
        }
    }
}
