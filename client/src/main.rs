use std::net::TcpStream;
use std::io::{self, Write, Read};

fn main() {
    // Connect to the "server" service in the Docker network
    match TcpStream::connect("server:8080") {
        Ok(mut stream) => {
            println!("Connected to the server!");

            let mut input = String::new();
            loop {
                input.clear();
                println!("Enter command (CREATE, LIST, JOIN, LEAVE, SEND):");
                io::stdin().read_line(&mut input).expect("Failed to read input");
                let message = input.trim();
                if message == "quit" {
                    break;
                }

                // Send the message to the server
                stream.write(message.as_bytes()).expect("Failed to send message");

                // Read the response from the server
                let mut buffer = [0; 512];
                match stream.read(&mut buffer) {
                    Ok(size) => {
                        let response = String::from_utf8_lossy(&buffer[..size]);
                        println!("Server: {}", response);
                    }
                    Err(_) => {
                        println!("Failed to receive response from server.");
                    }
                }
            }
        }
        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }
}
// ffs