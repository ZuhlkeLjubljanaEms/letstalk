//#![feature(globs)]

extern crate serialize;
use serialize::json;
use serialize::Encoder;
use serialize::Encodable;

pub mod client_information;


#[deriving(Encodable)]
pub enum MessageType {
    signIn,		        //client logs into server, provides username
    addressRequest ,    //client requests IP address of specified client
    addressResponse,    //server sends IP address of specified client 
    client_listRequest,  //client requests list of active clients
    client_listResponse, //server response to client_listRequest
    broadcastMessage,   //client requests message to be sent to all other clients
    privateMessage,     //client requests message to be sent to particular client
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
	pub client_list: Vec<client_information::ClientInformation>
}

 enum MessageData
{
	SignIn (SignInMessage),
	AddressRequest (AddressRequestMessage),
	AddressResponse (AddressResponseMessage),
	ClientListRequest (ClientListRequestMessage),
	ClientListResponse (ClientListResponseMessage)
}

impl<E, S: Encoder<E>> Encodable<S, E> for MessageData
{
	fn encode(&self, s: &mut S) -> Result<(), E>
	{
		match (*self)
		{
			SignIn(ref sign_in) => sign_in.encode(s),
			AddressRequest(ref address_request) => address_request.encode(s),
			AddressResponse(ref address_response) => address_response.encode(s),
			ClientListRequest(ref client_list_request) => client_list_request.encode(s),
			ClientListResponse(ref client_list_response) => client_list_response.encode(s), 
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
	fn convertToJSON(&self) -> String
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
//	let client_listRequestMessage = Message {messageType: client_listRequest, messageData: ClientListRequest(ClientListRequestMessage)};
//	print!("{}\n\n",client_listRequestMessage.convertToJSON());
//	print!("ClientListResponse Sample Message:\n");
//	let mut client_listResponseMessage = Message {messageType: client_listResponse, messageData: ClientListResponse(ClientListResponseMessage {client_list: Vec::new()})};
//	match client_listResponseMessage.messageData
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
//	print!("{}\n\n",client_listResponseMessage.convertToJSON());
//}