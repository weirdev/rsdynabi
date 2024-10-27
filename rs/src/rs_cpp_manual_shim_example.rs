use crate::manual_shim_cpp_class1::CppClass1___constructor__;

pub fn run_rs_cpp_manual_shim_example_test() {
    let cpp_class1 = unsafe { CppClass1___constructor__(42) };
    println!("cpp_class1: {:?}", cpp_class1);
}