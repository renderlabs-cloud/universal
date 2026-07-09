//!
//! The ***universal**://* standard.
//!
//! TODO: ...
//!
#[macro_use]
use crate::tree::*;

use ::universal;


pub const STANDARD: Tree = Tree(namespace! {
	// "test" => println!("Hello!"),
	"os" => namespace! {
		"click" => function!(|uri: universal::uri::URI| -> () {
			println!("*CLICK*");
		}),
	},
});