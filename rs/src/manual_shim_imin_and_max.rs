use ::libc::c_void;

extern "C" {
    pub fn IMinAndMax___destructor__(this: *mut c_void);

    pub fn IMinAndMax_min(iMinAndMax: *const c_void) -> i32;

    pub fn IMinAndMax_max(iMinAndMax: *const c_void) -> i32;
}
