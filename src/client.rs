use std::io::{self, Read, Write};
use std::net::TcpStream;

fn main() {
    // Client name
    let client_name = "Client of Ranveer".to_string();

    println!("Client started.");

    // Accept integer from keyboard
    let client_number: i32;

    loop {
        print!("Enter an integer between 1 and 100: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        match input.trim().parse::<i32>() {
            Ok(number) => {
                client_number = number;
                break;
            }
            Err(_) => {
                println!("Please enter an integer.");
            }
        }
    }

    println!("Client number: {}", client_number);

    // Open TCP connection to server
    println!("Connecting to server...");

    let mut stream = TcpStream::connect("127.0.0.1:5001")
        .expect("Failed to connect to server");

    println!("Connected to server.");

    // Send client name and number
    let message = format!("{}|{}", client_name, client_number);

    println!("Sending message...");
    println!("Client name: {}", client_name);
    println!("Client number: {}", client_number);

    stream
        .write_all(message.as_bytes())
        .expect("Failed to send message");

    println!("Message sent.");
    println!("Waiting for server reply...");

    // Receive server name and server number
    let mut buffer = [0; 1024];

    let bytes_read = stream
        .read(&mut buffer)
        .expect("Failed to receive server reply");

    if bytes_read == 0 {
        println!("Server closed the connection.");
        return;
    }

    let response = String::from_utf8_lossy(&buffer[..bytes_read]);

    println!("Server reply received.");

    // Response format: server name | server number
    let parts: Vec<&str> = response.trim().split('|').collect();

    let server_name = parts[0];

    let server_number: i32 = parts[1]
        .parse()
        .expect("Server number is not an integer");

    // Calculate sum
    let sum = client_number + server_number;

    // Display required information
    println!();
    println!("Client name: {}", client_name);
    println!("Server name: {}", server_name);
    println!("Client number: {}", client_number);
    println!("Server number: {}", server_number);
    println!("Sum: {}", sum);

    println!("Client terminating.");
}