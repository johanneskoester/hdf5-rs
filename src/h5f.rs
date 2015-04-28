use std::ffi::CString;
use std::io as io;
use std::path::Path;

use libc::c_uint;

use ffi::{hid_t, H5Fcreate, H5Fclose};
use location::Location;

use h5p::H5P_DEFAULT;

pub const H5F_ACC_RDONLY  : c_uint = 0x0000;
pub const H5F_ACC_RDWR    : c_uint = 0x0001;
pub const H5F_ACC_TRUNC   : c_uint = 0x0002;
pub const H5F_ACC_EXCL    : c_uint = 0x0004;
pub const H5F_ACC_DEBUG   : c_uint = 0x0008;
pub const H5F_ACC_CREAT   : c_uint = 0x0010;
pub const H5F_ACC_DEFAULT : c_uint = 0xffff;

pub struct File { handle: hid_t }

// taken from rust/src/libstd/sys/unix/fs2.rs
fn cstr(path: &Path) -> io::Result<CString> {
    path.as_os_str().to_cstring().ok_or(
        io::Error::new(io::ErrorKind::InvalidInput, "path contained a null"))
}

impl File {
    // TODO
    // 1. create hdf5 error struct
    // 2. convert this to Result<File, hdf5 error>
    // 3. perhaps make a H5Result struct
    pub fn new<P: AsRef<Path>>(path: P) -> io::Result<File> {
        let path = try!(cstr(path.as_ref()));
        let file_id = unsafe { H5Fcreate(path.as_ptr(), H5F_ACC_TRUNC, H5P_DEFAULT, H5P_DEFAULT) };
        assert!(file_id > 0);
        Ok(File { handle: file_id })
    }
}

impl Drop for File {
    fn drop(&mut self) {
        unsafe { H5Fclose(self.handle) };
    }
}

impl Location for File {
    fn loc_id(&self) -> hid_t {
        self.handle
    }
}
