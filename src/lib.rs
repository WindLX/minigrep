use colored::Colorize;
use std::env;
use std::error::Error;
use std::fs;

pub struct Config<'a> {
    pub query: &'a String,
    pub file_name: &'a String,
    pub case_sensitive: bool,
}

impl Config<'_> {
    pub fn new(args: &Vec<String>) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = &args[2];
        let file_name = &args[1];
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        Ok(Config {
            query,
            file_name,
            case_sensitive,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_name)?;
    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        println!(
            "{}{}",
            " INFO ".on_blue().bold(),
            " Case insensitive".green(),
        );
        search_case_insensitive(&config.query, &contents)
    };
    println!(
        "{}{}\n{}",
        " INFO ".on_blue().bold(),
        " Results contain lines below: ".green(),
        results,
    );
    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> String {
    let mut results = String::new();
    for line in contents.lines() {
        if line.contains(query) {
            results.push_str(line);
            results.push('\n');
        }
    }
    results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> String {
    let mut results = String::new();
    let query = query.to_lowercase();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push_str(line);
            results.push('\n');
        }
    }
    results
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
Pick three.";
        assert_eq!("safe, fast, productive.\n", search(query, contents))
    }

    #[test]
    fn case_insensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, proDuctive.
Pick three.";
        assert_eq!(
            "safe, fast, proDuctive.\n",
            search_case_insensitive(query, contents)
        )
    }
}
