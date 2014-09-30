/** ************************************************************************************************
 * \file      fileIO.rs
 * \ingroup   Let's_Talk!
 * \brief     
 * \details   
 * \author    Jeremy Hannon <hje@zuhlke.com>
 * \date      30th September 2014
 **************************************************************************************************/

extern crate serialize;
use serialize::json;
use std::io::{File, Open, Write, BufferedReader};



// Automatically generate `Decodable` and `Encodable` trait implementations
#[deriving(Decodable, Encodable)]
pub struct FriendInfo  {
    friend_nickname: String,
    friend_last_ip_address: String,
}

static FRIEND_LIST_FILENAME: &'static str = "friendList.json";

fn main() {
    write_json_to_file();
    read_friends_from_file(FRIEND_LIST_FILENAME);
}

fn write_json_to_file() {
    let will_friend_info = FriendInfo {
        friend_nickname: "Will43".to_string(),
        friend_last_ip_address: "192.168.20.151".to_string(),
    };

    // Serialize using `json::encode`
    let encoded_friend_info = json::encode(&will_friend_info);
    
    let path_to_friend_list_file = Path::new(FRIEND_LIST_FILENAME);

    let mut friend_list_file = match File::open_mode(&path_to_friend_list_file, Open, Write) {
            Ok(friend_list_file) => friend_list_file,
            Err(e) => fail!("file error: {}", e),
        };
    
    //println!("data: {}", encoded_friend_info);
    let _ = friend_list_file.write_line(encoded_friend_info.as_slice());
}

fn read_friends_from_file(filename: &str) -> () {

    let path = Path::new(filename);
    let mut file = BufferedReader::new(File::open(&path));
    
    let mut friend_info_collection: Vec<FriendInfo> = Vec::new();
    for line_iter in file.lines() {
        let line : String = match line_iter { Ok(x) => x, Err(e) => fail!(e) };
        // Deserialize using `json::decode`
        let decoded: FriendInfo = json::decode(line.as_slice()).unwrap();
        //println!("read data: {} decoded from {}", decoded.friend_nickname, line);
        friend_info_collection.push(decoded);
    }
    //for n in range(0u, friend_info_collection.len()) {
    //    println!("Vector contains: {}", friend_info_collection.get(n).friend_nickname);
    //}
}


