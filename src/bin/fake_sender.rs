use std::fs;
use std::io::Write;
use std::os::unix::net::UnixListener;

const SOCKET_PATH: &str = "/tmp/shredstream.sock";

fn main() {
    // Remove a stale socket path left by an earlier run.
    let _ = fs::remove_file(SOCKET_PATH);

    // Create the Unix socket that receivers can connect to.
    let listener =
        UnixListener::bind(SOCKET_PATH).expect("Failed to create Unix socket");

    println!("Waiting for receiver...");

    // Wait until one receiver establishes a connection.
    let (mut stream, _address) =
        listener.accept().expect("Failed to accept connection");

    // Prepare the message and its fixed-width length prefix.
    let message = "hello from fake sender";
    let message_bytes = message.as_bytes();
    let message_length = u32::try_from(message_bytes.len()).expect("Message is too large");

    // Send the message boundary before sending the message itself.
    stream
        .write_all(&message_length.to_be_bytes())
        .expect("Failed to send message length");

    // Send the complete message payload.
    stream
        .write_all(message_bytes)
        .expect("Failed to send message");

    println!("Sent length: {message_length}");
    println!("Message sent: {message}");
}
