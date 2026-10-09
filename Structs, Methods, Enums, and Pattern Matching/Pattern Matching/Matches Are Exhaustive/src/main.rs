fn plus_one(x: Option<i32>) -> Option<i32> {
    // !!! ERROR: Match must be exhaustive
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }

}

fn main() {
    let four = Some(4);
    let five = plus_one(four);
    let none = plus_one(None);

    println!("{:?}", five);
    println!("{:?}", none);
}
