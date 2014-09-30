extern crate core;
use std::io::{TcpListener, TcpStream};
use std::io::{Acceptor, Listener};

fn main() {

	let listener = TcpListener::bind("127.0.0.1", 7777);
	
	// bind the listener to the specified address
	let mut acceptor = listener.listen();
	
	fn handle_client(mut stream: TcpStream) {
		println!("handle_client");
		
		let mut buf = [0];
		let mut ss = String::new();
		loop {
			let res = stream.read(buf);
			//println!("res:{}", res);
			if res.is_ok() {
				let sz = buf.len();	
				
				for x in range(0, sz) {
					//print!("{}.", buf[x]);
					//let s = String::from_byte(buf[x]);
					//ss.append(&s);
					unsafe {
					ss.push_byte(buf[x]);
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