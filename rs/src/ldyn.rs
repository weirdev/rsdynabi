use libc::size_t;

#[repr(C)]
pub(crate) struct DynArg {
    pub(crate) arg: *mut core::ffi::c_void,
    pub(crate) trait_impls: *const extern "C" fn(*mut core::ffi::c_void) -> DynArg,
    pub(crate) trait_impls_len: size_t,
    pub(crate) cleanup: *const extern "C" fn(*mut core::ffi::c_void),
}

impl<'a> DynArg {
    pub fn call_dyn_fn(&self, idx: usize) -> DynArg {
        if idx < self.trait_impls_len as usize {
            let trait_impl: *const extern "C" fn(*mut std::ffi::c_void) -> DynArg =
                unsafe { self.trait_impls.add(idx) };
            unsafe { (*trait_impl)(self.arg) }
        } else {
            panic!("Index out of bounds");
        }
    }
}

impl<'a> Drop for DynArg {
    fn drop(&mut self) {
        if !self.cleanup.is_null() {
            unsafe { (*self.cleanup)(self.arg) };
        }
    }
}
