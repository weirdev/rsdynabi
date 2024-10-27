use crate::manual_shim_cpp_class1::*;

pub struct CppClass1Adapter {
    // NOTE: Doesn't need to be a DynArg because there are no virtual functions
    // NOTE: Should we always just use DynArg?
    cpp_class1: *mut ::libc::c_void,
}

impl CppClass1Adapter {
    pub fn new(input: i32) -> Self {
        let cpp_class1 = unsafe { CppClass1___constructor__(input) };
        Self { cpp_class1 }
    }

    pub fn get_field1(&self) -> i32 {
        unsafe { CppClass1_getField1(self.cpp_class1) }
    }

    pub fn print_field1(&self) {
        unsafe { CppClass1_printField1(self.cpp_class1) };
    }
}

impl Drop for CppClass1Adapter {
    fn drop(&mut self) {
        unsafe { CppClass1___destructor__(self.cpp_class1) };
    }
}