#[macro_use] extern crate litcontainers_derive;

pub mod container;
pub mod format;
pub mod slice;
pub mod storage;
pub mod iterator;

pub use container::*;
pub use format::*;
pub use slice::*;
pub use storage::*;
pub use iterator::*;


#[cfg(test)]
mod tests {
	#[test]
	fn it_works() {
		assert_eq!(2 + 2, 4);
	}
}
