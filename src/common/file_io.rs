#![feature(phase)]
/** ************************************************************************************************
 * \file      file_io.rs
 * \ingroup   Let's_Talk!
 * \brief     
 * \details   
 * \author    Jeremy Hannon <hje@zuhlke.com>
 * \date      30th September 2014
 **************************************************************************************************/

#[phase(plugin, link)] extern crate log;

extern crate serialize;
use serialize::json;
use std::io::{File, IoResult, BufferedReader};

use std::fmt;
use std::fmt::Formatter;
use std::fmt::Show;
#[test] use std::io::fs::PathExtensions;

// Automatically generate `Decodable` and `Encodable` trait implementations
#[deriving(Decodable, Encodable)]
pub struct FriendInfo  {
    pub friend_nickname: String,
    pub friend_last_ip_address: String,
}

impl Show for FriendInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Name: {} Ip: {} ", self.friend_nickname, self.friend_last_ip_address)
    }
}

pub fn read_friends_from_file(filename: &str) -> IoResult<Vec<FriendInfo>> {

    let mut error_reader = BufferedReader::new(File::open(&Path::new(filename)));
    if error_reader.lines().any(|a| a.is_err()) {
    	Err(error_reader.lines().find(|a| a.is_err()).unwrap().unwrap_err())
    	
    } else {
   		let mut friend_reader = BufferedReader::new(File::open(&Path::new(filename)));
    	let friends: Vec<FriendInfo> = friend_reader.lines().
    		map(|b| b.unwrap()).
    		map(|c| json::decode(c.as_slice()).unwrap()).
    		collect();
    	Ok(friends)
    }
}

#[test]
fn read_from_good_file_is_successful() {

    // Given - verify test file doesn't exist
    let filename: &str = "friendList.json";
    let path = Path::new(filename);
    assert!(path.exists());

    // When
    let result = read_friends_from_file(filename);

    // Then
    let _ = match result {
        Err(e) => fail!("Got unexpected error {}", e),
        Ok(ref d) => assert_eq!(d.len(), 3)
    };
    println!("{}", result.unwrap());
}

#[test]
fn file_not_present_should_return_error_code() {
  
    // Given - verify test file doesn't exist
    let non_existent_filename: &str = "nonexistent.json";
    let non_existent_path = Path::new(non_existent_filename);
    assert!(!(non_existent_path.exists()));

    // When
    let result = read_friends_from_file(non_existent_filename);

    // Then
    //let expected = IoError{kind:std::io::FileNotFound, desc: "couldn't open file ...", detail: None};
    let _ = match result {
        Err(e) => assert_eq!(e.kind, std::io::FileNotFound),
        _ => fail!("Should have returned an IoError.")
    };
}

