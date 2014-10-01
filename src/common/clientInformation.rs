extern crate time;

enum ClientInformationStatus
{
	Online,
	Offline,
}

struct ClientInformation
{
	userName: String,
	ipAddress: String,
	status: ClientInformationStatus,
	lastLogon:time::Tm
}