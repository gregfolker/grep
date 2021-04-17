use std::error::Error;
use std::fs;

static NUM_CLI_ARGS: usize = 3;

pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() != NUM_CLI_ARGS {
            return Err("Incorrect number of arguments provided");
        }

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

        Ok(Config { query, filename })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
    Ok(())
}
