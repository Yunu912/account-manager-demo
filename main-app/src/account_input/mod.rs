use std::io;
use std::io::Write;
use crate::account;

fn new_user() -> account::User {
    let mut input: String;

    print!("Input username: ");
    io::stdout().flush().expect("Flush error");
    io::stdin().read_line(&mut input).expect("Input error");
    let &username = &input;

    print!("Input email: ");
    io::stdout().flush().expect("Flush error");
    io::stdin().read_line(&mut input).expect("Input error");
    let &email = &input;

    print!("Input password: ");
    io::stdout().flush().expect("Flush error");
    io::stdin().read_line(&mut input).expect("Input error");
    let &password = &input;

    account::manage_account::new(username, email, password)
}

fn login(users: Vec<account::User>) -> String {
    let mut input: String;

    print!("Input username you want to login: ");
    io::stdout().flush().expect("Flush error");
    io::stdin().read_line(&mut input).expect("Input error");
}