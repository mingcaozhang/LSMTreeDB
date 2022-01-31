use std::fs::{File, OpenOptions};
use std::io::Write;

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
            .open("append.log")
            .expect("Could not open or create append log.")
    };

    return write_log;
}

impl WriteLog {
    pub fn append(&mut self, key: &str, value: &str) -> std::io::Result<()> {
        writeln!(self.file, "ADD {},{}", key, value)
    }

    // TODO move to SSTable
    // pub fn find_by_key(&mut self, key: &str) -> std::io::Result<Option<String>> {
    //     let mut file = self.file.borrow();
    //     let mut value: Option<String> = Option::None;
    //     let result: std::io::Result<Option<String>> = RevLines::new(BufReader::new(file)).map(|rev| {
    //         for line in rev {
    //             let kv_pair: Vec<&str> = line.split(",").collect();
    //             println!("{}", kv_pair[0]);
    //             if key == kv_pair[0] {
    //                 value = Option::Some(kv_pair[1].to_string());
    //                 break;
    //             }
    //         }
    //     }).map(|_| value);
    //     return result;
    // }
}