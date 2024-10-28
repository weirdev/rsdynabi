use crate::{manual_interop_cpp_class2::*, manual_interop_imax::IMax, manual_interop_imin::IMin};

pub fn run_rs_cpp_manual_interop_example2_test() {
    let cpp_class2 = CppClass2Adapter::new(7, 8, 9);
    let field1 = cpp_class2.get_field1();
    println!("field1: {}", field1);
    // Either only min called for max or max called for min, because pointer offsetting in cast to correct parent class not done
    let max = cpp_class2.max();
    println!("max: {}", max);
    let min = cpp_class2.min();
    println!("min: {}", min);
}