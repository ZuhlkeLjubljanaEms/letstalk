extern crate core;
extern crate ip_listener;

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
        
        let mut buf: [u8,..1000]=[0,..1000];
        let mut ss = String::new();
        loop {
            let res = stream.read(buf);
//          println!("read:{} - sz:{}", res, res.clone().unwrap());
            if res.is_ok() {
            
                for x in range(0, res.unwrap()) {
//                 println!("byte:{}", x);
                    unsafe {
                        ss.push_byte(buf[x]);
                    }
                }
                
                println!("Message: {}", ss);
                ss = String::new();
                
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
