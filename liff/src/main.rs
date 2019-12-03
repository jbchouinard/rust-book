fn to_owned_str(str_ref: &String) -> String {
    str_ref.clone()
}

fn main() {
    let s = String::from("foobar");
    let s2 = to_owned_str(&s);
    println!("{}, {}", s, s2)
}
