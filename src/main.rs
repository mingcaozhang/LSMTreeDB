extern crate rev_lines;

mod command;
mod mem_table;
mod write_log;
mod cli;

use crate::command::Command;
use crate::mem_table::MemTable;
use crate::write_log::WriteLog;

fn main() {
    let mut write_log: WriteLog = write_log::open_write_log();
    let mut mem_table: MemTable = mem_table::MemTable::new();

    loop {
        let input: String = cli::read_input("lsmtree_db > ");
        let cmd: Command = Command::from(&input);
        cmd.execute(&mut mem_table, &mut write_log).map_or_else(|_| println!("Could not execute command: {}", input), |_| ())
    }
}


