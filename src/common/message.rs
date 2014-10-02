extern crate serialize;
use serialize::json;
use serialize::Encoder;
use serialize::Encodable;

mod clientInformation;

#[deriving(Encodable)]
pub enum MessageType {
    signIn,		        //client logs into server, provides username
    addressRequest ,    //client requests IP address of specified client
    addressResponse,    //server sends IP address of specified client 
    clientListRequest,  //client requests list of active clients
    clientListResponse, //server response to clientListRequest
    broadcastMessage,   //client requests message to be sent to all other clients
    privateMessage,     //client requests message to be sent to particular client
}

#[deriving(Encodable)]
struct SignInMessage
{
	user_name:String 
}

#[deriving(Encodable)]
struct AddressRequestMessage
{
	user_name:String
}

#[deriving(Encodable)]
struct AddressResponseMessage
{
	user_name: String,
	ip_address: String
}

#[deriving(Encodable)]
struct ClientListRequestMessage;

#[deriving(Encodable)]
struct ClientListResponseMessage
{
	clientList: Vec<clientInformation::ClientInformation>
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
    messageType: MessageType,
   	messageData: MessageData,
}

impl Message
{
	fn convertToJSON(&self) -> String
	{
		json::encode(self)
	}
}

fn main()
{
	print!("SignIn Sample Message:\n");
	let signInMessage = Message {messageType: signIn, messageData: SignIn(SignInMessage {user_name: "Test".to_string()})};
	print!("{}\n\n",signInMessage.convertToJSON());
	print!("AddressRequest Sample Message:\n");
	let AddressRequestMessage = Message {messageType: addressRequest, messageData: AddressRequest(AddressRequestMessage {user_name: "Test".to_string()})};
	print!("{}\n\n",AddressRequestMessage.convertToJSON());
	print!("ClientListRequest Sample Message:\n");
	print!("AddressResponse Sample Message:\n");
	let AddressResponseMessage = Message {messageType: addressResponse, messageData: AddressResponse(AddressResponseMessage {user_name: "Test".to_string(), ip_address: "127.0.0.1".to_string()})};
	print!("{}\n\n",AddressResponseMessage.convertToJSON());
	print!("ClientListRequest Sample Message:\n");
	let clientListRequestMessage = Message {messageType: clientListRequest, messageData: ClientListRequest(ClientListRequestMessage)};
	print!("{}\n\n",clientListRequestMessage.convertToJSON());
	print!("ClientListResponse Sample Message:\n");
	let mut clientListResponseMessage = Message {messageType: clientListResponse, messageData: ClientListResponse(ClientListResponseMessage {clientList: Vec::new()})};
	match clientListResponseMessage.messageData
	{
		ClientListResponse(ref mut client_list_response) =>	{
			client_list_response.clientList.push(clientInformation::ClientInformation {
				user_name: "CBSW".to_string(), 
				ip_address: "192.168.0.1".to_string(), 
				status: clientInformation::Online, 
				last_logon: clientInformation::EncodableTime::zero()
				}
			);
			client_list_response.clientList.push(clientInformation::ClientInformation {
				user_name: "TestName".to_string(), 
				ip_address: "192.168.0.2".to_string(), 
				status: clientInformation::Online, 
				last_logon: clientInformation::EncodableTime::zero()
				}
			);
		},
		_	=> {}
	} 
	print!("{}\n\n",clientListResponseMessage.convertToJSON());
}