extern crate rev_lines;

mod operations;

use std::fs::{File, OpenOptions};
use std::io::{BufReader, Write};
use std::io::Result;
use std::str::FromStr;
use operations::Operation;
use rev_lines::RevLines;

fn main() {
    let db_file: File = OpenOptions::new()
        .read(true)
        .write(true)
        .append(true)
        .create(true)
        .open("log.db")
        .expect("Could not open or create append log.");

    loop {
        let input: String = prompt("lsmtree_db > ");
        handle_input(input.as_str(), &db_file).expect("Operation failed.")
    }
}

fn prompt(prefix: &str) -> String {
    let mut line: String = String::new();
    print!("{}", prefix);

    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut line).expect("Error: Could not read input.");
    return line.trim().to_string();
}

fn handle_input(input: &str, file: &File) -> Result<()> {
    let vec: Vec<&str> = input.split(" ").collect();
    match Operation::from_str(vec[0]) {
        Ok(Operation::ADD) => append_log(vec[1], file),
        Ok(Operation::GET) => read_log(vec[1], file),
        Err(..) => Ok(println!("Operation is invalid."))
    }
}

fn append_log(kv_pair: &str, mut file: &File) -> Result<()> {
    writeln!(file, "{}", kv_pair).map(|_| println!("Successfully wrote row."))
}

fn read_log(key: &str, file: &File) -> Result<()> {
    RevLines::new(BufReader::new(file)).map(|rev| for line in rev {
        let kv_pair: Vec<&str> = line.split(",").collect();
        if key == kv_pair[0] {
            println!("{}", kv_pair[1]);
            break;
        }
    })
}
