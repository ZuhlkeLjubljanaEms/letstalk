/** ************************************************************************************************
 * \file      letstalk.rs
 * \ingroup   Let's_Talk!
 * \brief     
 * \details   
 * \author    Jeremy Hannon <hje@zuhlke.com>
 * \date      29th September 2014
 **************************************************************************************************/


extern crate serialize;
use fileIO::FriendInfo;
mod fileIO;

static FRIEND_LIST_FILENAME: &'static str = "friendList.json";


fn main() {
	println!("Hi, Rusty, let's talk!");
	

    let mut stored_friend_info: Vec<FriendInfo> = fileIO::read_friends_from_file(FRIEND_LIST_FILENAME);
	
}
