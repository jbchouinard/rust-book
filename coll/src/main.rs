use std::collections::HashMap;
use std::fmt;

fn show<T: fmt::Debug>(o: &T) {
    println!("{:?}", o);
}

fn clone<T: Copy>(v: &Vec<T>) -> Vec<T> {
    let mut vc: Vec<T> = Vec::new();
    for i in v {
        vc.push(*i);
    }
    vc
}

fn incr(v: &mut Vec<i32>) {
    for i in v {
        *i += 1;
    }
}

fn main() {
    let mut v = vec![1, 2, 3, 4];
    v.push(5);
    show(&v);

    let mut v2 = clone(&v);
    show(&v2);

    incr(&mut v2);
    show(&v2);

    let first = &v[0];
    println!("The first element is: {}", first);

    let utf8 = String::from("नमस्ते");
    println!("{}", utf8);
    for b in utf8.bytes() {
        println!("{}", b);
    }
    for c in utf8.chars() {
        println!("{}", c);
    }

    let mut prices: HashMap<String, f64> = HashMap::new();
    prices.insert(String::from("apple"), 10.50);
    prices.insert(String::from("pear"), 12.50);
}
