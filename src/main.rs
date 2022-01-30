extern crate rev_lines;

mod command;
mod memtable;
mod writelog;
mod cli;

use std::env;
use std::fs::{File, OpenOptions};
use std::io::{BufReader, Write};
use std::str::FromStr;
use rev_lines::RevLines;
use crate::command::Command;
use crate::memtable::Memtable;
use crate::writelog::WriteLog;

fn main() {
    let write_log = writelog::open_write_log();
    // let mem_table = memtable::Memtable::new();

    loop {
        let input: String = cli::read_input("lsmtree_db > ");
        let cmd: Command = Command::from(&input);
        cmd.execute(&write_log).map_or_else(|_| println!("Could not execute command: {}", input), |_| ())
    }
}


// fn handle_input(input: &str, write_log: WriteLog, mem_table: &Memtable) -> Result<()> {
//     let vec: Vec<&str> = input.split(" ").collect();
//     match Operation::from_str(vec[0]) {
//         Ok(Operation::ADD) => write_log.append(vec[1]),
//         Ok(Operation::GET) => write_log.find_by_key(vec[1], file),
//         Err(..) => Ok(println!("Operation is invalid."))
//     }
// }


