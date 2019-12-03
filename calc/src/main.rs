#![feature(trait_alias)]

pub mod calc;
use std::io;

use crate::calc::CalculatorParser;

fn main() {
    let mut calc: CalculatorParser<f64> = CalculatorParser::new();
    loop {
        let mut line = String::new();
        match io::stdin().read_line(&mut line) {
            Ok(_) => (),
            Err(error) => println!("{}", error),
        };
        match calc.parse(&line[..]) {
            Ok(_) => (),
            Err(word) => println!("parser error: {}", word),
        }
        calc.show();
    }
}
