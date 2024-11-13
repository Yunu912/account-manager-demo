mod account;

fn main() {
    let user1 = account::manage_account::new(
        String::from("username"),
        String::from("anyemail@jetbrains.com"),
        String::from("thepassword"),
    );
}