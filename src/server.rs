use std::io;

// const HOSTPORT: &str = "0.0.0.0:8080";

// const ERR_USERNAME_TAKEN: u8 = 0;
// const ERR_USERNAME_TOO_LONG: u8 = 1;
// const ERR_USERNAME_CONTAINS_SPACES: u8 = 2;
// const ERR_UNKNOWN_USER_PRIVATE_MSG: u8 = 3;
// const ERR_UNKNOWN_MESSAGE_FORMAT: u8 = 4;

#[derive(Debug)]
pub enum MessageType {
    Register(String),
    PublicMessage(String),
    PrivateMessage(String, String),
    Exit,
    Invalid,
}

pub fn parse_message(message: &str) -> MessageType {
    let mut parts = message.split_whitespace();
    match parts.next() {
        Some(word) => {
            // REG: name
            if word == "REG:" {
                match parts.next() {
                    Some(username) => {
                        if parts.next().is_none() {
                            MessageType::Register(username.to_string())
                        } else {
                            MessageType::Invalid
                        }
                    }
                    None => MessageType::Invalid,
                }
            }
            // PUB: message
            else if word == "PUB:" {
                let message_content = parts.collect::<Vec<&str>>().join(" ");
                if message_content.is_empty() {
                    MessageType::Invalid
                } else {
                    MessageType::PublicMessage(message_content)
                }
            }
            // PRIV: username message
            else if word == "PRIV:" {
                match parts.next() {
                    Some(receiver) => {
                        if parts.next().is_none() {
                            MessageType::Invalid
                        } else {
                            let message_content = parts.collect::<Vec<&str>>().join(" ");
                            if message_content.is_empty() {
                                MessageType::Invalid
                            } else {
                                MessageType::PrivateMessage(receiver.to_string(), message_content)
                            }
                        }
                    }
                    None => MessageType::Invalid,
                }
            } else if word == "EXIT:" {
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
