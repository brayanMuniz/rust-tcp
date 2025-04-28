mod server;

use server::{MessageType, Server};

fn main() {
    // Start the "server"
    let mut server = Server::build();
    server
        .run()
        .expect("Was not able to bind to port number :(");

    println!("Commands: ");
    println!("REG: username");
    println!("PUB: message");
    println!("PRIV: username message");
    println!("EXIT");

    println!("\n--- Processing Commands ---");

    loop {
        // Test input
        let input_string = server::get_input();
        let message_type = server::parse_message(&input_string);

        match message_type {
            MessageType::Register(username) => {
                println!("Parsed command: Register");
                let err = server.add_user(&username);
                match err {
                    Ok(_) => println!("Added user: {}", username),
                    Err(err) => match err {
                        server::RegisterError::UsernameTaken => {
                            println!("username: {username} taken!")
                        }
                        server::RegisterError::UsernameTooLong => println!("username too long"),
                        server::RegisterError::UsernameContainsSpaces => {
                            println!("username has spaces!")
                        }
                    },
                }
            }

            MessageType::PublicMessage(message) => {
                println!("Parsed command: Public Message");
                println!("Message: {}", message);
            }

            MessageType::PrivateMessage(receiver, message) => {
                println!("Parsed command: Private Message");
                println!("Receiver: {}", receiver);
                println!("Message: {}", message);
            }

            MessageType::Exit => {
                println!("Exit");
                break;
            }

            MessageType::Invalid => {
                println!("Invalid Message");
            }
        }
    }
}
