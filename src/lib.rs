//! # virtio-net_rs
//!
//! A ixy-style virtio-network driver
//! Heavily influenced by github.com/ixy-languages/ixy.rs


extern crate libc;
extern crate byteorder;
#[macro_use]
extern crate log;

mod vitnet;
pub mod memory;
mod pci;
mod constants;



#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
