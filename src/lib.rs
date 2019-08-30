#[macro_use] extern crate litcontainers_derive;
#[macro_use] extern crate derive_new;

pub mod container;
pub mod format;
pub mod slice;
pub mod storage;

pub use container::*;
pub use format::*;
pub use slice::*;
pub use storage::*;


#[cfg(test)]
mod tests {
	#[test]
	fn it_works() {
		assert_eq!(2 + 2, 4);
	}
}
