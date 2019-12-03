fn main() {
    let arr = [300; 3];
    if is_large(arr3_sum(arr)) {
        println!("The sum is large");
    } else {
        println!("The sum is small");
    }
}

fn arr3_sum(arr: [i32; 3]) -> i32 {
    let mut sum = 0;
    for n in arr.iter() {
        sum += n;
    }
    sum
}

fn is_large(n: i32) -> bool {
    n > 100
}
