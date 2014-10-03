extern crate serialize;

use serialize::base64::ToBase64;
use std::io::TcpStream;
use std::os;
use common::message;
//use std::io;

#[path = "..\\common"]
mod common {pub mod message;}

fn main() {

	 let args = os::args();
	 if args.len() < 2 {
	 	println!("use: client addr");
	 }
	let addr = args[1].as_slice();

	let mut socket = TcpStream::connect(addr, 7777).unwrap();
	
	
	let q= String::from_str("quit");

    //loop {
    	//print!("Type message or 'quit' to finish >");
		//let mut reader = std::io::stdin();
		//let l = reader.read_line();
		
		//let sl = l.unwrap();
		
	    //let ssl = sl.replace("\n", "").replace("\r", "");
	    //if ssl == q {
	    //   break;
	    //}
	    
		//socket.write(ssl.into_bytes().as_slice());
		let webcam_message = common::message::Webcam(common::message::WebcamMessage {webcam_data: "123".to_string().into_bytes().as_slice().to_base64(serialize::base64::MIME)});
		socket.write(webcam_message.convert_to_json().into_bytes().as_slice());
//	match client_listResponseMessage.messageData
	//}
	
	println!("bye.");
	
//	let response = socket.read_to_end();
}