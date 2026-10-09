use std::io::{Read, Write};
use std::net::{TcpListener, Shutdown};

fn main() {
    // Server name
    let server_name = "Server of Ranveer".to_string();

    // Server chooses a number between 1 and 100
    let server_number = 50;

    // Create TCP server socket
    let listener = TcpListener::bind("127.0.0.1:5001")
        .expect("Failed to start server");

    println!("Server started.");
    println!("Server name: {}", server_name);
    println!("Waiting for a client...");

    // Accept clients
    for connection in listener.incoming() {
        let mut stream = match connection {
            Ok(stream) => stream,
            Err(error) => {
                println!("Connection error: {}", error);
                continue;
            }
        };

        println!("Client connected.");

        // Receive message from client
        let mut buffer = [0; 1024];

        let bytes_read = stream
            .read(&mut buffer)
            .expect("Failed to receive message");

        if bytes_read == 0 {
            println!("Client closed the connection.");
            continue;
        }

        let message = String::from_utf8_lossy(&buffer[..bytes_read]);

        println!("Received message: {}", message);

        // Message format: client name | client number
        let parts: Vec<&str> = message.trim().split('|').collect();

        let client_name = parts[0];

        let client_number: i32 = parts[1]
            .parse()
            .expect("Client number is not an integer");

        // Display required information
        println!("Client name: {}", client_name);
        println!("Server name: {}", server_name);
        println!("Client number: {}", client_number);

        // Check whether number is between 1 and 100
        if client_number < 1 || client_number > 100 {
            println!("Client number is out of range.");
            println!("Server is terminating.");

            stream.shutdown(Shutdown::Both).ok();
            break;
        }

        // Calculate sum
        let sum = client_number + server_number;

        println!("Server number: {}", server_number);
        println!("Sum: {}", sum);

        // Send server name and server number to client
        let response = format!("{}|{}", server_name, server_number);

        println!("Sending server name and server number...");

        stream
            .write_all(response.as_bytes())
            .expect("Failed to send response");

        println!("Response sent.");
        println!("Connection closed.");
        println!("Waiting for another client...");
        println!();
    }

    println!("Server terminated.");
}