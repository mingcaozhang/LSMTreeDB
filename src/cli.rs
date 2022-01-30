use std::io::Write;
use std::str::FromStr;
use std::string::ParseError;
use crate::Command;

pub fn read_input(prompt: &str) -> String {
    let mut line: String = String::new();
    print!("{}", prompt);

    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut line).expect("Error: Could not read input.");
    return line.trim().to_string();
}