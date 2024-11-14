#[derive(Debug)]
pub struct User {
    active: bool,
    username: String,
    email: String,
    password: String,
}

pub mod manage_account {
    use super::User;

    pub fn new(username: String, email: String, password: String) -> User {
        let user = User {
            active: true,
            username: username,
            email: email,
            password: password,
        };
        user
    }
    pub fn reset_password(user: User) -> User {
        let new_user = User {
            password: String::from("1234"),
            ..user
        };
        println!("Password is now 1234");
        new_user
    }
    pub fn change_email(user: User, email: String) -> User {
        let new_user = User {
            email: email,
            ..user
        };
        println!("Email changed");
        new_user
    }
    pub fn change_name(user: User, username: String) -> User {
        let new_user = User {
            username: username,
            ..user
        };
        println!("Username changed");
        new_user
    }
}

pub mod account_enum {
    use super::User;

    pub fn print(user: User) {
        println!("user -> {:#?}", user);
    }

    pub fn debug(user: User) {
        dbg!(user);
    }
}

pub mod list_users {
    use super::User;

    pub fn print_users(users: Vec<User>) {
        println!("users -> {:#?}", users);
    }
}