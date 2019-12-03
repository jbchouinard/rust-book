use std::convert::{From, TryInto};
use std::fmt::Debug;
use std::ops::{Add, Div, Mul, Sub};
use std::str::FromStr;

pub trait Pow {
    fn powt(self, other: Self) -> Self
    where
        Self: Copy;
}

impl Pow for f64 {
    fn powt(self, other: f64) -> f64 {
        self.powf(other)
    }
}

impl Pow for i64 {
    fn powt(self, other: i64) -> i64 {
        i64::pow(self, other.try_into().unwrap())
    }
}

pub trait Number = Clone
    + Copy
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
    + Pow
    + From<u8>;

pub struct Calculator<T: Number> {
    stack: Vec<T>,
}

impl<T: Number> Calculator<T> {
    pub fn new() -> Calculator<T> {
        Calculator { stack: Vec::new() }
    }
    pub fn copy(&mut self) {
        self.stack.push(self.stack[self.stack.len() - 1])
    }
    pub fn rotate(&mut self) {
        let x = self.stack.pop().expect("stack empty");
        let y = self.stack.pop().expect("stack empty");
        self.stack.push(x);
        self.stack.push(y);
    }
    fn stack_apply<F>(&mut self, op: F, default: T)
    where
        F: FnOnce(T, T) -> T,
    {
        let x = self.stack.pop().unwrap_or(default);
        let y = self.stack.pop().unwrap_or(default);
        self.stack.push(op(y, x));
    }
    fn stack_apply_all<F>(&mut self, op: F, default: T)
    where
        F: FnOnce(T, T) -> T + Copy,
    {
        while self.stack.len() > 1 {
            self.stack_apply(op, default);
        }
    }
    pub fn add(&mut self) {
        self.stack_apply(T::add, 0.into())
    }
    pub fn sum(&mut self) {
        self.stack_apply_all(T::add, 0.into())
    }
    pub fn sub(&mut self) {
        self.stack_apply(T::sub, 0.into())
    }
    pub fn mul(&mut self) {
        self.stack_apply(T::mul, 1.into())
    }
    pub fn product(&mut self) {
        self.stack_apply_all(T::mul, 1.into())
    }
    pub fn div(&mut self) {
        self.stack_apply(T::div, 1.into())
    }
    pub fn pow(&mut self) {
        self.stack_apply(T::powt, 1.into())
    }
}

pub struct CalculatorParser<T: Number + FromStr> {
    pub calc: Calculator<T>,
}

impl<T: Number + FromStr + Debug> CalculatorParser<T> {
    pub fn show(&self) {
        println!("{:?}", self.calc.stack);
    }
}

fn parse_macro(word: &str) -> Vec<&str> {
    match word {
        "square" => vec!["c", "*"],
        "pow2" => vec!["2", "r", "^"],
        "half" => vec!["2", "/"],
        _ => vec![word],
    }
}

impl<T: Number + FromStr> CalculatorParser<T> {
    pub fn new() -> CalculatorParser<T> {
        CalculatorParser {
            calc: Calculator::new(),
        }
    }
    pub fn parse(&mut self, line: &str) -> Result<(), <T as FromStr>::Err> {
        let macros = line.split_whitespace();
        for m in macros {
            for word in parse_macro(m).iter() {
                match *word {
                    "d" => {
                        self.calc.stack.pop();
                    }
                    "c" => self.calc.copy(),
                    "r" => self.calc.rotate(),
                    "+" => self.calc.add(),
                    "-" => self.calc.sub(),
                    "*" => self.calc.mul(),
                    "/" => self.calc.div(),
                    "^" => self.calc.pow(),
                    "s" => self.calc.sum(),
                    "p" => self.calc.product(),
                    _ => {
                        let n: T = word.parse()?;
                        self.calc.stack.push(n);
                    }
                }
            }
        }
        return Ok(());
    }
}
