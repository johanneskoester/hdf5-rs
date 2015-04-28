#![feature(libc)]
#![feature(convert)]

extern crate libc;
extern crate libhdf5_sys;

pub use libhdf5_sys as ffi;

pub mod location;

pub mod h5f;
pub mod h5p;

pub mod h5lt;   // HDF5 Lite API

#[test]
fn it_works() {
    use h5f::File;
    File::new("test.h5");
}
