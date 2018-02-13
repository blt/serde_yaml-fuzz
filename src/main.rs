#[macro_use]
extern crate serde_derive;
extern crate serde_yaml;

use std::io;

#[derive(Debug, PartialEq, Deserialize)]
struct Input { // 22 bytes
    seed_a: u32, // 4
    seed_b: u32, // 4
    seed_c: u32, // 4
    seed_d: u32, // 4
    max_in_memory_bytes: u32, // 4
    max_disk_bytes: u16, // 2
}

fn main() {
    let stdin = io::stdin();
    let stdin = stdin.lock();

    let _res: serde_yaml::Result<Input> =  serde_yaml::from_reader(stdin);
}
