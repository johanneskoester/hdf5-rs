/* automatically generated by rust-bindgen */

#[link(name = "hdf5_hl")]
extern "C" {
    pub fn H5DOwrite_chunk(dset_id: hid_t, dxpl_id: hid_t, filters: uint32_t,
                           offset: *const hsize_t, data_size: size_t,
                           buf: *const ::libc::c_void) -> herr_t;
}
