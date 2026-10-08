fn main() {
    let mut message = String::from("Rust is great!");

    let c = get_last_char(&message);
    println!("Last char is '{}'", c);

    uppercase_and_print(&mut message);
}

fn get_last_char(msg: &String) -> char {
    msg.chars().last().unwrap()
}

fn uppercase_and_print(msg: &mut String) {
    *msg = msg.to_uppercase();
    println!("The message is \"{}\"", msg);
}