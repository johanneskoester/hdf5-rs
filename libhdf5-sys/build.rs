use std::env;

fn main() {
    let libname = env::var("HDF5_LIBNAME").unwrap_or("hdf5".to_string());
    let hl_libname = env::var("HDF5_HL_LIBNAME").unwrap_or("hdf5_hl".to_string());
    println!("cargo:rustc-flags=-l {} -l {}", libname, hl_libname);
}
