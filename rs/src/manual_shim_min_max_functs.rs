use ::libc::c_void;

extern "C" {
    pub fn MinMaxFuncts_getMinnable() -> *mut c_void;

    pub fn MinMaxFuncts_getMaxable() -> *mut c_void;
}
