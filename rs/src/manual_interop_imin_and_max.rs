use crate::manual_interop_imin::IMin;
use crate::manual_interop_imax::IMax;
use crate::manual_shim_imin_and_max::*;

pub trait IMinAndMax : IMin + IMax {} // IMin and IMax required for IMinAndMax

pub struct IMinAndMaxAdapter {
    pub ptr: *mut ::libc::c_void,
}

impl IMin for IMinAndMaxAdapter {
    fn min(&self) -> i32 {
        unsafe { IMinAndMax_min(self.ptr) }
    }
}

impl IMax for IMinAndMaxAdapter {
    fn max(&self) -> i32 {
        unsafe { IMinAndMax_max(self.ptr) }
    }
}

impl IMinAndMax for IMinAndMaxAdapter {}

impl Drop for IMinAndMaxAdapter {
    fn drop(&mut self) {
        unsafe { IMinAndMax___destructor__(self.ptr) }
    }
}
