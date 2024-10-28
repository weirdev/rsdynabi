use ::libc::c_void;

extern "C" {
    pub fn CppClass2___constructor__(field1: i32, field2: i32, field3: i32) -> *mut c_void;

    pub fn CppClass2___destructor__(cpp_class1: *mut c_void);

    pub fn CppClass2_getField1(cpp_class1: *const c_void) -> i32;

    // Would have the non-virtual versions of min and max here, but leaving out for testing
}
