//#![crate_type = "lib"]
//#![crate_name = "message"]

extern crate serialize;
use serialize::json;
use serialize::Encoder;
use serialize::Encodable;

#[path = "..\\common"]
mod common {pub mod client_information;}


#[deriving(Encodable)]
pub enum MessageType {
    sign_in,		        //client logs into server, provides username
    address_request ,    //client requests IP address of specified client
    address_response,    //server sends IP address of specified client 
    client_list_request,  //client requests list of active clients
    client_list_response, //server response to client_list_request
    webcam,
    text,
}

#[deriving(Encodable)]
pub struct SignInMessage
{
	pub user_name: String 
}

#[deriving(Encodable)]
pub struct AddressRequestMessage
{
	pub user_name:String
}

#[deriving(Encodable)]
pub struct AddressResponseMessage
{
	pub user_name: String,
	pub ip_address: String
}

#[deriving(Encodable)]
pub struct ClientListRequestMessage;

#[deriving(Encodable)]
pub struct ClientListResponseMessage
{
	pub client_list: Vec<common::client_information::ClientInformation>
}

#[deriving(Encodable)]
pub struct WebcamMessage
{
	pub webcam_data: String
}

#[deriving(Encodable)]
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
			SignIn(ref temp_sign_in) => sign_in.encode(s),
			AddressRequest(ref temp_address_request) => address_request.encode(s),
			AddressResponse(ref temp_address_response) => address_response.encode(s),
			ClientListRequest(ref temp_client_list_request) => client_list_request.encode(s),
			ClientListResponse(ref temp_client_list_response) => client_list_response.encode(s), 
			Webcam(ref w) => w.encode(s),
			Text(ref t) => t.encode(s)
		}
	}
}
    
#[deriving(Encodable)]
pub struct Message {
    pub messageType: MessageType,
   	pub messageData: MessageData,
}

impl Message
{
	pub fn convertToJSON(&self) -> String
	{
		json::encode(self)
	}
}


//fn main()
//{
//	print!("SignIn Sample Message:\n");
//	let sign_inMessage = Message {messageType: sign_in, messageData: SignIn(SignInMessage {user_name: "Test".to_string()})};
//	print!("{}\n\n",sign_inMessage.convertToJSON());
//	print!("AddressRequest Sample Message:\n");
//	let AddressRequestMessage = Message {messageType: address_request, messageData: AddressRequest(AddressRequestMessage {user_name: "Test".to_string()})};
//	print!("{}\n\n",AddressRequestMessage.convertToJSON());
//	print!("ClientListRequest Sample Message:\n");
//	print!("AddressResponse Sample Message:\n");
//	let AddressResponseMessage = Message {messageType: address_response, messageData: AddressResponse(AddressResponseMessage {user_name: "Test".to_string(), ip_address: "127.0.0.1".to_string()})};
//	print!("{}\n\n",AddressResponseMessage.convertToJSON());
//	print!("ClientListRequest Sample Message:\n");
//	let client_list_requestMessage = Message {messageType: client_list_request, messageData: ClientListRequest(ClientListRequestMessage)};
//	print!("{}\n\n",client_list_requestMessage.convertToJSON());
//	print!("ClientListResponse Sample Message:\n");
//	let mut client_list_responseMessage = Message {messageType: client_list_response, messageData: ClientListResponse(ClientListResponseMessage {client_list: Vec::new()})};
//	match client_list_responseMessage.messageData
//	{
//		ClientListResponse(ref mut client_list_response) =>	{
//			client_list_response.client_list.push(clientInformation::ClientInformation {
//				user_name: "CBSW".to_string(), 
//				ip_address: "192.168.0.1".to_string(), 
//				status: clientInformation::Online, 
//				last_logon: clientInformation::EncodableTime::zero()
//				}
//			);
//			client_list_response.client_list.push(clientInformation::ClientInformation {
//				user_name: "TestName".to_string(), 
//				ip_address: "192.168.0.2".to_string(), 
//				status: clientInformation::Online, 
//				last_logon: clientInformation::EncodableTime::zero()
//				}
//			);
//		},
//		_	=> {}
//	} 
//	print!("{}\n\n",client_list_responseMessage.convertToJSON());
//}