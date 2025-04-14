use std::io;

// const HOSTPORT: &str = "0.0.0.0:8080";

// const ERR_USERNAME_TAKEN: u8 = 0;
// const ERR_USERNAME_TOO_LONG: u8 = 1;
// const ERR_USERNAME_CONTAINS_SPACES: u8 = 2;
// const ERR_UNKNOWN_USER_PRIVATE_MSG: u8 = 3;
// const ERR_UNKNOWN_MESSAGE_FORMAT: u8 = 4;

fn main() {
    let test_input = get_input();
}

fn get_input() -> String {
    let mut user_input = String::new();
    io::stdin()
        .read_line(&mut user_input)
        .expect("Could not read it :(");

    user_input
}
