// Project: grep
// Author: Greg Folker

use std::env;

static NUM_CLI_ARGS: usize = 3;

fn main() {
    let args: Vec<String> = env::args().collect();

    let (query, filename) = parse_cli_args(&args);

    println!("Searching for '{}' in {}...", query, filename);
}

fn parse_cli_args(args: &[String]) -> (&str, &str) {
    assert_eq!(
        args.len(), NUM_CLI_ARGS,
        "Expected {} arguments, got {}",
        (NUM_CLI_ARGS - 1), (args.len() - 1)
    );

    let query = &args[1];
    let filename = &args[2];

    (query, filename)
}
