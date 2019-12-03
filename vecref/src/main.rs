fn get_third<'a, T>(vec: &'a Vec<T>) -> &'a T {
    &vec[2]
}

fn main() {
    let myvec = vec![1, 2, 3, 4];
    let a = myvec[0]; // Works because int has Copy trait
    println!("{}", a);

    let mut mystrvec: Vec<String> = Vec::new();
    mystrvec.push(String::from("foo"));
    mystrvec.push(String::from("bar"));
    println!("{:?}", mystrvec);
    let s = &(mystrvec[0]); // [] binds more closely than &
    println!("{}", s);

    let mystrvec2 = vec!["foo", "bar", "baz"];
    println!("{}", get_third(&mystrvec2));
}
