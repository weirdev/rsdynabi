use crate::{
    manual_interop_cpp_class2::*,
    manual_interop_imax::IMax,
    manual_interop_imin::IMin,
    manual_interop_min_max_functs::{getMaxable, getMinMaxable, getMinnable},
};

// Static calls to interface method
pub fn run_rs_cpp_manual_interop_example2_test() {
    let cpp_class2 = CppClass2Adapter::new(7, 8, 9);
    let field1 = cpp_class2.get_field1();
    println!("field1: {}", field1);
    let max = cpp_class2.max();
    println!("max: {}", max);
    let min = cpp_class2.min();
    println!("min: {}", min);

    let minnable = getMinnable();
    let min = minnable.min();
    println!("Minnable min: {}", min);
    let maxable = getMaxable();
    let max = maxable.max();
    println!("Maxable max: {}", max);

    let min_and_maxable = getMinMaxable();
    let min = min_and_maxable.min();
    println!("MinMaxable min: {}", min);
    let max = min_and_maxable.max();
    println!("MinMaxable max: {}", max);
}
