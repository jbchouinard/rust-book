pub mod geom;
pub mod point;

use geom::Area;
use point::Point;

fn swap<T: Copy>(p1: &mut Point<T>, p2: &mut Point<T>) {
    let tmp0 = p1.0;
    let tmp1 = p1.1;
    p1.0 = p2.0;
    p1.1 = p2.1;
    p2.0 = tmp0;
    p2.1 = tmp1;
}

fn main() {
    let mut p1 = Point(3.0, 4.0);
    println!("p1 = {:?}", p1);
    let mut p2 = p1 * 2.0;
    println!("p2 = p1 * 2.0 = {:?}", p2);
    let p3 = p1 + p2;
    println!("p3 = p1 + p2 = {:?}", p3);
    println!("p3: area={}, magnitude={}", p3.area(), p3.magnitude());
    println!("p1 == p1: {}", p1 == p1);
    println!("p1 == p2: {}", p1 == p2);

    swap(&mut p1, &mut p2);
}
