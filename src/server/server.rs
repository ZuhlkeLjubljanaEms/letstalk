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
                
                //println!("{}",ss);
                let decoded: message::Message = serialize::json::decode(ss.as_slice()).unwrap();
                match decoded
                {
                	message::SignIn(sign_in_msg) => println!("User {} signing in.", sign_in_msg.user_name.as_slice()),
                    message::AddressRequest(ip_request_msg) => println!("IP address requested for user {}.", ip_request_msg.user_name.as_slice()),
                    message::Webcam(w) => println!("Webcam Message: {}", w.webcam_data.as_slice().from_base64()),
                	_ => println!("message not recognized: {}", ss),
                }
                ss = String::new();
                
            }
            else {   // res is an error.
                //if (res.Err.kind != std::io::EndOfFile) {
                //    println!("error: {}", res);
                //}
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
