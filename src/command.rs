use std::fmt::Error;
use std::process::exit;
use crate::MemTable;
use crate::write_log::WriteLog;

pub enum Command {
    ADD { key: String, value: String },
    GET { key: String },
    EXIT,
    NOOP,
}

impl From<&String> for Command {
    fn from(input: &String) -> Self {
        let vec: Vec<&str> = input.split(" ").collect();
        match vec[0].to_uppercase().as_str() {
            "ADD" => {
                let kv_vec: Vec<&str> = vec.get(1).map(|kv_pair| kv_pair.split(",").collect()).unwrap_or(Vec::new());
                kv_vec.get(0).map(|key| kv_vec.get(1).map(|value| Command::ADD { key: key.to_string(), value: value.to_string() })).flatten().unwrap_or(Command::NOOP)
            }
            "GET" => vec.get(1).map(|key| Command::GET { key: key.to_string() }).unwrap_or(Command::NOOP),
            "EXIT" | "QUIT" => Command::EXIT,
            _ => Command::NOOP
        }
    }
}

impl Command {
    pub fn execute(&self, mem_table: &mut MemTable, write_log: &mut WriteLog) -> Result<(), Error> {
        match self {
            Command::ADD { key, value } => add(mem_table, write_log, key, value),
            Command::GET { key } => get(mem_table, key),
            Command::EXIT => Ok(quit()),
            Command::NOOP => Ok(())
        }
    }
}

pub fn add(mem_table: &mut MemTable, write_log: &mut WriteLog, key: &str, value: &str) -> Result<(), Error> {
    let write_result = write_log.append(key, value);
    let put_result = mem_table.put(key.to_string(), value.to_string());
    write_result.map(|_| put_result).map(|_| println!("Successfully wrote row.")).map_err(|_| Error)
}

pub fn get(mem_table: &mut MemTable, key: &str) -> Result<(), Error> {
    let opt_val = mem_table.get(key.to_string()).unwrap_or(Option::None);
    match opt_val {
        Option::None => Ok(println!("No value found for key.")),
        Option::Some(val) => Ok(println!("{}", val))
    }
}

pub fn quit() {
    exit(0)
}