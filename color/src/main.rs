use crate::color::Color;
pub mod color;

fn main() {
    let red = Color {
        red: 255,
        green: 0,
        blue: 0,
    };
    let blue = Color {
        red: 0,
        green: 0,
        blue: 255,
    };
    let purple = red.mix(&blue);
    println!("{:?}", purple);
}
