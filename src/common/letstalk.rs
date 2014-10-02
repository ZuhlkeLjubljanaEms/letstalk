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
use file_io::FriendInfo;
mod file_io;

static FRIEND_LIST_FILENAME: &'static str = "friendList.json";


fn main() {
	println!("Hi, Rusty, let's talk!");
	
    // read the stored friends list to know which friends to request from the server.
    let result = file_io::read_friends_from_file(FRIEND_LIST_FILENAME);
    // TODO: try using 'try' instead of 'match'.
    let mut stored_friend_info = match result {
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
