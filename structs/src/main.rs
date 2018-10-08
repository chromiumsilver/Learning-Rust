fn main() {
    let mut user1 = User {
        email: String::from("one@example.com"),
        username: String::from("onename"),
        sign_in_count: 1,
        active: true,
    };
    // change a value
    user1.email = String::from("another@example.com");
    
    // update
    let user2 = User {
        email: String::from("aaa@example.com"),
        username: String::from("twoname"),
        ..user1 // use the rest of the values of user1
    };

    // tuple structs
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);
    let black = color(0, 0, 0);
    let origin = Point(0, 0, 0);
}

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

// field init
fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

