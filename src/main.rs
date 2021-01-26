use regex::Regex;
use serde::{Deserialize, Serialize};
use std::io::Read;

#[derive(Deserialize, Default)]
#[serde(rename_all = "camelCase", default)]
struct Input {
    reg_exp: String,
    test_string: String,
    options: Vec<String>,
    mode: String,
    substitution: String,
}

#[derive(Serialize, Default)]
#[serde(rename_all = "camelCase")]
struct Output {
    group_names: Vec<String>,
    indexes: Vec<Vec<usize>>,
    substitute_text: String,
}

fn main() {
    // read from stdin
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    // define input and output, with defaults
    let input: Input = serde_json::from_str(&buf).unwrap();
    let mut output = Output::default();

    // setup options
    let is_global = input.options.contains(&String::from("g"));
    let options: Vec<String> = input
        .options
        .into_iter()
        .filter(|o| o != &String::from("g"))
        .collect();
    let options_fmt = match options.len() {
        0 => String::from(""),
        _ => format!("(?{})", options.join("")),
    };

    // format and compile the regexp
    let regexp = format!("{}{}", options_fmt, input.reg_exp);
    let compiled = Regex::new(&regexp).unwrap();

    // get the group names
    output.group_names = compiled
        .capture_names()
        .map(|c| match c {
            Some(name) => String::from(name),
            None => String::from(""),
        })
        .collect();

    // get matches
    if is_global {
        output.indexes = compiled
            .find_iter(&input.test_string)
            .map(|m| vec![m.start(), m.end()])
            .collect();
    } else {
        output.indexes = match compiled.find(&input.test_string) {
            Some(m) => vec![vec![m.start(), m.end()]],
            None => Vec::new(),
        }
    }

    // get substitutions
    if input.mode == "Substitution" {
        let n = match is_global {
            true => 0,
            false => 1,
        };
        output.substitute_text = compiled
            .replacen(&input.test_string, n, &input.substitution[..])
            .to_string();
    }

    // write to stdout
    print!("{}", serde_json::to_string(&output).unwrap());
}
