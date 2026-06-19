use std::io::Read;
use std::os::unix::net::UnixStream;

fn main() {
    // Connect to the local sender through the Unix socket.
    let mut stream =
        UnixStream::connect("/tmp/shredstream.sock").expect("Failed to connect to socket");

    // Read the fixed-size prefix that describes the message boundary.
    let mut length_bytes = [0u8; 4];
    stream
        .read_exact(&mut length_bytes)
        .expect("Failed to read message length");

    // Decode the prefix and reserve exactly enough space for the message.
    let message_length = u32::from_be_bytes(length_bytes);
    let mut message_bytes = vec![0u8; message_length as usize];

    // Read one complete message using the announced length.
    stream
        .read_exact(&mut message_bytes)
        .expect("Failed to read message");

    // Validate the received bytes as UTF-8 text.
    let message = String::from_utf8(message_bytes).expect("Message was not valid UTF-8");

    println!("Received length: {message_length}");
    println!("Received: {message}");
}
