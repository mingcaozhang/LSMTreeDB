use std::process::exit;
use crate::Memtable;
use crate::writelog::WriteLog;

pub enum Command {
    ADD { kv_pair: String },
    GET { key: String },
    EXIT,
    NOOP,
}

impl From<&String> for Command {
    fn from(input: &String) -> Self {
        let vec: Vec<&str> = input.split(" ").collect();
        match vec[0].to_uppercase().as_str() {
            "ADD" => vec.get(1).map(|kv_pair| Command::ADD { kv_pair: kv_pair.to_string() }).unwrap_or(Command::NOOP),
            "GET" => vec.get(1).map(|key| Command::GET { key: key.to_string() }).unwrap_or(Command::NOOP),
            "EXIT" | "QUIT" => Command::EXIT,
            _ => Command::NOOP
        }
    }
}

impl Command {
    pub fn execute(&self, write_log: &WriteLog) -> Result<(), ()> {
        match self {
            Command::ADD { kv_pair } => Ok((println!("Add {}", kv_pair))),
            Command::GET { key } => Ok((println!("Get {}", key))),
            Command::EXIT => Ok(quit()),
            Command::NOOP => Ok(())
        }
    }
}


pub fn quit() {
    exit(0)
}