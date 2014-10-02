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
//use file_io::FriendInfo;
//use message::Message;
//use message::{Message, SignInMessage};
pub mod file_io;
pub mod client_information;
pub mod message {use client_information;}

static USER_INFO_FILENAME: &'static str = "userInfo.json";
static FRIEND_LIST_FILENAME: &'static str = "friendList.json";

fn main() {
	println!("Let's Talk!");
	
	// read client info, such as my nickname
    let result = file_io::read_friends_from_file(USER_INFO_FILENAME);
	let stored_user_info = match result {
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
    //let test_message_type: MessageType = signIn;
    //let signInMessage = Message {messageType: signIn, messageData: SignIn(SignInMessage {user_name: "Test".to_string()})};
    //let register_msg = message::Message{messageType: message::signIn, messageData: SignIn(message::SignInMessage{user_name: stored_user_info.get(0).friend_nickname})};
    
    
	
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
