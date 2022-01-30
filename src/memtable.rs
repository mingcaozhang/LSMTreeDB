use std::collections::HashMap;

const SIZE: usize = 10000;


pub struct Memtable {
    data: HashMap<String, String>,
    size: usize,
}