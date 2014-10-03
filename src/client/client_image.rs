extern crate serialize;
extern crate libc;
extern crate opencv;

use serialize::base64::ToBase64;
use std::io::TcpStream;
use std::os;
//use common::message;
//use std::io;

#[path = "../common"]

fn main() {

	 let args = os::args();
	 if args.len() < 2 {
	 	println!("use: client addr");
	 	return;
	 }
	let addr = args[1].as_slice();

	let mut socket = TcpStream::connect(addr, 7777).unwrap();

    let window_name = "Client GUI";

    let result = opencv::named_window(window_name, 0);
    
    
    let mut image = opencv::Image::new(opencv::load_image("ZuhlkeLogo.gif", opencv::CV_LOAD_IMAGE_UNCHANGED));
    let loaded_image = image.get_image();
    
    
    let camera = opencv::Camera::new();

    loop {
        let mut camera_image = camera.grab_image();
        
        let mut encoded_image = camera_image.encoded_image();
        
        opencv::show_image(window_name, camera_image.get_image());
        
        let key = opencv::wait_key(2000);
        if key > -1 {
            println!("Key {}", key);
            break;
        }
	    
		socket.write(encoded_image.as_slice());
	}
	
	println!("bye.");
}