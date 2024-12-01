struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username, //short version of username: username (parameter name)
        email,
        sign_in_count: 1,
    }
}

//tuple struct
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("someusername"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    println!("is user1 active? {}", user1.active);
    println!("user1 username: {}", user1.username);
    println!("user1 email: {}", user1.email);
    user1.username = "newusername".to_string();
    println!("user1 username updated: {}", user1.username);

    let user2 = build_user("test@example.com".to_string(), "testusername".to_string());
    println!("user2 username: {}", user2.username);

    /*
    let mut user3 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    };
    */

    //The syntax .. specifies that the remaining fields not explicitly
    //set should have the same value as the fields in the given instance.
    let user3 = User {
        email: String::from("another@example.com"),
        // the following uses syntax =, moving data
        //or copying it if the type implements copy trait (case of active and sign_in_count)
        ..user1
    };
    println!("user3 username: {}", user3.username);
    println!("user3 email: {}", user3.email);
    // the following will work as it wasn't moved
    println!("user1 email: {}", user1.email);
    /*println!("user1 username: {}", user1.username); // won't work as it was moved*/
    // the following will work as it was copied only
    println!("user1 sign in count: {}", user1.sign_in_count);

    let black = Color(0, 0, 0);
    println!("{}{}{}", black.0, black.1, black.2);
    let origin = Point(0, 5, 10);
    println!("{}{}{}", origin.0, origin.1, origin.2);
    /* Note that the black and origin values are different types because they’re instances of
     * different tuple structs. Each struct you define is its own type, even though the fields
     * within the struct might have the same types. For example, a function that takes a parameter
     * of type Color cannot take a Point as an argument, even though both types are made up of
     * three i32 values. Otherwise, tuple struct instances are similar to tuples in that you can
     * destructure them into their individual pieces, and you can use a . followed by the index
     * to access an individual value.
     */
}
