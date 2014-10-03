extern crate core;
extern crate ip_listener;
extern crate libc;
extern crate opencv;

use std::io::TcpStream;

use ip_listener::IChatListener;




struct ChatListener {    dummy : int}


impl IChatListener for ChatListener {

    fn new () -> ChatListener {
        ChatListener{ 
        dummy : 3 
        }
    }

    fn handle_client(&self, mut stream: TcpStream) {
        let window_name = "Server GUI";
        let result = opencv::named_window(window_name, 0);
        //spawn(proc() {
        println!("handle_client");
        
        let mut buf: [u8,..1000000]=[0,..1000000];
        
        let mut image = opencv::Image::new(opencv::load_image("ZuhlkeLogo.gif", opencv::CV_LOAD_IMAGE_UNCHANGED));
    
    	let res = stream.read(buf);
    	let mut buf_index = 0;
    	let mut buf_size = res.unwrap();
    	
        loop {
            	let mut size: u32 = 0;
        		size+=buf[buf_index+0] as u32;
        		size+=buf[buf_index+1] as u32 *0x100;
        		size+=buf[buf_index+2] as u32 *0x10000;
        		size+=buf[buf_index+3] as u32 *0x1000000;
        		buf_index += 4;
        		
        		println!("{}",size);
                 	
                let mut image_vector = Vec::with_capacity(size as uint);
                
        		while (buf_index + size as uint >= buf_size) {
        		
        		  image_vector.push_all(buf.slice_mut(buf_index, buf_size));
        		  size -= (buf_size-buf_index) as u32;
        		  let res = stream.read(buf);
        		  buf_size = res.unwrap();
        		  buf_index = 0; 
        		}
        		
        		image_vector.push_all(buf.slice_mut(buf_index, size as uint));
        		buf_index += size as uint;
        		
                image.decode_image(&mut image_vector);
        
            	opencv::show_image(window_name, image.get_image());
            	let key = opencv::wait_key(20);
			    if key > -1 {
			        println!("Key {}", key);
			        break;
			    }
        }
        println!("... disconnect");
      //  })
    }

}



fn main() {

	let chat :ChatListener = IChatListener::new();
	chat.start(7777);

	
}
