/* automatically generated by rust-bindgen */


extern "C" {
    pub fn H5FD_core_init() -> hid_t;
    pub fn H5FD_core_term() -> ();
    pub fn H5Pset_fapl_core(fapl_id: hid_t, increment: size_t,
                            backing_store: hbool_t) -> herr_t;
    pub fn H5Pget_fapl_core(fapl_id: hid_t, increment: *mut size_t,
                            backing_store: *mut hbool_t) -> herr_t;
}
