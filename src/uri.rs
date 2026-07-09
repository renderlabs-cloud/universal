use ::prse::{
	Parse,
	ParseError,
	try_parse,
};

use ::serde::*;

use ::std::{
	collections::{
		HashMap,
	},
};

use ::serde_json;

///
/// TODO: ...
///
#[derive(Clone, Default, Serialize, Deserialize, Debug)]
#[serde(bound(deserialize = "'de: 'input"))]
pub struct URI<'input> {
	pub path: Vec<&'input str>,
	pub parameters: Parameters<'input>,
}

// TODO: Report the issue to the maintainer of `prse`, since this is probably a bug!
impl<'input> Parse<'input> for URI<'input> {
	fn from_str(string: &'input str) -> Result<Self, ParseError> {
		let mut uri: Self = Self::default();
		// Parse everything after "universal://".
		let mut rest = string
			.strip_prefix("universal://")
			.ok_or_else(|| ParseError::new("Expected 'universal://' prefix."))?;

		// Split the path segments by '/'.
		// TODO: Enforce allowed characters.
		let path: Vec<&'input str> = if rest.is_empty() {
			vec![]
		} else {
			let (before, after) = rest.split_once(';').unwrap_or((rest, ""));
			rest = after;
			before.split('/').collect()
		};

		uri.path = path;

		if rest.is_empty() {
			return Ok(uri);
		};

		match rest.split_once(';') {
			Some(_) => {},
			None => {
				// Invalid scheme.
				return Err(ParseError::Other(format!("Expected ';', got {}", rest,)));
			},
		};

		let mut parameters = Parameters::new();

		for pair in rest.split(';') {
			if pair.is_empty() {
				continue;
			};

			let (key, value) = pair.split_once('=').unwrap_or((pair, ""));

			parameters.insert(key, Value::from_str(value)?);
		}

		uri.parameters = parameters;

		return Ok(uri);
	}
}

impl<'uri> TryFrom<&'uri str> for URI<'uri> {
	type Error = ParseError;

	fn try_from(value: &'uri str) -> Result<Self, Self::Error> {
		return try_parse!(value, "{}");
	}
}

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub enum Value {
	Integer(i128),
	Float(f64), // TODO: Update to `f128` when supported.
	String(String),
	Boolean(bool),
}

impl Default for Value {
	fn default() -> Self {
		// When using the short-hand assignment.
		return Self::Boolean(true);
	}
}

impl Parse<'_> for Value {
	fn from_str(string: &'_ str) -> Result<Self, ParseError> {
		if string.is_empty() {
			return Ok(Self::default());
		};

		// We must check integers before floats because x is a valid float.
		return if let Ok(integer) = string.parse::<i128>() {
			Ok(Self::Integer(integer))
		} else if let Ok(float) = string.parse::<f64>() {
			Ok(Self::Float(float))
		} else if let Ok(string) = serde_json::from_str(string) {
			Ok(Self::String(string))
		} else if let Ok(boolean) = string.parse::<bool>() {
			Ok(Self::Boolean(boolean))
		} else {
			Err(ParseError::Other(String::from(
				"URI parameter value didn't match any pattern.",
			)))
		};
	}
}

pub type Parameters<'uri> = HashMap<&'uri str, Value>;
