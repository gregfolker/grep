// Project: grep
// Author: Greg Folker

use std::env;

fn parse_cli_args() -> Vec<String> {
    env::args().collect()
}

fn main() {
    let args = parse_cli_args();

    let query = &args[1];
    let filename = &args[2];

    println!("Searching for '{}' in {}...", query, filename);
}
