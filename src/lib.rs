#![feature(libc)]
#![feature(convert)]

extern crate libc;
extern crate libhdf5_sys;

pub use libhdf5_sys as ffi;

pub mod h5f;
pub mod h5p;
pub mod h5lt;   // HDF5 Lite API

pub trait AsHandle {
    fn as_handle(&self) -> &ffi::hid_t;
}

pub trait AsLocation : AsHandle {
    fn as_location(&self) -> &ffi::hid_t {
        self.as_handle()
    }
}

