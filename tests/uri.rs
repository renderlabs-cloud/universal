/*#[cfg(test)]
mod tests {
	use ::universal::uri;

	use ::map_macro::*;

	#[test]
	pub fn parse() -> () {
		let uri = uri::URI::try_from("universal://dev/null;read;write=false;").expect("");

		assert!(uri.path == vec!["dev", "null"]);
		assert!(
			uri.parameters
				== hash_map! {
					"read" => uri::Value::Boolean(true),
					"write" => uri::Value::Boolean(false,)
				}
		);

		println!("{uri:?}");

		uri::URI::try_from("universal://dev/null/").expect("");
	}
}
*/
