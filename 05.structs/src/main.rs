#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let user1 = User {
        active: true,
        username: String::from("username1"),
        email: String::from("email1@gmail.com"),
        sign_in_count: 1,
    };

    println!("user1: {user1:?}");
    // user1.email = String::from("email11@gmail.com"); Cannot assign value to immutable user1
    
    let mut user2 = User{
        active: false,
        username: String::from("username2"),
        email: String::from("email2@gmail.com"),
        sign_in_count: 0,
    };
    println!("user2: {user2:?}");
    user2.email = String::from("email2222@gmail.com");
    println!("user2: {user2:?}");

    let mut user3 = build_user(String::from("email3@gmail.com"), String::from("username3"));
    user3.email = String::from("email333@gmail.com");
    println!("user3: {user3:?}");

    let user4 = User {
        username: String::from("username4"),
        email: String::from("email4@gmail.com"),
        ..user3
    };
    println!("user4: {user4:?}");

    // user3 still can be accessed
    // because active and sign_in_count are the types that implement Copy trait
    println!("user3: {user3:?}");

    let user5 = User {
        username: String::from("username5"),
        ..user3
    };

    // user3 cannot be used 
    // because user, email are the types that implement Move trait
    // println!("{} {} {} {}", user3.username, user3.active, user3.email, user3.sign_in_count);
    println!("user5: {user5:?}");
    println!("user5 active: {}", user5.id_user_active());
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

impl User {
    fn id_user_active(&self) -> bool {
        self.active
    }
}
