// Project: grep
// Author: Greg Folker

use std::env;

static NUM_CLI_ARGS: usize = 3;

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Config {
        assert_eq!(
           args.len(), NUM_CLI_ARGS,
            "Expected {} arguments, got {}",
            (NUM_CLI_ARGS - 1), (args.len() - 1)
        );

        /*
        * @note: The `clone()` method takes more time and memory
        * than using a reference to the string data. However, the
        * tradeoff is that it is more straightforward because now
        * the lifetime of the references do not need to be managed.
        * In this circumstance, giving up a little performance to
        * gain simplicity is a worthwhile trade-off.
        *
        * There is a tendency for many Rust programmers to avoid
        * using the `clone()` method because of the implications
        * to performance. However, sometimes it is better to have
        * a working program that is a little bit less efficient
        * than try to hyperoptimize it.
        */
        let query = args[1].clone();
        let filename = args[2].clone();

        Config { query, filename }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args);

    println!("Searching for '{}' in {}...", config.query, config.filename);
}
