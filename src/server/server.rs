extern crate core;


use std::io::{TcpListener, TcpStream};
use std::io::{Acceptor, Listener};


fn main() {

	let listener = TcpListener::bind("0.0.0.0", 7777);
	
	// bind the listener to the specified address
	let mut acceptor = listener.listen();
	
	fn handle_client(mut stream: TcpStream) {
		println!("handle_client");
		
		let mut buf: [u8,..1000]=[0,..1000];
		let mut ss = String::new();
		loop {
			let res = stream.read(buf);
//			println!("read:{} - sz:{}", res, res.clone().unwrap());
			if res.is_ok() {
			
				for x in range(0, res.unwrap()) {
//				   println!("byte:{}", x);
					unsafe {
						ss.push_byte(buf[x]);
					}
				}
				
				println!("Message: {}", ss);
				ss = String::new();
				
			}
            else {
				break;
			}
		}
		println!("... disconnect");
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
