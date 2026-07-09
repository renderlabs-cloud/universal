use crate::{
	uri::URI,
	protocol::{
		Meta,
		Message,
	},
};

use ::std::{
	io::{
		self,
		Read,
		Write,
	},
};

use ::postcard;

///
/// TODO: ...
///
// #[read_only::cast]
#[derive(Getters, Setters)]
pub struct Handle<'data> {
	pub uri: URI<'data>,
	pub status: Status,

	#[getset(get = "pub")]
	pub(self) meta: Meta,

	// #[readonly]
	#[getset(get = "pub", set = "pub")]
	pub method: Method<'data>,
	#[getset(get = "pub", set = "pub")]
	pub(self) buffer: Option<&'data mut Vec<u8>>,
}

impl<'data> Handle<'data> {
	pub fn new(uri: URI<'data>, method: Method<'data>, meta: Meta) -> Self {
		return Self {
			uri: uri,
			status: Status::Inactive,

			meta: meta,

			method: method,

			buffer: None,
			// socket: None,
		};
	}
}

///
/// The status of a handle.
///
#[derive(Clone, Default)]
pub enum Status {
	/// The handle is inactive.
	#[default]
	Inactive,
	/// The handle is still connecting.
	Connecting,
	/// The handle is ready to be used.
	Open(),
	/// The handle is closed.
	Closed,
	/// There was an error.
	Error(String),
}

///
/// TODO: ...
///
#[allow(clippy::complexity)]
pub enum Method<'a> {
	Read(Option<&'a mut dyn Reader<'a>>),
	Write(Option<&'a mut dyn Writer<'a>>),
	Socket(Option<&'a mut dyn Socket<'a>>),
}

pub trait Reader<'data>: Read {
	fn read_message(&'data mut self) -> io::Result<Message<'data>> {
		let buffer = self.get_buffer();
		buffer.clear();
		buffer.resize(4096, 0);
		let size = self.read(&mut buffer[..])?;
		buffer.truncate(size);

		return postcard::from_bytes::<Message<'data>>(buffer).map_err(io::Error::other);
	}
	fn read_all_messages(&mut self) -> io::Result<Vec<Message<'data>>> {
		todo!();
	}

	fn get_buffer(&mut self) -> &'data mut Vec<u8>;
}

pub trait Writer<'data>: Write {
	fn write_message(&mut self, message: Message<'data>) -> io::Result<()> {
		let bytes = postcard::to_allocvec(&message).map_err(io::Error::other)?;

		self.write_all(&bytes)?;
		self.flush()?;

		return Ok(());
	}
	fn write_all_messages(&mut self, messages: Vec<Message<'data>>) -> io::Result<()> {
		for message in messages {
			self.write_message(message)?;
		}

		return Ok(());
	}
}

pub trait Socket<'data>: Reader<'data> + Writer<'data> {
	// fn get_write_seen(&self) -> Seen;
	// fn get_read_seen(&self) -> Seen;
}

// pub type Reader = (Box<dyn Read>, ());
// pub type Writer = ((), Box<dyn Write>);

// pub type Socket = (Box<dyn Read>, Box<dyn Write>);

pub type Seen = u32;
