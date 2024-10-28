use crate::manual_shim_imin::*;

pub trait IMin {
    fn min(&self) -> i32;
}

pub struct IMinAdapter {
    pub ptr: *mut ::libc::c_void,
}

impl IMin for IMinAdapter {
    fn min(&self) -> i32 {
        unsafe { IMin_min(self.ptr) }
    }
}

impl Drop for IMinAdapter {
    fn drop(&mut self) {
        unsafe { IMin___destructor__(self.ptr) }
    }
}

// trait IMinAdapterTrait: IMin {
//     fn index() -> usize;
//     fn get_dyn_arg(&self) -> &ldyn::DynArg;
//     fn adapt(&self) -> i32 {
//         let arg: &ldyn::DynArg = self.get_dyn_arg();
//         let idx = Self::index();
//         let result = arg.call_dyn_fn(idx);
//         unsafe { *(result.arg as *mut i32) }
//     }
// }
