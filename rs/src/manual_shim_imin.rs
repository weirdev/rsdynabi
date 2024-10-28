use ::libc::c_void;

extern "C" {
    pub fn IMin___destructor__(this: *mut c_void);

    pub fn IMin_min(iMin: *const c_void) -> i32;
}
