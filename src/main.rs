use std::env;

use regex::Regex;

fn main() {
    let args: Vec<String> = env::args().collect();
    let regexp_in = &args[1];
    let text_in = &args[2];

    let regexp = Regex::new(regexp_in);
    let compiled = match regexp {
        Ok(r) => r,
        Err(e) => {
            panic!(e.to_string())
        }
    };

    let result = compiled.is_match(text_in);

    println!("Result: {}", result);
}
