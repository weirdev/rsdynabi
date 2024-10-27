use crate::manual_interop_cpp_class1::*;

pub fn run_rs_cpp_manual_interop_example_test() {
    let cpp_class1 = CppClass1Adapter::new(50);
    let field1 = cpp_class1.get_field1();
    println!("field1 from rs: {}", field1);
    println!("field1 from cpp: ");
    cpp_class1.print_field1();
    println!("Dropping");
}