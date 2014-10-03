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

    let window_name = "Client GUI";

    let result = opencv::named_window(window_name, 0);
    
    
    let mut image = opencv::Image::new(opencv::load_image("ZuhlkeLogo.gif", opencv::CV_LOAD_IMAGE_UNCHANGED));
    let loaded_image = image.get_image();
    
    
    let camera = opencv::Camera::new();

    loop {
        let mut camera_image = camera.grab_image();
        
        let mut encoded_image = camera_image.encoded_image();
        
        opencv::show_image(window_name, camera_image.get_image());
        
        let key = opencv::wait_key(40);
        if key > -1 {
            println!("Key {}", key);
            break;
        }
	    
	    let size: u32 = encoded_image.len() as u32;
	    let mut size_chunks: [u8, ..4] = [0, ..4];
	    size_chunks[0] = size as u8;
	    size_chunks[1] = (size / 0x100) as u8;
	    size_chunks[2] = (size / 0x10000) as u8;
        size_chunks[3] = (size / 0x1000000) as u8;
	    
	    //println!("------> SIZE: {} {} {} {}", size_chunks[0], size_chunks[1], size_chunks[2], size_chunks[3]);
	    socket.write(size_chunks);
		socket.write(encoded_image.as_slice());
	}
	
	println!("bye.");
}