use std::io::TcpStream;
use std::os;

fn main() {

	let mut addr = "127.0.0.1";
	 let args = os::args();
	 if args.len() > 1 {
	 	addr = args[1].as_slice();
	 }

	let mut socket = TcpStream::connect(addr, 7777).unwrap();
	
	let buf:[u8, ..6] = [65, 66, 67, 68, 69, 70];
	socket.write(buf);
//	let response = socket.read_to_end();
}