#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![feature(libc)]

extern crate libc;

pub type int64_t = i64;
pub type uint64_t = u64;

pub type int32_t = i32;
pub type uint32_t = u32;

use libc::{size_t, ssize_t, time_t, off_t, FILE};

// Main HDF5 Library, or Low-level APIs

include!("H5public.rs");

include!("H5Apublic.rs");
include!("H5Cpublic.rs");
include!("H5Dpublic.rs");
include!("H5Epublic.rs");
include!("H5Epubgen.rs");
include!("H5Fpublic.rs");
include!("H5Gpublic.rs");
include!("H5Ipublic.rs");
include!("H5Opublic.rs");
include!("H5Lpublic.rs");
include!("H5Ppublic.rs");
include!("H5Rpublic.rs");
include!("H5Spublic.rs");
include!("H5Tpublic.rs");
include!("H5Zpublic.rs");

include!("H5ACpublic.rs");
include!("H5FDcore.rs");
include!("H5FDfamily.rs");
include!("H5FDlog.rs");
include!("H5FDmpi.rs");
include!("H5FDmulti.rs");
include!("H5FDpublic.rs");
include!("H5FDsec2.rs");
include!("H5FDstdio.rs");
include!("H5MMpublic.rs");

// High-level HDF5 APIs

include!("H5DOpublic.rs");
include!("H5DSpublic.rs");
include!("H5IMpublic.rs");
include!("H5LTpublic.rs");
include!("H5PTpublic.rs");
include!("H5TBpublic.rs");

#[test]
fn it_works() {
    use libc::c_uint;

    let mut maj : c_uint = 0;
    let mut min : c_uint = 0;
    let mut rel : c_uint = 0;

    unsafe {
        let err = H5get_libversion(&mut maj, &mut min, &mut rel);
        assert!(err >= 0);
    }
}
