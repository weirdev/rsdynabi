use crate::ldyn;

pub trait IMin {
    fn min(&self) -> i32;
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
