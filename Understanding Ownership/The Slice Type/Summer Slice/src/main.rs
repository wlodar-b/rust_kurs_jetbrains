fn main() {
    let months = ["January", "February", "March",
        "April", "May", "June",
        "July", "August", "September",
        "October", "November", "December"];

    let summer_months = &months[5..8];
    println!("{:?}", summer_months)
}
