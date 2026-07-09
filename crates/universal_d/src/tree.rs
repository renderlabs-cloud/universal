use ::universal;

use ::either::Either;


pub struct Leaf (
	pub fn (Vec<String>) -> Either<Tree, Self>,
);

pub struct Tree(
	pub fn(uri: universal::uri::URI, depth: usize) -> (),
);

impl Tree {
	///
	/// TODO: ...
	///
	pub fn execute(&self, uri: universal::uri::URI, depth: usize) -> () {
		(self.0)(uri, depth);
	}
}

pub use crate::namespace;
#[macro_export]
macro_rules! namespace {
	($($arm_pattern:pat $(if $guard:expr)? => $arm_expr:expr),* $(,)?) => {
		|uri: $crate::__universal::uri::URI, depth: usize| -> () {
			match *uri.path.get(depth).expect("TODO") {
				$(
					$arm_pattern $(if $guard)? => ($arm_expr)(uri, depth + 1),
				)*
				_ => todo!(),
			};
		}
	};
}

pub use crate::function;
#[macro_export]
macro_rules! function {
	($function:expr) => {
		|uri: $crate::__universal::uri::URI, _depth: usize| -> () {
			($function)(uri);
		}
	};
}