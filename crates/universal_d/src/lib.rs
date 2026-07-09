#![feature(never_type)]

pub mod standard;
pub mod tree;

#[macro_use]
extern crate map_macro;

#[cfg(feature = "daemon")]
#[doc(hidden)]
pub use ::interprocess::local_socket;
#[doc(hidden)]
pub use ::universal as __universal;

pub const SERVICE_NAME: &str = "universal_d.sock";
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg(feature = "daemon")]
pub mod daemon {
	use crate::{
		SERVICE_NAME,
		local_socket::{
			*,
			Stream,
			traits::{
				Stream as _,
				ListenerExt as _,
			},
		},
		standard::STANDARD,
	};

	use ::interprocess::TryClone;

	use ::universal::{
		handle::{
			Handle,
			Socket,
			Writer,
			Reader,
			Status,
			Method,
		},
		protocol::{
			Message,
		},
	};

	use ::getset::*;

	use ::thiserror;

	use ::std::{
		io,
		sync::{
			Arc,
		},
	};

	///
	/// TODO: ...
	///
	#[derive(Getters, Setters)]
	pub struct Connection {
		pub(self) stream: Stream,
		pub(self) buffer: Arc<Vec<u8>>,
		#[getset(get = "pub", set = "pub")]
		pub(self) initalized: bool,
	}

	impl Connection {
		pub fn new(stream: Stream) -> Self {
			return Self {
				stream: stream,
				buffer: Arc::new(Vec::new()),
				initalized: false,
			};
		}
	}

	impl TryClone for Connection {
		fn try_clone(&self) -> io::Result<Self> {
			let stream = self.stream.try_clone()?;
			let buffer = &**self.buffer;

			return Ok(Self {
				stream: stream,
				buffer: Arc::new(Vec::from(buffer)),
				initalized: true,
			});
		}
	}

	impl io::Write for Connection {
		fn write(&mut self, buffer: &[u8]) -> io::Result<usize> {
			self.stream.write(buffer)
		}

		fn flush(&mut self) -> io::Result<()> {
			self.stream.flush()
		}
	}
	impl io::Read for Connection {
		fn read(&mut self, buffer: &mut [u8]) -> io::Result<usize> {
			self.stream.read(buffer)
		}
	}

	impl<'data> Writer<'data> for Connection {
	}

	impl<'data> Reader<'data> for Connection {
		fn get_buffer(&mut self) -> &'data mut Vec<u8> {
			return Box::leak(Box::new(self.buffer.to_vec()));
		}
	}

	impl<'data> Socket<'data> for Connection {
	}

	#[derive(Clone, Debug, thiserror::Error)]
	pub enum Error {
		#[error("The universal:// server is unreachable.")]
		Unreachable(),
	}

	pub fn connect<'data>(handle: &'data mut Handle<'data>) -> io::Result<&'data mut Handle<'data>> {
		let socket_name = if GenericNamespaced::is_supported() {
			SERVICE_NAME.to_ns_name::<GenericNamespaced>()?
		} else {
			format!("/tmp/{SERVICE_NAME}").to_fs_name::<GenericFilePath>()?
		};

		let connection: Stream = Stream::connect(socket_name)?;

		// handle.socket = Some(connection);

		let socket = Box::leak(Box::new(Connection::new(connection)));

		socket.write_message(Message::Open(handle.uri.clone(), handle.meta().clone()))?;

		handle.set_method(match handle.method() {
			Method::Read(..) => Method::Read(Some(socket)),
			Method::Write(..) => Method::Write(Some(socket)),
			Method::Socket(..) => Method::Socket(Some(socket)),
		});

		handle.status = Status::Open();

		return Ok(handle);
	}

	pub fn main() -> io::Result<!> {
		loop {
			let socket_name = if GenericNamespaced::is_supported() {
				SERVICE_NAME.to_ns_name::<GenericNamespaced>()?
			} else {
				SERVICE_NAME.to_fs_name::<GenericFilePath>()?
			};

			let listener = match ListenerOptions::new().name(socket_name).create_sync() {
				Err(error) if error.kind() == io::ErrorKind::AddrInUse => {
					todo!();
				},

				result => result?,
			};

			#[cfg(test)]
			{
				println!("[DAEMON] Listening...");
			};

			for mut connection in listener
				.incoming()
				.filter_map(|result| {
					result
						.map_err(|e| eprintln!("[DAEMON] Incoming connection failed: {e}."))
						.ok()
				})
				.map(Connection::new)
			{
				#[cfg(test)]
				{
					println!("[DAEMON] Connected!");
				};
	
				let (uri, meta) = match connection.read_message()? {
					Message::Open(uri, meta) => (uri, meta),

					_ => {
						todo!();
					},
				};

				{
					println!("[DAEMON] Got URI: {uri:?}.");
					println!("[DAEMON] Got meta: {meta:?}.");
				};

				STANDARD.execute(uri, 0);
			}
		}
	}
}

/*
///
/// TODO: ...
///
pub struct Handshake {
	pub(self) client_tx: Vec<u64>,
}
*/
