mod server;

// TODO: For now the server will just be inputs to the console
// Have all the functionality working well (parsing the message, username check etx)
fn main() {
    let input = server::get_input();
    println!("Please enter your name");
    println!("Input in main.rs {input}");
    println!("Hello, world!");
}
