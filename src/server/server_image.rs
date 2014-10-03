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
    
        loop {
            let res = stream.read(buf);
            if res.is_ok() {
            let ss = res.unwrap();
             let mut buf_vec = buf.slice(0, ss).to_vec();
                
                if ss < 1000000
                {
            
               		image.decode_image(&mut buf_vec);
        
            	   opencv::show_image(window_name, image.get_image());
            	  }
            	   let key = opencv::wait_key(20);
			        if key > -1 {
			            println!("Key {}", key);
			            break;
			        }
            }
            else {
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
