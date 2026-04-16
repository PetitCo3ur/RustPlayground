use std::env::{self, args};
use std::error::Error;
use std::process;

use minigrep::search;

fn main() {
    let config = Config::build(args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = run(config) {
        eprintln!("Error running minigrep: {}", e);
        process::exit(1);
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = std::fs::read_to_string(config.filename)?;

    let results = if config.ignore_case {
        Box::new(minigrep::search_case_insensitive(&config.query, &content)) as Box<dyn Iterator<Item = &str>>
    } else {
        Box::new(search(&config.query, &content)) as Box<dyn Iterator<Item = &str>>
    };

    for line in results {
        println!("{}", line);
    }
    Ok(())
}

struct Config {
    query: String,
    filename: String,
    ignore_case: bool,
}

impl Config {
    fn build(args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        let args = args.collect::<Vec<String>>();
        
        if args.len() < 3 {
            return Err("Not enough arguments");
        }

        let query = match args.get(1) {
            Some(arg) => arg.clone(),
            None => return Err("Didn't get a query string"),
        };

        let filename = match args.get(2) {
            Some(arg) => arg.clone(),
            None => return Err("Didn't get a filename"),
        };

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config { query, filename, ignore_case })
    }
}
