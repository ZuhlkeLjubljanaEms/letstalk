extern crate core;


use std::io::{TcpListener, TcpStream};
use std::io::{Acceptor, Listener};


fn main() {

	let listener = TcpListener::bind("0.0.0.0", 7777);
	
	// bind the listener to the specified address
	let mut acceptor = listener.listen();
	
	fn handle_client(mut stream: TcpStream) {
		println!("handle_client");
		
		let mut buf = [0];
		let mut ss = String::new();
		loop {
			let res = stream.read(buf);
			if res.is_ok() {
				for x in buf.iter() {
					unsafe {
						ss.push_byte(*x);
					}
				}
			}
			else {
				break;
			}
		}
		println!("< end {}", ss);
		   // ...
	}
	// accept connections and process them, spawning a new tasks for each one
	for stream in acceptor.incoming() {
	    match stream {
	        Err(e) => { /* connection failed */ }
	        Ok(stream) => spawn(proc() {
	            // connection succeeded
	            handle_client(stream)
	        })
	    }
	}

	// close the socket server
	drop(acceptor);
}