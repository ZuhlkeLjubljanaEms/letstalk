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
    
    	let mut buf_vec: Vec<u8> = Vec::new();
    	let res = stream.read(buf);
    	let ss = res.unwrap();
    	buf_vec.push_all(buf.slice(0, ss));
        loop {
        	while (buf_vec.len() < 4)
        	{
        		let res = stream.read(buf);
        		let ss = res.unwrap();
        		buf_vec.push_all(buf.slice(0, ss));
        	}
        		
            	let mut size: u32 = 0;
        		size+=buf_vec[0] as u32;
        		size+=buf_vec[1] as u32 *0x100;
        		size+=buf_vec[2] as u32 *0x10000;
        		size+=buf_vec[3] as u32 *0x1000000;
        		
        		println!("{}",size);
            	
             	
        
             	
        		while (buf_vec.len() < (size as uint))
        		{
        			let res = stream.read(buf);
        			let ss = res.unwrap();
        			buf_vec.push_all(buf.slice(0, ss));
        		}   
        		
        		let mut image_vector = buf_vec.slice_mut(4, size as uint + 4 ).to_vec();
        		
        		let length=buf_vec.len();
        		if (size as uint != length)
        		{
        			println!("Before Slice");
        			buf_vec = buf_vec.slice_mut(size as uint + 4 ,length).to_vec();
        		}
        		else
        		{
        			buf_vec.clear();
        		}
        			
            
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
