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
        spawn(proc() {
        println!("handle_client");
        
        let mut buf: [u8,..1000000]=[0,..1000000];
        let window_name = "TestGUI";

        let result = opencv::named_window(window_name, 0);
    
    
        let mut image = opencv::Image::new(opencv::load_image("ZuhlkeLogo.gif", opencv::CV_LOAD_IMAGE_UNCHANGED));
    
        loop {
            let res = stream.read(buf);
            if res.is_ok() {
             let mut buf_vec = buf.slice(0, res.unwrap()).to_vec();
                
            
               image.decode_image(&mut buf_vec);
        
               opencv::show_image(window_name, image.get_image());
            }
            else {
                break;
            }
        }
        println!("... disconnect");
        })
    }

}



fn main() {

	let chat :ChatListener = IChatListener::new();
	chat.start(7777);

	
}
