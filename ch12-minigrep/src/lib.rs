use std::{error::Error, fs};

pub struct Config<'a> {
    pub query: &'a String,
    pub file_path: &'a String,
}

impl Config<'_> {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }

        let query = &args[1];
        let file_path = &args[2];

        Ok(Config { query, file_path })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    println!("With text:\n{contents}");

    Ok(())
}
