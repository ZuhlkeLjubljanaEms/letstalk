//#![crate_type = "lib"]
//#![crate_name = "message"]

extern crate serialize;
use serialize::json;
use serialize::Encoder;
use serialize::Encodable;
use serialize::Decoder;
use serialize::Decodable;

#[path = "../common"]
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

#[deriving(Encodable, Decodable)]
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

#[deriving(Encodable, Decodable)]
pub enum Message
{
	SignIn (SignInMessage),
	AddressRequest (AddressRequestMessage),
	AddressResponse (AddressResponseMessage),
	ClientListRequest (ClientListRequestMessage),
	ClientListResponse (ClientListResponseMessage),
	Webcam (WebcamMessage),
	Text (TextMessage)
}

//impl<E, S: Encoder<E>> Encodable<S, E> for MessageData
//{
//	fn encode(&self, s: &mut S) -> Result<(), E>
//	{
//		match (*self)
//		{
//			SignIn(ref sign_in) => sign_in.encode(s),
//			AddressRequest(ref address_request) => address_request.encode(s),
//			AddressResponse(ref address_response) => address_response.encode(s),
//			ClientListRequest(ref client_list_request) => client_list_request.encode(s),
//			ClientListResponse(ref client_list_response) => client_list_response.encode(s), 
//			Webcam(ref w) => w.encode(s),
//			Text(ref t) => t.encode(s)
//		}
//	}
//}
    
//#[deriving(Encodable, Decodable)]
//pub struct Message {
//    pub messageType: MessageType,
//  	pub messageData: MessageData,
//}

impl Message
{
	pub fn convert_to_json(&self) -> String
	{
		json::encode(self)
	}
}

//impl<E, D: Decoder<E>> Decodable<D, E> for Message
//{
//    fn decode(d: &mut D) -> Result<Message, E>
//    {
//    	let mut message = Message {messageType: signIn, messageData: SignIn(SignInMessage {user_name: "Test".to_string()})};
//    	let strings = ["signIn", "addressRequest" , "addressResponse", "clientListRequest", "clientListResponse", "webcam", "text"];
//    	let message_type = d.read_enum_variant(strings,
//    	|a,intValue| -> Result<MessageType,E> {match (intValue) {
//  			0 =>  Ok(signIn),
//  			_ =>  Ok(addressRequest)
//  		}});
//  		   
//  		match message_type
//    	{
//    		Ok(signIn) => Decodable::decode(d),
//			Ok(addressRequest) => message.messageData.AddressRequest.decode(d),
//			Ok(addressResponse) => message.messageData.AddressResponse.decode(d),
//			Ok(clientListRequest) => message.messageData.ClientListRequest.decode(d),
//			Ok(clientListResponse) => message.messageDataClientListResponse.decode(d),
//			Ok(webcam) => message.messageData.Webcam.decode(d),
//			Ok(text) => message.messageData.Text.decode(d),
//    	}
//    	
//    	message
//    }
//}


//fn main()
//{
//	print!("SignIn Sample Message:\n");
//	let signInMessage = Message {messageType: signIn, messageData: SignIn(SignInMessage {user_name: "Test".to_string()})};
//	print!("{}\n\n",signInMessage.convert_to_json());
//	print!("AddressRequest Sample Message:\n");
//	let AddressRequestMessage = Message {messageType: addressRequest, messageData: AddressRequest(AddressRequestMessage {user_name: "Test".to_string()})};
//	print!("{}\n\n",AddressRequestMessage.convert_to_json());
//	print!("ClientListRequest Sample Message:\n");
//	print!("AddressResponse Sample Message:\n");
//	let AddressResponseMessage = Message {messageType: addressResponse, messageData: AddressResponse(AddressResponseMessage {user_name: "Test".to_string(), ip_address: "127.0.0.1".to_string()})};
//	print!("{}\n\n",AddressResponseMessage.convert_to_json());
//	print!("ClientListRequest Sample Message:\n");
//	let clientListRequestMessage = Message {messageType: clientListRequest, messageData: ClientListRequest(ClientListRequestMessage)};
//	print!("{}\n\n",clientListRequestMessage.convert_to_json());
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
//	print!("{}\n\n",clientListResponseMessage.convert_to_json());
//}