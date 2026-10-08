fn main() {
    let hello = String::from("Hello");

    println!("{} is `{}`", "hello", hello);
    
    let mut hello1 = add_exclamation(hello);

    hello1.push_str("!");

    println!("{} is `{}`", "hello1", hello1);
}

fn add_exclamation(s: String) -> String {
    let mut str = s;
    str.push_str("!");
    str
}
