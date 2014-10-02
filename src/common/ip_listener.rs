#![crate_type = "lib"]
#![crate_name = "ip_listener"]


use std::io::{TcpListener, TcpStream};
use std::io::{Acceptor, Listener};

pub trait IChatListener {
    fn new() ->Self;

    fn handle_client(&self, mut stream: TcpStream);
    
    fn start(&self, port : u16) {
        let listener = TcpListener::bind("0.0.0.0", port);
        
        // bind the listener to the specified address
        let mut acceptor = listener.listen();
        // accept connections and process them, spawning a new tasks for each one
        for stream in acceptor.incoming() {
            match stream {
                Err(e) => { /* connection failed */ }
                Ok(stream) => self.handle_client(stream)
            }
        }
        // close the socket server
        drop(acceptor);
    }
}

