extern crate core;
extern crate ip_listener;
extern crate serialize;

use serialize::base64::FromBase64;
use std::io::TcpStream;

use ip_listener::IChatListener;

use common::message;

#[path = "../common"]
mod common {pub mod message;}

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
                
                println!("{}",ss);
                let decoded: message::Message = serialize::json::decode(ss.as_slice()).unwrap();
                match decoded
                {
                	message::Webcam(w) => println!("Message: {}", w.webcam_data.as_slice().from_base64()),
                	_	=> ()
                }
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
