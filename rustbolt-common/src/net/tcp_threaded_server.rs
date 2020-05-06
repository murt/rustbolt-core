use std::collections::HashMap;
use std::io::Result;
use std::io::{Read, Write};
use std::net::{Shutdown, TcpListener, TcpStream};
use std::thread;

use uuid::Uuid;

pub struct TcpThreadedServer {
    listener: TcpListener,
    connections: HashMap<String, &'static TcpThreadedClient>,
}

impl TcpThreadedServer {
    /// Create a new threaded TCP server and immediately start listening.
    pub fn start(host: String, port: u16) -> TcpThreadedServer {
        // Initialise the server socket and list of threads
        let mut server = TcpThreadedServer {
            listener: TcpListener::bind(format!("{}:{}", host, port)).unwrap(),
            connections: HashMap::new(),
        };

        // Start listening on the socket (indefinitely)
        for stream in server.listener.incoming() {
            match stream {
                // Accept the new connection and start a thread by moving into the provided handler
                Ok(stream) => {
                    println!("New Connection: {}", stream.peer_addr().unwrap());

                    // Create a new Tcp client wrapper
                    let client = TcpThreadedClient::new(stream);
                    // Store the client in the client map
                    server.connections.insert(client.id.clone(), &client);
                    // Start the thread
                    client.start();
                }
                // Error out with just a log message for now
                Err(e) => {
                    println!("Error: {}", e);
                }
            }
        }

        return server;
    }
}

pub struct TcpThreadedClient {
    id: String,
    stream: TcpStream,
    join_thread: Option<thread::JoinHandle<()>>,
}

impl TcpThreadedClient {
    /// Constructor that builds a new tcp client with an automated UUID
    pub fn new(stream: TcpStream) -> TcpThreadedClient {
        return TcpThreadedClient {
            id: Uuid::new_v4().to_string(),
            stream,
            join_thread: None,
        };
    }

    /// Starts the thread
    pub fn start(&mut self) {
        self.join_thread = Some(thread::spawn(move || {}));
    }

    /// Waits for the thread to finish
    pub fn join(&mut self) -> TcpThreadedClient {
        match &self.join_thread {
            None => *self,
            Some(jt) => match jt.join() {
                Ok(_) => *self,
                Err(e) => {
                    println!("Error joining thread: {}", self.id.to_string());
                    *self
                }
            },
        }
    }
}
