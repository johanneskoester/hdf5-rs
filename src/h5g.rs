use std::ffi::CString;
use std::io;

use ffi;
use h5p::H5P_DEFAULT;

use std::marker::PhantomData;

// For PhantomData see
// http://doc.rust-lang.org/nightly/std/marker/struct.PhantomData.html
// and
// http://rustbyexample.com/generics/phantom.html

pub struct Group<'a>(ffi::hid_t, PhantomData<&'a ()>);

impl<'a> Group<'a> {
    pub fn from_handle(id: ffi::hid_t) -> Group<'a> {
        assert!(id > 0);
        Group(id, PhantomData)
    }
}

use ::{AsHandle, AsLocation};

impl<'a> AsHandle for Group<'a> {
    fn as_handle(&self) -> &ffi::hid_t {
        &self.0
    }
}

impl<'a> AsLocation for Group<'a> {}

impl<'a> Drop for Group<'a> {
    fn drop(&mut self) {
        unsafe { ffi::H5Gclose(self.0) };
    }
}

#[test]
fn it_creates_groups() {
    use std::fs;
    use h5f::CreateOptions;;
    let f = CreateOptions::new().create("test_group.h5").unwrap();
    let g1 = f.create_group("test");
    let g2 = g1.create_group("test");
    // this doesn't work yet
    // f.create_group("a").create_group("b");
    fs::remove_file("test_group.h5");
}


