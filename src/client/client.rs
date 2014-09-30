use std::io::TcpStream;
use std::os;

fn main() {

	 let args = os::args();
	 if args.len() < 3 {
	 	println!("use: client addr message");
	 }
	let addr = args[1].as_slice();

	let mut socket = TcpStream::connect(addr, 7777).unwrap();
	socket.write(args[2].clone().into_bytes().as_slice());
	
	
//	let response = socket.read_to_end();
}