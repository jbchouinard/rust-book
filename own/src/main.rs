use std::io;

#[derive(Debug)]
struct Point {
    x: u64,
    y: u64,
}

impl Point {
    fn magnitude(&self) -> u64 {
        self.x * self.y
    }
    fn swap(&mut self) {
        let tmp = self.x;
        self.x = self.y;
        self.y = tmp;
    }
    fn double(self) -> Point {
        Point {
            x: self.x * 2,
            y: self.y * 2,
        }
    }
}

fn read_number() -> u64 {
    let mut n = String::new();
    io::stdin().read_line(&mut n)
        .expect("Could not read stdin");
    let n: u64 = n.trim()
        .parse()
        .expect("Not a number");
    n
}

fn main() {
    let x = read_number();
    let y = read_number();
    let mut p = Point{x, y};
    println!("{:#?}", p);
    p.swap();
    println!("{:#?}", p);
    let p = p.double();
    println!("{:#?}", p);
    println!("{}", p.magnitude());
}
