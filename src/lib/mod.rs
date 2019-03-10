extern crate regex;

pub fn count_syllables(word: &str) -> usize {
    use self::regex::Regex;

    lazy_static! {
        // regex inspired by https://codegolf.stackexchange.com/a/47325
        // credit to StackOverflow user Sp3000
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