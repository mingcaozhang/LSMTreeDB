use std::collections::HashMap;
use std::fmt::Error;

const SIZE: usize = 10000;


pub struct MemTable {
    data: HashMap<String, String>,
    size: usize,
}

impl MemTable {
    pub fn new() -> MemTable {
        MemTable {
            data: HashMap::with_capacity(SIZE),
            size: 0,
        }
    }

    pub fn put(&mut self, key: String, value: String) -> Result<(), Error> {
        self.data.insert(key, value);
        self.size += 1;
        Ok(())
    }

    pub fn get(&mut self, key: String) -> Result<Option<&String>, ()> {
        Ok(self.data.get(key.as_str()))
    }
}