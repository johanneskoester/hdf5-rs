/* automatically generated by rust-bindgen */

pub type H5DS_iterate_t =
    ::std::option::Option<extern "C" fn(dset: hid_t, dim: ::libc::c_uint,
                                        scale: hid_t,
                                        visitor_data: *mut ::libc::c_void)
                              -> herr_t>;

extern "C" {
    pub fn H5DSattach_scale(did: hid_t, dsid: hid_t, idx: ::libc::c_uint)
     -> herr_t;
    pub fn H5DSdetach_scale(did: hid_t, dsid: hid_t, idx: ::libc::c_uint)
     -> herr_t;
    pub fn H5DSset_scale(dsid: hid_t, dimname: *const ::libc::c_char)
     -> herr_t;
    pub fn H5DSget_num_scales(did: hid_t, dim: ::libc::c_uint)
     -> ::libc::c_int;
    pub fn H5DSset_label(did: hid_t, idx: ::libc::c_uint,
                         label: *const ::libc::c_char) -> herr_t;
    pub fn H5DSget_label(did: hid_t, idx: ::libc::c_uint,
                         label: *mut ::libc::c_char, size: size_t) -> ssize_t;
    pub fn H5DSget_scale_name(did: hid_t, name: *mut ::libc::c_char,
                              size: size_t) -> ssize_t;
    pub fn H5DSis_scale(did: hid_t) -> htri_t;
    pub fn H5DSiterate_scales(did: hid_t, dim: ::libc::c_uint,
                              idx: *mut ::libc::c_int,
                              visitor: H5DS_iterate_t,
                              visitor_data: *mut ::libc::c_void) -> herr_t;
    pub fn H5DSis_attached(did: hid_t, dsid: hid_t, idx: ::libc::c_uint)
     -> htri_t;
}
