use minigrep::search;
use std::env;
use std::error::Error;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    dbg!(&args);

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("{err}");
        process::exit(1);
    });

    println!("Search for {}", config.query);
    println!("In file {}", config.file_path);

    if let Err(err) = run(config) {
        println!("Application error: {err}");
        process::exit(1);
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    for line in search(&config.query, &contents) {
        println!("{line}");
    }

    Ok(())
}

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn build(args: &Vec<String>) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Syntax: cargo run -- [search_string] [file]");
        }

        let query = &args[1];
        let file_path = &args[2];

        Ok(Config {
            query: String::from(query),
            file_path: String::from(file_path),
        })
    }
}
