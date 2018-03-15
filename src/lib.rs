extern crate failure;
extern crate avro;
extern crate serde;

#[macro_use] mod utils;

mod core;

pub use core::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
