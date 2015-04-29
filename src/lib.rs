#![feature(libc)]
#![feature(convert)]

extern crate libc;
extern crate libhdf5_sys;

pub use libhdf5_sys as ffi;

pub mod h5f;
pub mod h5g;
pub mod h5p;
pub mod h5lt;   // HDF5 Lite API

pub trait AsHandle {
    fn as_handle(&self) -> &ffi::hid_t;
}

use std::io;
use std::ffi::CString;

use h5p::H5P_DEFAULT;
use h5g::Group;

pub trait AsLocation : AsHandle {
    fn as_location(&self) -> &ffi::hid_t {
        self.as_handle()
    }

    fn create_group<'a, T: AsRef<str>>(&'a self, name: T) -> Group<'a> {
        let name = CString::new(name.as_ref()).unwrap();
        let id = unsafe {
            ffi::H5Gcreate2(*self.as_location(), name.as_ptr(), H5P_DEFAULT, H5P_DEFAULT, H5P_DEFAULT)
        };
        Group::from_handle(id)
    }
}

