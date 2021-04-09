// Project: grep
// Author: Greg Folker

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let (query, filename) = parse_cli_args(&args);

    println!("Searching for '{}' in {}...", query, filename);
}

fn parse_cli_args(args: &[String]) -> (&str, &str) {
    let query = &args[1];
    let filename = &args[2];

    (query, filename)
}
