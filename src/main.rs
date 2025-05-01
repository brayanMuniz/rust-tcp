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
    }
}
