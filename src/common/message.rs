//#![crate_type = "lib"]
//#![crate_name = "message"]

extern crate serialize;
use serialize::json;
use serialize::Encoder;
use serialize::Encodable;

#[path = "..\\common"]
mod common {pub mod client_information;}


#[deriving(Encodable, Decodable)]
pub enum MessageType {
    signIn,		        //client logs into server, provides username
    addressRequest,     //client requests IP address of specified client
    addressResponse,    //server sends IP address of specified client 
    clientListRequest,  //client requests list of active clients
    clientListResponse, //server response to clientListRequest
    webcam,
    text,
}

#[deriving(Encodable, Decodable)]
pub struct SignInMessage
{
	pub user_name: String 
}

#[deriving(Encodable, Decodable)]
pub struct AddressRequestMessage
{
	pub user_name: String
}

#[deriving(Encodable, Decodable)]
pub struct AddressResponseMessage
{
	pub user_name: String,
	pub ip_address: String
}

#[deriving(Encodable, Decodable)]
pub struct ClientListRequestMessage;

#[deriving(Encodable)]
pub struct ClientListResponseMessage
{
	pub client_list: Vec<common::client_information::ClientInformation>
}

#[deriving(Encodable, Decodable)]
pub struct WebcamMessage
{
	pub webcam_data: String
}

#[deriving(Encodable, Decodable)]
pub struct TextMessage
{
	pub text_data: String
}

pub enum MessageData
{
	SignIn (SignInMessage),
	AddressRequest (AddressRequestMessage),
	AddressResponse (AddressResponseMessage),
	ClientListRequest (ClientListRequestMessage),
	ClientListResponse (ClientListResponseMessage),
	Webcam (WebcamMessage),
	Text (TextMessage)
}

impl<E, S: Encoder<E>> Encodable<S, E> for MessageData
{
	fn encode(&self, s: &mut S) -> Result<(), E>
	{
		match (*self)
		{
			SignIn(ref temp_sign_in) => temp_sign_in.encode(s),
			AddressRequest(ref temp_address_request) => temp_address_request.encode(s),
			AddressResponse(ref temp_address_response) => temp_address_response.encode(s),
			ClientListRequest(ref temp_clien_list_request) => temp_clien_list_request.encode(s),
			ClientListResponse(ref temp_client_list_response) => temp_client_list_response.encode(s), 
			Webcam(ref w) => w.encode(s),
			Text(ref t) => t.encode(s)
		}
	}
}
    
#[deriving(Encodable)]
pub struct Message {
    pub message_type: MessageType,
   	pub message_data: MessageData,
}

impl Message
{
	pub fn convert_to_json(&self) -> String
	{
		json::encode(self)
	}
}


//fn main()
//{
//	print!("SignIn Sample Message:\n");
//	let signInMessage = Message {messageType: signIn, messageData: SignIn(SignInMessage {user_name: "Test".to_string()})};
//	print!("{}\n\n",signInMessage.convertToJSON());
//	print!("AddressRequest Sample Message:\n");
//	let AddressRequestMessage = Message {messageType: addressRequest, messageData: AddressRequest(AddressRequestMessage {user_name: "Test".to_string()})};
//	print!("{}\n\n",AddressRequestMessage.convertToJSON());
//	print!("ClientListRequest Sample Message:\n");
//	print!("AddressResponse Sample Message:\n");
//	let AddressResponseMessage = Message {messageType: addressResponse, messageData: AddressResponse(AddressResponseMessage {user_name: "Test".to_string(), ip_address: "127.0.0.1".to_string()})};
//	print!("{}\n\n",AddressResponseMessage.convertToJSON());
//	print!("ClientListRequest Sample Message:\n");
//	let clientListRequestMessage = Message {messageType: clientListRequest, messageData: ClientListRequest(ClientListRequestMessage)};
//	print!("{}\n\n",clientListRequestMessage.convertToJSON());
//	print!("ClientListResponse Sample Message:\n");
//	let mut clientListResponseMessage = Message {messageType: clientListResponse, messageData: ClientListResponse(ClientListResponseMessage {client_list: Vec::new()})};
//	match clientListResponseMessage.messageData
//	{
//		ClientListResponse(ref mut clientListResponse) =>	{
//			clientListResponse.client_list.push(clientInformation::ClientInformation {
//				user_name: "CBSW".to_string(), 
//				ip_address: "192.168.0.1".to_string(), 
//				status: clientInformation::Online, 
//				last_logon: clientInformation::EncodableTime::zero()
//				}
//			);
//			clientListResponse.client_list.push(clientInformation::ClientInformation {
//				user_name: "TestName".to_string(), 
//				ip_address: "192.168.0.2".to_string(), 
//				status: clientInformation::Online, 
//				last_logon: clientInformation::EncodableTime::zero()
//				}
//			);
//		},
//		_	=> {}
//	} 
//	print!("{}\n\n",clientListResponseMessage.convertToJSON());
//}