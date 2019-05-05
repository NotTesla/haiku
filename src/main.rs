#[macro_use] extern crate lazy_static;
extern crate clap;

mod haiku;

use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::string::String;

use clap::{App, Arg};

use haiku::{get_haiku};

const ERR_READ_LINE : &str = "error reading string";
const ERR_OPEN_FILE : &str = "error opening file";

fn main() {
    let matches = App::new("haiku")
        .version("1.0")
        .about("Output all haikus contained in a file")
        .author("Mark Grass")
        .arg(Arg::with_name("file")
            .help("Additional input file(s)")
            .short("f")
            .long("file")
            .value_name("INPUT")
            .takes_value(true)
            .min_values(1)
            .multiple(true))
        .arg(Arg::with_name("INPUT")
            .help("Input file(s)")
            .index(1)
            .multiple(true))
        .get_matches();

    // get all input files listed by the user

    let mut input_files = Vec::new();

    if let Some(input) = matches.values_of("INPUT") {
        input_files.append(&mut input.collect());
    }
    if let Some(input) = matches.values_of("file") {
        input_files.append(&mut input.collect());
    }

    // iterate over each file and count the number of syllables inside
    for file in input_files.iter() {
        
        let mut stream = open_file(file);
        let mut line = String::new();

        while stream.read_line(&mut line).expect(ERR_READ_LINE) != 0 {
            
            if let Some(haiku) = get_haiku(&line) {
                println!("{}", haiku);
            }
            
            line.clear();
        }
    }
}

fn open_file(file_name: &str) -> BufReader<File> {
    let file = File::open(file_name).expect(ERR_OPEN_FILE);
    BufReader::new(file)
}
