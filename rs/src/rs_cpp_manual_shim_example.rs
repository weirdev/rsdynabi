use crate::manual_shim_cpp_class1::*;

pub fn run_rs_cpp_manual_shim_example_test() {
    let cpp_class1 = unsafe { CppClass1___constructor__(42) };
    let field1 = unsafe { CppClass1_getField1(cpp_class1) };
    println!("field1 from rs: {}", field1);
    println!("field1 from cpp: ");
    unsafe { CppClass1_printField1(cpp_class1) };
    println!("cpp_class1: {:?}", cpp_class1);
    unsafe { CppClass1___destructor__(cpp_class1) };
}