use std::io;

// const HOSTPORT: &str = "0.0.0.0:8080";

// const ERR_USERNAME_TAKEN: u8 = 0;
// const ERR_USERNAME_TOO_LONG: u8 = 1;
// const ERR_USERNAME_CONTAINS_SPACES: u8 = 2;
// const ERR_UNKNOWN_USER_PRIVATE_MSG: u8 = 3;
// const ERR_UNKNOWN_MESSAGE_FORMAT: u8 = 4;

#[derive(Debug)]
enum MessageType {
    Register,
    PublicMessage,
    PrivateMessage,
    Exit,
    Invalid,
}

// WARNING: Not used
pub fn parse_message(message: &str) -> MessageType {
    let mut parts = message.split_whitespace();
    match parts.next() {
        Some(word) => {
            if word == "REG" {
                MessageType::Register
            } else if word == "PUB" {
                MessageType::PublicMessage
            } else if word == "PRIV" {
                MessageType::PrivateMessage
            } else if word == "EXIT" {
                MessageType::Exit
            } else {
                MessageType::Invalid
            }
        }
        None => MessageType::Invalid,
    }
}

pub fn get_input() -> String {
    let mut user_input = String::new();
    io::stdin()
        .read_line(&mut user_input)
        .expect("Could not read it :(");

    user_input
}
