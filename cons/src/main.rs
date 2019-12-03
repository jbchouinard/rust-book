use std::collections::HashMap;

#[derive(Debug)]
enum Lval {
    Cons(Box<Lval>, Box<Lval>),
    Str(String),
    Sym(String),
    Int(i64),
    Float(f64),
    Bool(bool),
    Nil,
}

type Lenv = HashMap<String, Lval>;

use crate::Lval::{Bool, Cons, Float, Int, Nil, Str, Sym};

impl Lval {
    fn cons(self, b: crate::Lval) -> Lval {
        Cons(Box::new(self), Box::new(b))
    }
    fn str(s: &str) -> Lval {
        Str(String::from(s))
    }
    fn int<T: Into<i64>>(i: T) -> Lval {
        Int(i.into())
    }
    fn float<T: Into<f64>>(f: T) -> Lval {
        Float(f.into())
    }
    fn nil() -> Lval {
        Lval::Nil
    }
    fn bool<T: Into<bool>>(b: T) -> Lval {
        Lval::Bool(b.into())
    }
}

impl Lval {
    fn r#type(&self) -> &str {
        match self {
            Cons(_, _) => "cons",
            Str(_) => "str",
            Sym(_) => "symbol",
            Int(_) => "int",
            Float(_) => "float",
            Bool(_) => "bool",
            Nil => "nil",
        }
    }
}

fn main() {
    let mut env = Lenv::new();
    let list = Lval::bool(true).cons(Lval::float(64.5).cons(Lval::bool(true).cons(Lval::nil())));
    env.insert(String::from("foo"), list);
    println!("{:?}", env);
}
