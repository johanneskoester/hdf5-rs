use std::ffi::CString;
use libc::{c_int, c_char};

use location::Location;
use h5f::File;
use ffi::{hsize_t, H5LTmake_dataset_char, H5LTread_dataset_char};

fn make_dataset_char<L: Location>(loc: L, name: &str, dims: &[usize], data: &[u8]) -> () {
    // necessary? I need to copy dims into an array of hsize_t
    let dims : Vec<_> = dims.iter().map(|&d| d as hsize_t).collect();
    let name = CString::new(name).unwrap();
    unsafe {
        H5LTmake_dataset_char(loc.loc_id(), name.as_ptr(), dims.len() as c_int, dims.as_ptr(), data.as_ptr() as *const c_char);
    }
    // TODO error checking
}

fn read_dataset_char<L: Location>(loc: L, name: &str, data: &mut [u8]) -> () {
    // necessary? I need to copy dims into an array of hsize_t
    let name = CString::new(name).unwrap();
    unsafe {
        H5LTread_dataset_char(loc.loc_id(), name.as_ptr(), data.as_ptr() as *mut c_char);
    }
    // TODO error checking
}

#[test]
fn it_works() {
    use h5f::File;
    let f = File::new("test.h5").unwrap();
    make_dataset_char(f, "test", &[3], &[1, 2, 3, 4]);
}
