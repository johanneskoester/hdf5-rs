#![feature(libc)]
#![feature(convert)]

extern crate libc;
extern crate libhdf5_sys;

use libhdf5_sys::*;
use libc::c_uint;

pub const H5P_DEFAULT     : hid_t = 0;

// H5F

use std::path::Path;

pub const H5F_ACC_RDONLY  : c_uint = 0x0000;
pub const H5F_ACC_RDWR    : c_uint = 0x0001;
pub const H5F_ACC_TRUNC   : c_uint = 0x0002;
pub const H5F_ACC_EXCL    : c_uint = 0x0004;
pub const H5F_ACC_DEBUG   : c_uint = 0x0008;
pub const H5F_ACC_CREAT   : c_uint = 0x0010;
pub const H5F_ACC_DEFAULT : c_uint = 0xffff;

pub struct File { handle: hid_t }

impl File {
    // TODO
    // 1. create hdf5 error struct
    // 2. convert this to Result<File, hdf5 error>
    // 3. perhaps make a H5Result struct
    pub fn new<P: AsRef<Path>>(path: P) -> Option<File> {
        // this looks complicated TODO
        let f = path.as_ref().as_os_str().to_cstring().unwrap();
        unsafe {
            let file_id = H5Fcreate(f.as_ptr(), H5F_ACC_TRUNC, H5P_DEFAULT, H5P_DEFAULT);
            if file_id <= 0 {
                Some(File { handle: file_id })
            } else {
                None
            }
        }
    }
}

impl Drop for File {
    fn drop(&mut self) {
        unsafe {
            H5Fclose(self.handle);
        }
    }
}

#[test]
fn it_works() {
    File::new("test.h5");
}
