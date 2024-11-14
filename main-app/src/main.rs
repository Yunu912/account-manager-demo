use crate::account::User;

mod account;
mod account_input;
mod stdio;

fn main() {
    let user1 = account::manage_account::new(
        String::from("username"),
        String::from("anyemail@jetbrains.com"),
        String::from("thepassword"),
    );
    let user2 = account::manage_account::new(
        String::from("user2"),
        String::from("user2email@example.com"),
        String::from("password2"),
    );
    let users: Vec<User> = vec![user1, user2];
    account::list_users::print_users(users);
}