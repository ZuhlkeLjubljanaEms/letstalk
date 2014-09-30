use std::io::TcpStream;
use std::os;
//use std::io;

fn main() {

	 let args = os::args();
	 if args.len() < 2 {
	 	println!("use: client addr");
	 }
	let addr = args[1].as_slice();

	let mut socket = TcpStream::connect(addr, 7777).unwrap();
	
	
	let q= String::from_str("quit");

    loop {
    	print!("Type message or 'quit' to finish >");
		let mut reader = std::io::stdin();
		let l = reader.read_line();
		
		let sl = l.unwrap();
		
	    let ssl = sl.replace("\n", "").replace("\r", "");
	    if ssl == q {
	       break;
	    }
	    
		socket.write(ssl.into_bytes().as_slice());
	}
	
	println!("bye.");
	
//	let response = socket.read_to_end();
}