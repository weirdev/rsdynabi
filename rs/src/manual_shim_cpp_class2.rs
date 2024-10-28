use ::libc::c_void;

extern "C" {
    pub fn CppClass2___constructor__(field1: i32, field2: i32, field3: i32) -> *mut c_void;

    pub fn CppClass2___destructor__(cpp_class2: *mut c_void);

    pub fn CppClass2_getField1(cpp_class2: *const c_void) -> i32;

    pub fn CppClass2_max(cpp_class2: *const c_void) -> i32;

    pub fn CppClass2_min(cpp_class2: *const c_void) -> i32;
}
