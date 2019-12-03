#[macro_use]
extern crate simple_error;

use std::env;
use std::fs;

use minigrep::Config;

fn main() -> Result<(), simple_error::SimpleError> {
    let args: Vec<String> = env::args().collect();
    let config = Config::from_args(&args)?;
    let contents = try_with!(fs::read_to_string, config.filename());
    println!("{:#?}", config);
    println!("{}", contents);
    Result::Ok(())
}
