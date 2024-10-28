mod ldyn;
mod rs_only_example;
mod manual_shim_cpp_class1;
mod rs_cpp_manual_shim_example;
mod manual_interop_cpp_class1;
mod rs_cpp_manual_interop_example;
mod manual_shim_imax;
mod manual_shim_imin;
mod manual_interop_imax;
mod manual_interop_imin;
mod manual_shim_cpp_class2;
mod manual_interop_cpp_class2;
mod rs_cpp_manual_interop_example2;
mod manual_shim_min_max_functs;
mod manual_interop_min_max_functs;

use rs_only_example::*;
use rs_cpp_manual_shim_example::*;
use rs_cpp_manual_interop_example::*;
use rs_cpp_manual_interop_example2::*;

fn main() {
    println!("RS only example:");
    run_rs_only_example_test();
    println!("RS CPP manual shim example:");
    run_rs_cpp_manual_shim_example_test();
    println!("RS CPP manual interop example:");
    run_rs_cpp_manual_interop_example_test();
    println!("RS CPP manual interop example2:");
    run_rs_cpp_manual_interop_example2_test();
}
