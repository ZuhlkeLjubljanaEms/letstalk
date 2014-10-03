extern crate serialize;
extern crate libc;
extern crate opencv;

use serialize::base64::ToBase64;
use std::io::TcpStream;
use std::os;
//use common::message;
//use std::io;

#[path = "..\\common"]

fn main() {

	 let args = os::args();
	 if args.len() < 2 {
	 	println!("use: client addr");
	 	return;
	 }
	let addr = args[1].as_slice();

	let mut socket = TcpStream::connect(addr, 7777).unwrap();
	
	
	let q= String::from_str("quit");

    let window_name = "Client GUI";

    let result = opencv::named_window(window_name, 0);
    
    
    let mut image = opencv::Image::new(opencv::load_image("ZuhlkeLogo.gif", opencv::CV_LOAD_IMAGE_UNCHANGED));
    let loaded_image = image.get_image();
    
    //image.encoded_image();
    
    let camera = opencv::Camera::new();

    loop {
        let mut camera_image = camera.grab_image();
        
        let mut encoded_image = camera_image.encoded_image();
        
        opencv::show_image(window_name, camera_image.get_image());
        
        let key = opencv::wait_key(20);
        if key > -1 {
            println!("Key {}", key);
            break;
        }
    	//print!("Type message or 'quit' to finish >");
		//let mut reader = std::io::stdin();
		//let l = reader.read_line();
		
		//let sl = l.unwrap();
		
	    //let ssl = sl.replace("\n", "").replace("\r", "");
	    //if ssl == q {
	    //   break;
	    //}
	    
		//socket.write(ssl.into_bytes().as_slice());
		//let webcam_message = common::message::Message {message_type: common::message::webcam, message_data: common::message::Webcam(common::message::WebcamMessage {webcam_data: "123".to_string().into_bytes().as_slice().to_base64(serialize::base64::MIME)})};
		socket.write(encoded_image.as_slice());
//	match client_listResponseMessage.messageData
	}
	
	println!("bye.");
	
//	let response = socket.read_to_end();
}