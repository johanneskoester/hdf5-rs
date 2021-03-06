/* automatically generated by rust-bindgen */

pub type Enum_H5F_scope_t = ::libc::c_uint;
pub const H5F_SCOPE_LOCAL: ::libc::c_uint = 0;
pub const H5F_SCOPE_GLOBAL: ::libc::c_uint = 1;
pub type H5F_scope_t = Enum_H5F_scope_t;
pub type Enum_H5F_close_degree_t = ::libc::c_uint;
pub const H5F_CLOSE_DEFAULT: ::libc::c_uint = 0;
pub const H5F_CLOSE_WEAK: ::libc::c_uint = 1;
pub const H5F_CLOSE_SEMI: ::libc::c_uint = 2;
pub const H5F_CLOSE_STRONG: ::libc::c_uint = 3;
pub type H5F_close_degree_t = Enum_H5F_close_degree_t;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_H5F_info_t {
    pub super_ext_size: hsize_t,
    pub sohm: Struct_H5Fpublic_Unnamed1,
}
impl ::std::clone::Clone for Struct_H5F_info_t {
    fn clone(&self) -> Struct_H5F_info_t { *self }
}
impl ::std::default::Default for Struct_H5F_info_t {
    fn default() -> Struct_H5F_info_t { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy)]
pub struct Struct_H5Fpublic_Unnamed1 {
    pub hdr_size: hsize_t,
    pub msgs_info: H5_ih_info_t,
}
impl ::std::clone::Clone for Struct_H5Fpublic_Unnamed1 {
    fn clone(&self) -> Struct_H5Fpublic_Unnamed1 { *self }
}
impl ::std::default::Default for Struct_H5Fpublic_Unnamed1 {
    fn default() -> Struct_H5Fpublic_Unnamed1 { unsafe { ::std::mem::zeroed() } }
}
pub type H5F_info_t = Struct_H5F_info_t;
pub type Enum_H5F_mem_t = ::libc::c_int;
pub const H5FD_MEM_NOLIST: ::libc::c_int = -1;
pub const H5FD_MEM_DEFAULT: ::libc::c_int = 0;
pub const H5FD_MEM_SUPER: ::libc::c_int = 1;
pub const H5FD_MEM_BTREE: ::libc::c_int = 2;
pub const H5FD_MEM_DRAW: ::libc::c_int = 3;
pub const H5FD_MEM_GHEAP: ::libc::c_int = 4;
pub const H5FD_MEM_LHEAP: ::libc::c_int = 5;
pub const H5FD_MEM_OHDR: ::libc::c_int = 6;
pub const H5FD_MEM_NTYPES: ::libc::c_int = 7;
pub type H5F_mem_t = Enum_H5F_mem_t;
pub type Enum_H5F_libver_t = ::libc::c_uint;
pub const H5F_LIBVER_EARLIEST: ::libc::c_uint = 0;
pub const H5F_LIBVER_LATEST: ::libc::c_uint = 1;
pub type H5F_libver_t = Enum_H5F_libver_t;

extern "C" {
    pub fn H5Fis_hdf5(filename: *const ::libc::c_char) -> htri_t;
    pub fn H5Fcreate(filename: *const ::libc::c_char, flags: ::libc::c_uint,
                     create_plist: hid_t, access_plist: hid_t) -> hid_t;
    pub fn H5Fopen(filename: *const ::libc::c_char, flags: ::libc::c_uint,
                   access_plist: hid_t) -> hid_t;
    pub fn H5Freopen(file_id: hid_t) -> hid_t;
    pub fn H5Fflush(object_id: hid_t, scope: H5F_scope_t) -> herr_t;
    pub fn H5Fclose(file_id: hid_t) -> herr_t;
    pub fn H5Fget_create_plist(file_id: hid_t) -> hid_t;
    pub fn H5Fget_access_plist(file_id: hid_t) -> hid_t;
    pub fn H5Fget_intent(file_id: hid_t, intent: *mut ::libc::c_uint)
     -> herr_t;
    pub fn H5Fget_obj_count(file_id: hid_t, types: ::libc::c_uint) -> ssize_t;
    pub fn H5Fget_obj_ids(file_id: hid_t, types: ::libc::c_uint,
                          max_objs: size_t, obj_id_list: *mut hid_t)
     -> ssize_t;
    pub fn H5Fget_vfd_handle(file_id: hid_t, fapl: hid_t,
                             file_handle: *mut *mut ::libc::c_void) -> herr_t;
    pub fn H5Fmount(loc: hid_t, name: *const ::libc::c_char, child: hid_t,
                    plist: hid_t) -> herr_t;
    pub fn H5Funmount(loc: hid_t, name: *const ::libc::c_char) -> herr_t;
    pub fn H5Fget_freespace(file_id: hid_t) -> hssize_t;
    pub fn H5Fget_filesize(file_id: hid_t, size: *mut hsize_t) -> herr_t;
    pub fn H5Fget_file_image(file_id: hid_t, buf_ptr: *mut ::libc::c_void,
                             buf_len: size_t) -> ssize_t;
    pub fn H5Fget_mdc_config(file_id: hid_t,
                             config_ptr: *mut H5AC_cache_config_t) -> herr_t;
    pub fn H5Fset_mdc_config(file_id: hid_t,
                             config_ptr: *mut H5AC_cache_config_t) -> herr_t;
    pub fn H5Fget_mdc_hit_rate(file_id: hid_t,
                               hit_rate_ptr: *mut ::libc::c_double) -> herr_t;
    pub fn H5Fget_mdc_size(file_id: hid_t, max_size_ptr: *mut size_t,
                           min_clean_size_ptr: *mut size_t,
                           cur_size_ptr: *mut size_t,
                           cur_num_entries_ptr: *mut ::libc::c_int) -> herr_t;
    pub fn H5Freset_mdc_hit_rate_stats(file_id: hid_t) -> herr_t;
    pub fn H5Fget_name(obj_id: hid_t, name: *mut ::libc::c_char, size: size_t)
     -> ssize_t;
    pub fn H5Fget_info(obj_id: hid_t, bh_info: *mut H5F_info_t) -> herr_t;
    pub fn H5Fclear_elink_file_cache(file_id: hid_t) -> herr_t;
}
