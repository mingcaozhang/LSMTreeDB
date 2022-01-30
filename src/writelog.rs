use std::fs::{File, OpenOptions};
use std::io::{BufReader, Write};
use rev_lines::RevLines;

pub struct WriteLog {
    file: File,
}

pub fn open_write_log() -> WriteLog {
    let write_log: WriteLog = WriteLog {
        file: OpenOptions::new()
            .read(true)
            .write(true)
            .append(true)
            .create(true)
            .open("write_log.db")
            .expect("Could not open or create append log.")
    };

    return write_log;
}

impl WriteLog {
    pub fn append(&mut self, kv_pair: &str) -> std::io::Result<()> {
        writeln!(self.file, "{}", kv_pair).map(|_| println!("Successfully wrote row."))
    }

    pub fn find_by_key(&mut self, key: &str, file: &File) -> std::io::Result<()> {
        RevLines::new(BufReader::new(file)).map(|rev| for line in rev {
            let kv_pair: Vec<&str> = line.split(",").collect();
            if key == kv_pair[0] {
                println!("{}", kv_pair[1]);
                break;
            }
        })
    }
}