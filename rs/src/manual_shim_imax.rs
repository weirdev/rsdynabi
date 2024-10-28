use ::libc::c_void;

extern "C" {
    pub fn IMax___destructor__(this: *mut c_void);

    pub fn IMax_max(imax: *const c_void) -> i32;
}
