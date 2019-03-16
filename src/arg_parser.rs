use std::collections::HashMap;
use std::string::String;

struct ArgParams {
    identifier: &'static str
}

pub struct ArgHandler {
    flag: &'static str,
    synopsis: &'static str,
    variadic: bool,
}

pub struct ArgParser {
    handlers: HashMap <&'static str, ArgHandler>,
}

impl ArgHandler {

    pub fn new(flag: &'static str, synopsis: &'static str) -> ArgHandler {
        ArgHandler {
            flag:       flag,
            synopsis:   synopsis,
            variadic:   false,
        }
    }

    pub fn with_variadic(flag: &'static str, synopsis: &'static str) -> ArgHandler {
        ArgHandler {
            flag:       flag,
            synopsis:   synopsis,
            variadic:   true,
        }
    }
}

impl ArgParser {

    pub fn new(handlers: Vec<ArgHandler>) -> ArgParser {
        let mut parser = ArgParser {
            handlers: HashMap::new(),
        };
        
        for handler in handlers {
            parser.handlers.insert(handler.flag, handler);
        }
        
        parser
    }

    pub fn interpret_args(&self, _args: &Vec<String>) {
        // for arg in args {
        //     TODO: some sort of state machine
        //     that tracks prev flag, and then
        //     checks for relevant vars afterwards
        //     e.g. $ -> -f -> [filename] -> $ -> -h -> $
        //     match arg.as_ref() {
        //         "-f" => {},
        //         "-h" => {},
        //         "-u" => {},
        //         _ => {}
        //     }
        // }
    }
}
