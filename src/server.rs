use std::net::{TcpListener, TcpStream};
use std::{collections::HashMap, io, io::Read, io::Write};

const ADDR: &str = "127.0.0.1:8080";

#[derive(Debug)]
pub enum MessageError {
    UnknownUserForPMSG,
    UnknownMessageFormat,
}

#[derive(Debug)]
pub enum RegisterError {
    UsernameTaken,
    UsernameTooLong,
    UsernameContainsSpaces,
}

// TODO: update this to have the connection as the value
#[derive(Debug)]
pub struct Server {
    clients: HashMap<String, TcpStream>,
}

impl Server {
    pub fn build() -> Self {
        Server {
            clients: HashMap::new(), // username : network number
        }
    }

    pub fn run(&mut self) -> std::io::Result<()> {
        let listener = TcpListener::bind(ADDR)?;
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => self.handle_client(stream)?, // Propogate error since its same type
                Err(_) => println!("Connection bad!"),
            }
        }

        Ok(())
    }

    // TODO: modify to use the new connection hashmap
    // WARNING: not able to handle disconnect
    // WARNING: not able to get a new connection, not like go func
    pub fn handle_client(&mut self, mut stream: TcpStream) -> std::io::Result<()> {
        println!("Got a connection");

        loop {
            let mut buf = vec![0; 128];
            let amount_read = stream.read(&mut buf)?;
            if amount_read == 0 {
                break;
            } else if amount_read > 0 {
                match String::from_utf8(buf) {
                    // need to 0..amount_read to not get extra bytes
                    Ok(client_message) => match parse_message(&client_message[0..amount_read]) {
                        MessageType::Register { username } => {
                            println!("Making a new user with username: {username}");
                            // NOTE: would I have to "manage" this clone?
                            let stream_clone = stream.try_clone()?;
                            match self.add_user(&username, stream_clone) {
                                Ok(_) => {
                                    stream.write(b"Able to register with that username")?;
                                }
                                Err(err) => match err {
                                    RegisterError::UsernameTaken => {
                                        stream.write(b"Username taken")?;
                                    }
                                    RegisterError::UsernameTooLong => {
                                        stream.write(b"username too long")?;
                                    }
                                    RegisterError::UsernameContainsSpaces => {
                                        stream.write(b"username contains spaces")?;
                                    }
                                },
                            }
                        }
                        MessageType::PublicMessage { message } => {
                            println!("Public Message with {message}")
                        }
                        MessageType::PrivateMessage { receiver, message } => {
                            println!("Private message {receiver}, with message: {message}")
                        }
                        MessageType::Exit => {
                            println!("Exit");
                        }
                        MessageType::Invalid => {
                            println!("Invalid message type: {client_message}");
                        }
                    },
                    Err(_) => println!("Not valid!"),
                }
            }
            stream.write(&[1])?;
        }

        Ok(())
    }

    pub fn add_user(&mut self, username: &str, stream: TcpStream) -> Result<(), RegisterError> {
        if username.len() > 16 {
            return Err(RegisterError::UsernameTooLong);
        }

        if username.contains(" ") {
            return Err(RegisterError::UsernameContainsSpaces);
        }

        for (key, _) in self.clients.iter() {
            if key == username {
                return Err(RegisterError::UsernameTaken);
            }
        }

        self.clients.insert(username.to_string(), stream);
        Ok(())
    }

    // NOTE: since there is not a current network connection, this will just check for username
    pub fn pmsg(&mut self, username: &str, message: &str) -> Result<(), MessageError> {
        if !self.clients.contains_key(username) {
            return Err(MessageError::UnknownUserForPMSG);
        }

        Ok(())
    }
}

#[derive(Debug)]
pub enum MessageType {
    Register { username: String },
    PublicMessage { message: String },
    PrivateMessage { receiver: String, message: String },
    Exit,
    Invalid,
}

pub fn parse_message(message: &str) -> MessageType {
    let mut parts = message.trim().split_whitespace();

    match parts.next() {
        Some(word) => {
            println!("Command: {word}");
            // REG: name
            if word == "REG:" {
                let username = parts.next();
                let extra = parts.next();
                match (username, extra) {
                    (Some(username_str), None) => MessageType::Register {
                        username: username_str.to_string(),
                    },
                    (Some(_), Some(_)) => MessageType::Invalid,
                    (None, None) => MessageType::Invalid,
                    (None, Some(_)) => MessageType::Invalid,
                }
            }
            // PUB: message
            else if word == "PUB:" {
                let message_content = parts.collect::<Vec<&str>>().join(" ");
                if message_content.is_empty() {
                    MessageType::Invalid
                } else {
                    MessageType::PublicMessage {
                        message: message_content,
                    }
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
                                MessageType::PrivateMessage {
                                    receiver: receiver.to_string(),
                                    message: message_content,
                                }
                            }
                        }
                    }
                    None => MessageType::Invalid,
                }
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
