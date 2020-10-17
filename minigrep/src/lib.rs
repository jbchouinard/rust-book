#[macro_use]
extern crate simple_error;

use simple_error::SimpleError;

use std::fs;

#[derive(Debug)]
pub struct Config {
    query: String,
    filename: String,
}

impl Config {
    pub fn new(query: String, filename: String) -> Config {
        Config { query, filename }
    }
    pub fn from_args(args: &[String]) -> Result<Config, SimpleError> {
        if args.len() != 3 {
            return Result::Err(SimpleError::new("Expected 2 arguments"));
        }
        Result::Ok(Self::new(args[1].clone(), args[2].clone()))
    }
    pub fn filename(&self) -> &String {
        &self.filename
    }
    pub fn query(&self) -> &String {
        &self.query
    }
}

pub fn search<'a>(query: &str, text: &'a str) -> Vec<&'a str> {
    let mut res: Vec<&str> = Vec::new();
    for line in text.lines() {
        if line.contains(query) {
            res.push(line);
        }
    }
    res
}

pub fn run(config: Config) -> Result<(), simple_error::SimpleError> {
    let text = try_with!(fs::read_to_string(config.filename()), "Failed to read file");
    for line in search(config.query(), &text) {
        println!("{}", line);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_from_args() {
        let args = [
            "minigrep".to_string(),
            "foo".to_string(),
            "bar.txt".to_string(),
        ];
        let config = Config::from_args(&args).unwrap();

        assert_eq!(config.query(), "foo");
        assert_eq!(config.filename(), "bar.txt");
    }

    #[test]
    fn search_exact_string() {
        let text = "\
apple
banana
pear
pineapple";
        let query = "ap";

        let res = search(query, text);

        assert_eq!(res, vec!["apple", "pineapple"]);
    }
}
