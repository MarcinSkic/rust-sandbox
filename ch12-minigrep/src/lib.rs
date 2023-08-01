use std::{env, error::Error, fs};

#[derive(Debug, PartialEq)]
pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
    pub print_author: bool,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();

        let mut ignore_case = env::var("IGNORE_CASE").is_ok();
        let mut print_author = false;

        let query = match args.next() {
            Some(query) => query,
            None => return Err("Missing query string"),
        };

        let file_path = match args.next() {
            Some(path) => path,
            None => return Err("Missing path string"),
        };

        for flag in args {
            match flag.as_str() {
                "-i" => ignore_case = true,
                "-a" => print_author = true,
                _ => (),
            }
        }

        Ok(Config {
            query,
            file_path,
            ignore_case,
            print_author,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    if config.print_author {
        println!("Author: Marcin Skic");
    }

    for line in results {
        println!("{line}");
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();

    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }

    fn command_to_args(command: &str) -> Vec<String> {
        let mut args: Vec<String> = vec![];

        for arg in command.split(" ") {
            args.push(String::from(arg));
        }

        args
    }

    #[test]
    fn work_without_flags() {
        let args = command_to_args("program_name to poem.txt").into_iter();

        assert_eq!(
            Config::build(args).unwrap(),
            Config {
                query: "to".to_string(),
                file_path: "poem.txt".to_string(),
                ignore_case: false,
                print_author: false
            }
        );
    }

    #[test]
    fn detect_one_flag() {
        let args = command_to_args("program_name to poem.txt -i").into_iter();

        assert_eq!(
            Config::build(args).unwrap(),
            Config {
                query: "to".to_string(),
                file_path: "poem.txt".to_string(),
                ignore_case: true,
                print_author: false
            }
        );
    }

    #[test]
    fn detect_multiple_flags() {
        let args = command_to_args("program_name to poem.txt -a -i").into_iter();

        assert_eq!(
            Config::build(args).unwrap(),
            Config {
                query: "to".to_string(),
                file_path: "poem.txt".to_string(),
                ignore_case: true,
                print_author: true
            }
        );
    }

    #[test]
    fn ignore_undefined_flags() {
        let args = command_to_args("program_name lol poem.txt -lol -i -xd -a").into_iter();

        assert_eq!(
            Config::build(args).unwrap(),
            Config {
                query: "lol".to_string(),
                file_path: "poem.txt".to_string(),
                ignore_case: true,
                print_author: true
            }
        );
    }
}
