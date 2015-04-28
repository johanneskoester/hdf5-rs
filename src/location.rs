
use ffi::hid_t;

pub trait Location {
    fn loc_id(&self) -> hid_t;
}

