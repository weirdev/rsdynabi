use ::libc::c_void;

extern "C" {
    pub fn IMin_min(iMin: *const c_void) -> i32;
}
