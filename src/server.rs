use std::{collections::HashMap, io};

#[derive(Debug)]
pub enum ServerError {
    UsernameTaken,
    UsernameTooLong,
    UsernameContainsSpaces,
    UnknownUserForPMSG,
    UnknownMessageFormat,
}

#[derive(Debug)]
pub struct Server {
    hostport: String,
    clients: HashMap<String, bool>,
}

impl Server {
    pub fn build(port: String) -> Self {
        Server {
            hostport: port,
            clients: HashMap::new(), // username : network number
        }
    }

    pub fn add_user(&mut self, username: &str) -> Result<(), ServerError> {
        let mut taken = false;
        for (key, _) in self.clients.iter() {
            if key == username {
                taken = true
            }
        }

        if !taken {
            self.clients.insert(username.to_string(), true);
            return Ok(());
        }
        Err(ServerError::UsernameTaken)
    }
}

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
