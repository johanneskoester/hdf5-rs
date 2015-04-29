use std::ffi::CString;
use std::io as io;
use std::path::Path;

use libc::c_uint;

use ffi;
use ::{AsHandle, AsLocation};

use h5p::H5P_DEFAULT;

pub const H5F_ACC_RDONLY  : c_uint = 0x0000;
pub const H5F_ACC_RDWR    : c_uint = 0x0001;
pub const H5F_ACC_TRUNC   : c_uint = 0x0002;
pub const H5F_ACC_EXCL    : c_uint = 0x0004;
pub const H5F_ACC_DEBUG   : c_uint = 0x0008;
pub const H5F_ACC_CREAT   : c_uint = 0x0010;
pub const H5F_ACC_DEFAULT : c_uint = 0xffff;

// taken from rust/src/libstd/sys/unix/fs2.rs
fn cstr(path: &Path) -> io::Result<CString> {
    path.as_os_str().to_cstring().ok_or(
        io::Error::new(io::ErrorKind::InvalidInput, "path contained a null"))
}

/// A reference to an open HDF5 file
///
pub struct File(ffi::hid_t);

/// Options and flags which can be used to configure how a file is opened.
///
/// Inspired by http://doc.rust-lang.org/nightly/style/ownership/builders.html
/// and std::fs:OpenOptions
pub struct OpenOptions {
    write: bool,
    access_properties: ffi::hid_t,
}

impl OpenOptions {
    pub fn new() -> OpenOptions {
        OpenOptions {
            write: false,
            access_properties: H5P_DEFAULT
        }
    }

    pub fn write(&mut self, flag: bool) -> &mut OpenOptions {
        self.write = flag;
        self
    }

    pub fn open<P: AsRef<Path>>(&self, path: P) -> io::Result<File> {
        let path = try!(cstr(path.as_ref()));
        let flags = if self.write { H5F_ACC_RDWR } else { H5F_ACC_RDWR };
        let file_id = unsafe {
            ffi::H5Fopen(path.as_ptr(), flags, self.access_properties)
        };
        if file_id > 0 {
            Ok(File(file_id))
        } else {
            Err(io::Error::new(io::ErrorKind::Other, "oh no!"))
        }
    }
}

pub struct CreateOptions {
    truncate: bool,
    access_properties: ffi::hid_t,
    creation_properties: ffi::hid_t
}

impl CreateOptions {
    pub fn new() -> CreateOptions {
        CreateOptions {
            truncate: false,
            access_properties: H5P_DEFAULT,
            creation_properties: H5P_DEFAULT
        }
    }

    pub fn truncate(&mut self, flag: bool) -> &mut CreateOptions {
        self.truncate = flag;
        self
    }

    pub fn create<P: AsRef<Path>>(&self, path: P) -> io::Result<File> {
        let path = try!(cstr(path.as_ref()));
        let flags = if self.truncate { H5F_ACC_TRUNC } else { H5F_ACC_EXCL };
        let file_id = unsafe {
            ffi::H5Fcreate(path.as_ptr(), flags, self.access_properties, self.creation_properties)
        };
        if file_id > 0 {
            Ok(File(file_id))
        } else {
            Err(io::Error::new(io::ErrorKind::Other, "oh no!"))
        }
    }
}

impl Drop for File {
    fn drop(&mut self) {
        unsafe { ffi::H5Fclose(self.0) };
    }
}

impl AsHandle for File {
    fn as_handle(&self) -> &ffi::hid_t { &self.0 }
}

impl AsLocation for File {}

#[test]
fn it_creates_and_opens_a_file() {
    use std::fs;
    {
        use h5f::CreateOptions;;
        let f = CreateOptions::new().create("test.h5").unwrap();
    }
    fs::metadata("test.h5");
    {
        use h5f::OpenOptions;;
        let f = OpenOptions::new().open("test.h5").unwrap();
    }
    fs::remove_file("test.h5");
}

