struct User {
    username: String, // !!! ERROR
    email: String,   // !!! ERROR
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let _user = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
}
