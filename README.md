# Course Deliverable 1: TCP Socket Programming

**Course:** Essentials of Health Informatics

## About

This project implements a TCP client-server application in Rust as part of Course Deliverable 1 for Essentials of Health Informatics.

The client accepts an integer from the keyboard and sends its name and number to the server. The server responds with its name and a number of its choice. Both programs display the names, numbers, and their sum.

## Features

- TCP communication using `std::net`
- Client sends its name and an integer between 1 and 100
- Server responds with its name and an integer
- Both programs display the numbers and their sum
- Server terminates when it receives an out-of-range integer
- Informative console messages for sending, receiving, and connection status

## Project Structure

```text
simple-socket-server/
├── Cargo.toml
└── src/
    ├── client.rs
    └── server.rs
```

## Requirements

- Rust
- Cargo

## How to Run

Open two terminals in the project directory.

### 1. Start the server

```bash
cargo run --bin server
```

### 2. Start the client

In the second terminal, run:

```bash
cargo run --bin client
```

Enter an integer when prompted. The client will connect to the server and display the results.

## Example

If the client enters `25` and the server uses `50`:

```text
Client number: 25
Server number: 50
Sum: 75
```

## Technologies

- Rust
- TCP sockets
- Cargo
