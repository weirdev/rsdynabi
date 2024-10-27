mod ldyn;
mod rs_only_example;
mod manual_shim_cpp_class1;
mod rs_cpp_manual_shim_example;
mod manual_interop_cpp_class1;
mod rs_cpp_manual_interop_example;

use rs_only_example::*;
use rs_cpp_manual_shim_example::*;
use rs_cpp_manual_interop_example::*;

fn main() {
    println!("RS only example:");
    run_rs_only_example_test();
    println!("RS CPP manual shim example:");
    run_rs_cpp_manual_shim_example_test();
    println!("RS CPP manual interop example:");
    run_rs_cpp_manual_interop_example_test();
}
