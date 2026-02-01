struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

//  Tuple structs
struct Color (i32, i32, i32);
struct Point (i32, i32, i32);

//  Unit like structs
struct AlwaysEqual;

fn main() {
    println!("Hello, world!");
    let user_1 = User {
        active: true,
        username: String::from("someusername"),
        email: String::from("someusername@example.com"),
        sign_in_count: 1,
    };

    println!("User 1's email is {}", user_1.email);
    println!("User 1's username is {}", user_1.username);
    println!("User 1's sign in count is {}", user_1.sign_in_count);
    println!("User 1's active status is {}", user_1.active);

    let mut user_2 = User {
        active: false,
        username: String::from("anotherusername"),
        email: String::from("anotherusername@example.com"),
        sign_in_count: 5,
    };

    println!("User 2's email is {}", user_2.email);
    let new_email = String::from("newemailaddress@example.com");
    println!("updating email to {}", new_email);
    user_2.email = new_email;
    println!("User 2's updated email is {}", user_2.email);

    let user_3 = User {
        email: String::from("user3@example.com"),
        username: String::from("user3name"),
        active: user_2.active,
        sign_in_count: user_2.sign_in_count,
    };

    let user_4 = User {
        email: String::from("user4@example.com"),
        username: String::from("user4name"),
        ..user_3
    };

    let black = Color (0, 0, 0);
    let origin = Point (0, 0, 0);

    let Point(x, y, z) = origin;
    println!("The value of origin is: {}, {}, {}", x, y, z);
    let Color(r, g, b) = black;
    println!("The value of black is: {}, {}, {}", r, g, b);
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

fn build_user_with_struct_update_syntax(email: String, username: String, other: User) -> User {
    User {
        email,
        username,
        ..other
    }
}
