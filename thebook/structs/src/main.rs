struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let user1 = User {
        active: true,
        username: String::from("jjackson"),
        email: String::from("jjackson@someplace.com"),
        sign_in_count: 22,
    };

    println!("user: {}", user1.username);
}
