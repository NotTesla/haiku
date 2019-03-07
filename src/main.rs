#[macro_use] extern crate lazy_static;
extern crate regex;

fn main() {
    let line0 = "Start spirit; behold";
    let line1 = "the skull. A living head loved";
    let line2 = "earth. My bones resign";

    // let words = line.split_whitespace();

    println!("syl({}) - {}", count_syllables(line0), line0);
    println!("syl({}) - {}", count_syllables(line1), line1);
    println!("syl({}) - {}", count_syllables(line2), line2);
}

fn count_syllables(word: &str) -> usize {
    use regex::Regex;

    // regex inspired by https://codegolf.stackexchange.com/a/47325
    // credit to StackOverflow user Sp3000
    lazy_static! {
        static ref SYLLABLE_REGEX: Regex
            = Regex::new("e?[aiouy]+e*|[td]ed|le[^\\w]|e(d|s|ly)([^\\w]|$)|e").unwrap();

        // Our regex crate doesn't have negative lookaheads
        // To create equivalent functionality, catch the undesired inputs
        // In this case, ["ed", "ely", "es"], and remove them after we match
        static ref REMOVE_REGEX: Regex
            = Regex::new("e(d|s|ly)([^\\w]|$)").unwrap();
    }

    let mut syllable_counter = 0usize;
    for m in SYLLABLE_REGEX.find_iter(&word.to_lowercase()[..]) {
        let syllable = m.as_str();
        
        if !REMOVE_REGEX.is_match(syllable) {
            syllable_counter += 1;
        }
    }
    
    syllable_counter
}
