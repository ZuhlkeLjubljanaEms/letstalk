#![feature(phase)]
/** ************************************************************************************************
 * \file      letstalk.rs
 * \ingroup   Let's_Talk!
 * \brief     
 * \details   
 * \author    Jeremy Hannon <hje@zuhlke.com>
 * \date      29th September 2014
 **************************************************************************************************/

#[phase(plugin, link)] extern crate log;

extern crate serialize;             // TODO: why is this required here?  Used in file_io file.
use common::message;
use std::io::TcpStream;
use std::os;
pub mod file_io;
pub mod client_information;
#[path = "..\\common"]
mod common {pub mod message;}

static USER_INFO_FILENAME: &'static str = "userInfo.json";
static FRIEND_LIST_FILENAME: &'static str = "friendList.json";

fn main() {
	println!("Let's Talk!");
	
    let args = os::args();
    if args.len() < 2 {
        println!("use: client addr");
    }
    let addr = args[1].as_slice();
	
	
	// Open socket to the server.
    let mut socket = TcpStream::connect(addr, 7777).unwrap();
	
	// read client info, such as my nickname
    let result = file_io::read_friends_from_file(USER_INFO_FILENAME);
	let mut stored_user_info = match result {
        Ok(x)  => x, 
        Err(e) => {
            error!("read_friends_from_file() returned Err({}). What should we do?", e);
            Vec::new()
        }
    };
    for n in range(0u, stored_user_info.len()) {
        println!("User Info contains: {}", stored_user_info.get(n).friend_nickname);
    }
    
    // send my nickname to the server
    let temp_user_info = stored_user_info.pop();
    if temp_user_info.is_some() {
        let register_msg = message::Message{messageType: message::sign_in, messageData: message::SignIn(message::SignInMessage{user_name: temp_user_info.unwrap().friend_nickname})};
        // send the message to the server
        socket.write(register_msg.convertToJSON().into_bytes().as_slice());
    }
    
	
    // read the stored friends list to know which friends to request from the server.
    let result = file_io::read_friends_from_file(FRIEND_LIST_FILENAME);
    let stored_friend_info = match result {
        Ok(x)  => x, 
        Err(e) => {
	        error!("read_friends_from_file() returned Err({}). What should we do?", e);
	        Vec::new()
        }
    };
    
    for n in range(0u, stored_friend_info.len()) {
        println!("Friend list contains: {}", stored_friend_info.get(n).friend_nickname);
    }
    // now send a request to the server to get their IP addresses.
	
}
