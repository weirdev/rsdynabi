use crate::manual_shim_imax::*;

pub trait IMax {
    fn max(&self) -> i32;
}

pub struct IMaxAdapter {
    pub ptr: *mut ::libc::c_void,
}

impl IMax for IMaxAdapter {
    fn max(&self) -> i32 {
        unsafe { IMax_max(self.ptr) }
    }
}

// trait IMaxAdapterTrait: IMax {
//     fn index() -> usize;
//     fn get_dyn_arg(&self) -> &ldyn::DynArg;
//     fn adapt(&self) -> i32 {
//         let arg: &ldyn::DynArg = self.get_dyn_arg();
//         let idx = Self::index();
//         let result = arg.call_dyn_fn(idx);
//         unsafe { *(result.arg as *mut i32) }
//     }
// }
