struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// tuple structs
struct Point(i32, i32, i32);

fn main() {
    let username = String::from("someusername123");
    let email = String::from("someone@example.com");

    // Note that the entire instance must be mutable; Rust doesn’t allow us to
    // mark only certain fields as mutable.
    let mut user1 = build_user(email, username);

    user1.email = String::from("anotheremail@example.com");

    println!("user1 e-mail: {}", user1.email);

    let user1 = increase_user_sign_in_count(user1);

    println!("user1 sign in count: {}", user1.sign_in_count);

    let user1 = increase_user_sign_in_count(user1);

    println!("user1 sign in count: {}", user1.sign_in_count);

    let origin = Point(0, 0, 0);
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

fn increase_user_sign_in_count(user: User) -> User {
    User {
        sign_in_count: user.sign_in_count + 1,
        ..user // struct update syntax
    }
}
