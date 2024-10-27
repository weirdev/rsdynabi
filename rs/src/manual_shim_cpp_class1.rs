use ::libc::c_void;

extern "C" {
    pub fn CppClass1___constructor__(input: i32) -> *mut c_void;

    pub fn CppClass1___destructor__(cpp_class1: *mut c_void);

    pub fn CppClass1_getField1(cpp_class1: *const c_void) -> i32;

    pub fn CppClass1_printField1(cpp_class1: *const c_void);
}
