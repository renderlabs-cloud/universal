use crate::{
	uri::URI,
};

use ::std::collections::HashMap;

use ::serde::*;

///
/// A message for the ***universal**://* protocol.
///
#[derive(Clone, Serialize, Deserialize)]
#[serde(bound(deserialize = "'de: 'data"))]
pub enum Message<'data> {
	/// Request the socket to open.
	/// Can only be sent by client.
	Open(URI<'data>, Meta),

	/// Send data over the socket.
	Send(&'data [u8]),

	/// Close the socket.
	Close(),
}

pub type Meta = HashMap<String, String>;
