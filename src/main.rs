#[macro_use] extern crate lazy_static;

mod haiku;
mod arg_parser;

use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::string::String;

use haiku::{count_syllables};
use arg_parser::{ArgHandler, ArgParser};

const ERR_READ_LINE : &str = "error reading string";
const ERR_OPEN_FILE : &str = "error opening file";

fn main() {
    let parser = ArgParser::new(vec![
        ArgHandler::new("-h", "print synopsis"),
        ArgHandler::new("-f", "find haikus in file"),
        ArgHandler::new("-u", "write haiku in cmd line"),
    ]);

    parser.interpret_args(&env::args().collect());
    
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
