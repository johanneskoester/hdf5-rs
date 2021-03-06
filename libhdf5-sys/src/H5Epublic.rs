/* automatically generated by rust-bindgen */

pub type Enum_H5E_type_t = ::libc::c_uint;
pub const H5E_MAJOR: ::libc::c_uint = 0;
pub const H5E_MINOR: ::libc::c_uint = 1;
pub type H5E_type_t = Enum_H5E_type_t;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_H5E_error2_t {
    pub cls_id: hid_t,
    pub maj_num: hid_t,
    pub min_num: hid_t,
    pub line: ::libc::c_uint,
    pub func_name: *const ::libc::c_char,
    pub file_name: *const ::libc::c_char,
    pub desc: *const ::libc::c_char,
}
impl ::std::clone::Clone for Struct_H5E_error2_t {
    fn clone(&self) -> Struct_H5E_error2_t { *self }
}
impl ::std::default::Default for Struct_H5E_error2_t {
    fn default() -> Struct_H5E_error2_t { unsafe { ::std::mem::zeroed() } }
}
pub type H5E_error2_t = Struct_H5E_error2_t;
pub type Enum_H5E_direction_t = ::libc::c_uint;
pub const H5E_WALK_UPWARD: ::libc::c_uint = 0;
pub const H5E_WALK_DOWNWARD: ::libc::c_uint = 1;
pub type H5E_direction_t = Enum_H5E_direction_t;
pub type H5E_walk2_t =
    ::std::option::Option<extern "C" fn(n: ::libc::c_uint,
                                        err_desc: *const H5E_error2_t,
                                        client_data: *mut ::libc::c_void)
                              -> herr_t>;
pub type H5E_auto2_t =
    ::std::option::Option<extern "C" fn(estack: hid_t,
                                        client_data: *mut ::libc::c_void)
                              -> herr_t>;
pub type H5E_major_t = hid_t;
pub type H5E_minor_t = hid_t;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_H5E_error1_t {
    pub maj_num: H5E_major_t,
    pub min_num: H5E_minor_t,
    pub func_name: *const ::libc::c_char,
    pub file_name: *const ::libc::c_char,
    pub line: ::libc::c_uint,
    pub desc: *const ::libc::c_char,
}
impl ::std::clone::Clone for Struct_H5E_error1_t {
    fn clone(&self) -> Struct_H5E_error1_t { *self }
}
impl ::std::default::Default for Struct_H5E_error1_t {
    fn default() -> Struct_H5E_error1_t { unsafe { ::std::mem::zeroed() } }
}
pub type H5E_error1_t = Struct_H5E_error1_t;
pub type H5E_walk1_t =
    ::std::option::Option<extern "C" fn(n: ::libc::c_int,
                                        err_desc: *mut H5E_error1_t,
                                        client_data: *mut ::libc::c_void)
                              -> herr_t>;
pub type H5E_auto1_t =
    ::std::option::Option<extern "C" fn(client_data: *mut ::libc::c_void)
                              -> herr_t>;

extern "C" {
    pub static mut H5E_ERR_CLS_g: hid_t;
}

extern "C" {
    pub fn H5Eregister_class(cls_name: *const ::libc::c_char,
                             lib_name: *const ::libc::c_char,
                             version: *const ::libc::c_char) -> hid_t;
    pub fn H5Eunregister_class(class_id: hid_t) -> herr_t;
    pub fn H5Eclose_msg(err_id: hid_t) -> herr_t;
    pub fn H5Ecreate_msg(cls: hid_t, msg_type: H5E_type_t,
                         msg: *const ::libc::c_char) -> hid_t;
    pub fn H5Ecreate_stack() -> hid_t;
    pub fn H5Eget_current_stack() -> hid_t;
    pub fn H5Eclose_stack(stack_id: hid_t) -> herr_t;
    pub fn H5Eget_class_name(class_id: hid_t, name: *mut ::libc::c_char,
                             size: size_t) -> ssize_t;
    pub fn H5Eset_current_stack(err_stack_id: hid_t) -> herr_t;
    pub fn H5Epush2(err_stack: hid_t, file: *const ::libc::c_char,
                    func: *const ::libc::c_char, line: ::libc::c_uint,
                    cls_id: hid_t, maj_id: hid_t, min_id: hid_t,
                    msg: *const ::libc::c_char, ...) -> herr_t;
    pub fn H5Epop(err_stack: hid_t, count: size_t) -> herr_t;
    pub fn H5Eprint2(err_stack: hid_t, stream: *mut FILE) -> herr_t;
    pub fn H5Ewalk2(err_stack: hid_t, direction: H5E_direction_t,
                    func: H5E_walk2_t, client_data: *mut ::libc::c_void)
     -> herr_t;
    pub fn H5Eget_auto2(estack_id: hid_t, func: *mut H5E_auto2_t,
                        client_data: *mut *mut ::libc::c_void) -> herr_t;
    pub fn H5Eset_auto2(estack_id: hid_t, func: H5E_auto2_t,
                        client_data: *mut ::libc::c_void) -> herr_t;
    pub fn H5Eclear2(err_stack: hid_t) -> herr_t;
    pub fn H5Eauto_is_v2(err_stack: hid_t, is_stack: *mut ::libc::c_uint)
     -> herr_t;
    pub fn H5Eget_msg(msg_id: hid_t, _type: *mut H5E_type_t,
                      msg: *mut ::libc::c_char, size: size_t) -> ssize_t;
    pub fn H5Eget_num(error_stack_id: hid_t) -> ssize_t;
    pub fn H5Eclear1() -> herr_t;
    pub fn H5Eget_auto1(func: *mut H5E_auto1_t,
                        client_data: *mut *mut ::libc::c_void) -> herr_t;
    pub fn H5Epush1(file: *const ::libc::c_char, func: *const ::libc::c_char,
                    line: ::libc::c_uint, maj: H5E_major_t, min: H5E_minor_t,
                    str: *const ::libc::c_char) -> herr_t;
    pub fn H5Eprint1(stream: *mut FILE) -> herr_t;
    pub fn H5Eset_auto1(func: H5E_auto1_t, client_data: *mut ::libc::c_void)
     -> herr_t;
    pub fn H5Ewalk1(direction: H5E_direction_t, func: H5E_walk1_t,
                    client_data: *mut ::libc::c_void) -> herr_t;
    pub fn H5Eget_major(maj: H5E_major_t) -> *mut ::libc::c_char;
    pub fn H5Eget_minor(min: H5E_minor_t) -> *mut ::libc::c_char;
}
