#[macro_use] extern crate lazy_static;

mod lib;

use lib::{count_syllables};
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

const ERR_READ_LINE : &str = "error reading string";
const ERR_OPEN_FILE : &str = "error opening file";

fn main() {

    let mut buffer = open_file("test.txt");
    let mut line = String::new();

    while buffer.read_line(&mut line).expect(ERR_READ_LINE) != 0 {

        for word in line.split_whitespace() {
            println!("syl({}) - {}", count_syllables(word), word);
        }

    }
}

fn open_file(file_name : &str) -> BufReader<File> {
    let file = File::open(file_name).expect(ERR_OPEN_FILE);
    BufReader::new(file)
}
