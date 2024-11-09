use crate::manual_shim_cpp_class2::*;
use crate::manual_interop_imax;
use crate::manual_interop_imin;

pub struct CppClass2Adapter {
    // NOTE: Doesn't need to be a DynArg because there are no virtual functions
    // NOTE: Should we always just use DynArg?
    cpp_class2: *mut ::libc::c_void,
}

impl CppClass2Adapter {
    pub fn new(field1: i32, field2: i32, field3: i32) -> Self {
        let cpp_class2 = unsafe { CppClass2___constructor__(field1, field2, field3) };
        Self { cpp_class2 }
    }

    pub fn get_field1(&self) -> i32 {
        unsafe { CppClass2_getField1(self.cpp_class2) }
    }
}

impl Drop for CppClass2Adapter {
    fn drop(&mut self) {
        unsafe { CppClass2___destructor__(self.cpp_class2) };
    }
}

impl manual_interop_imax::IMax for CppClass2Adapter {
    fn max(&self) -> i32 {
        unsafe { CppClass2_max(self.cpp_class2) }
    }
}

impl manual_interop_imin::IMin for CppClass2Adapter {
    fn min(&self) -> i32 {
        unsafe { CppClass2_min(self.cpp_class2) }
    }
} 
