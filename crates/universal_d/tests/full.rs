#[cfg(test)]
mod tests {
	use ::universal::{
		self,
		handle::{
			Handle,
			Method,
		},
		protocol::Message,
	};

	use ::std::{
		thread,
		time::Duration,
	};

	use ::map_macro::*;

	///
	/// A full test of ***universal**://*.
	///
	#[test]
	pub fn full() -> () {
		thread::spawn(|| universal_d::daemon::main().expect("Server daemon failed!"));

		thread::sleep(Duration::from_millis(1000)); // Wait for daemon. 1s should always be enough.

		let uri = universal::uri::URI::try_from("universal://os/click")
			.expect("URI parsing failed!");

		let mut handle = Handle::new(uri, Method::Socket(None), hash_map! {
			
		});

		let handle_ref = universal_d::daemon::connect(&mut handle).expect("Could not take handle!");

		let socket = match handle_ref.method {
			Method::Socket(Some(ref mut socket)) => socket,
			_ => unreachable!(),
		};

		println!("Connected!");

		socket
			.write_message(Message::Send(b"Hello, Earth!"))
			.expect("Failed to write message!");
	}
}
