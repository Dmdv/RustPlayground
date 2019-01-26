struct User {
    name: String,
    email: String,
    sign_in_count: u64,
    active: bool
}


fn main() {
    
    let user1 = User {
        name: String::from("User name"),
        email: String::from("example@google.com"),
        sign_in_count: 10,
        active: true
    };

    let user2 = User {
        name: String::from("new mail"),
        active: false,
        ..user1
    };

    // This will result to error. bc email is borrowed
    //println!("User = {}", user1.email);
    println!("User = {}", user2.email);

    let user3 = create_user(String::from("user3"), String::from("user3@mail.com"));

    println!("User3 = {}", user3.email);
}

fn create_user(name:String, email:String) -> User {
    User {
        name,
        email,
        active: true,
        sign_in_count: 1
    }
}
