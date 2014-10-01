pub enum MessageType {
    signIn,		        //client logs into server, provides username
    addressRequest ,    //client requests IP address of specified client
    addressResponse,    //server sends IP address of specified client 
    clientListRequest,  //client requests list of active clients
    clientListResponse, //server response to clientListRequest
    broadcastMessage,   //client requests message to be sent to all other clients
    privateMessage,     //client requests message to be sent to particular client
}

pub struct Message {
    String user_name,
    
}