extern crate time;
extern crate serialize;
use serialize::json;
use serialize::Encoder;
use serialize::Encodable;

#[deriving(Encodable)]
enum ClientInformationStatus
{
	Online,
	Offline,
}

struct EncodableTime
{
	encodableTime: time::Tm
}

impl EncodableTime
{
	pub fn zero() -> EncodableTime
	{
		EncodableTime {
			encodableTime: time::Tm {
				tm_sec: 0,
				tm_min: 0,
				tm_hour: 0,
				tm_mday: 0,
				tm_mon: 0,
				tm_year: 0,
				tm_wday: 0,
				tm_yday: 0,
				tm_isdst: 0,
				tm_gmtoff: 0,
				tm_nsec: 0
			}
		}
	}
}

#[deriving(Encodable)]
struct ClientInformation
{
	userName: String,
	ipAddress: String,
	status: ClientInformationStatus,
	lastLogon: EncodableTime
}

impl<E, S: Encoder<E>> Encodable<S, E> for EncodableTime
{
	fn encode(&self, s: &mut S) -> Result<(), E>
	{
		s.emit_i64(self.encodableTime.to_timespec().sec)
	}
}


fn main()
{
	let clientInformation = ClientInformation {userName: "TestName".to_string(), ipAddress: "127.0.0.1".to_string(), status: Online, lastLogon: EncodableTime::zero()};
	let encoded = json::encode(&clientInformation);
	print!("Test{}", encoded);
}