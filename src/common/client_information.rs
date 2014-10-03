extern crate time;
extern crate serialize;
#[test] use serialize::json;
use serialize::Encoder;
use serialize::Encodable;
use serialize::Decoder;
use serialize::Decodable;

#[deriving(Encodable, Decodable)]
pub enum ClientInformationStatus
{
	Online,
	Offline,
}

pub struct EncodableTime
{
	encodable_time: time::Tm
}

impl EncodableTime
{
	pub fn zero() -> EncodableTime
	{
		EncodableTime {
			encodable_time: time::Tm {
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

#[deriving(Encodable,Decodable)]
pub struct ClientInformation
{
	pub user_name: String,
	pub ip_address: String,
	pub status: ClientInformationStatus,
	pub last_logon: EncodableTime
}

impl<E, S: Encoder<E>> Encodable<S, E> for EncodableTime
{
	fn encode(&self, s: &mut S) -> Result<(), E>
	{
		s.emit_i64(self.encodable_time.to_timespec().sec)
	}
}


impl<E, D: Decoder<E>> Decodable<D, E> for EncodableTime
{
    fn decode(d: &mut D) -> Result<EncodableTime, E>
    {
    	Ok(EncodableTime::zero())
    }
}


#[test]
fn create_basic_client_information_message()
{
	let client_information = ClientInformation {user_name: "TestName".to_string(), ip_address: "127.0.0.1".to_string(), status: Online, last_logon: EncodableTime::zero()};
	let encoded = json::encode(&client_information);
	assert_eq!(encoded.as_slice(), "{\"user_name\":\"TestName\",\"ip_address\":\"127.0.0.1\",\"status\":\"Online\",\"last_logon\":-28801}")
}