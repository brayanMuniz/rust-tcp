mod server;

use server::{MessageType, Server};

// TODO: For now the server will just be inputs to the console
// Have all the functionality working well (parsing the message, username check etx)
fn main() {
    // Start the "server"
    let server = Server::new();

    // Test input
    let input_string = server::get_input();
    let message_type = server::parse_message(&input_string);

    println!("\n--- Processing Command ---");

    match message_type {
        MessageType::Register(username) => {
            println!("Parsed command: Register");
            println!("Username: {}", username);
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
        }
        MessageType::Invalid => {
            println!("Invalid Message");
        }
    }
}
