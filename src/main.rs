use libc::size_t;
use std::{
    fmt::{Display, Formatter},
    ptr,
};

fn main() {
    {
        let adapter = {
            let arg = {
                // C++ constructs this at call time:
                let extern_val1 = Box::new("Hello, world!".to_string());
                let extern_val1 = Box::into_raw(extern_val1);

                // Note: The explicit type is necessary here,
                // otherwise rust tries to be "helpful"
                // and makes the type specific to the function,
                // then optimizes away actually storing a value.
                let dynfns: Box<[extern "C" fn(*mut std::ffi::c_void) -> DynArg]> =
                    Box::new([extern_string_display_impl]);
                let trait_impls_len = dynfns.len() as size_t;
                // let dynfns = dynfns.as_ptr();
                let dynfns = Box::into_raw(dynfns);
                let dynfns = unsafe { (*dynfns).as_ptr() };

                let cleanup = &(extern_val1_cleanup_impl as extern "C" fn(*mut core::ffi::c_void));

                // C++ sends this to Rust
                DynArg {
                    arg: extern_val1 as *mut core::ffi::c_void,
                    trait_impls: dynfns,
                    trait_impls_len,
                    cleanup,
                }
            };

            // Rust adapter layer
            GeneratedAdapter1 { arg: arg }
        };

        // Rust calls the adapter
        println!("Rust dyn calling C++ obj: {}", adapter);

        // Cleanup starts
        println!("Cleanup starts");
    }

    // Cleanup completed
    println!("Cleanup completed");
}

// C++ statically available
extern "C" fn extern_string_cleanup_impl(arg: *mut core::ffi::c_void) {
    let arg = arg as *mut i8;
    let _string = unsafe { std::ffi::CString::from_raw(arg) };
    // Drop the string
}

extern "C" fn extern_string_display_impl(arg: *mut core::ffi::c_void) -> DynArg {
    let arg = arg as *mut String;
    let string = unsafe { &*arg };
    let result = format!("{}", string);
    let result = std::ffi::CString::new(result).unwrap();
    let result = result.into_raw(); // Leak the memory

    let cleanup = &(extern_string_cleanup_impl as extern "C" fn(*mut core::ffi::c_void));

    DynArg {
        arg: result as *mut core::ffi::c_void,
        trait_impls: ptr::null(),
        trait_impls_len: 0,
        cleanup,
    }
}

extern "C" fn extern_val1_cleanup_impl(arg: *mut core::ffi::c_void) {
    let arg = arg as *mut String;
    let _string = unsafe { Box::from_raw(arg) };
    // Drop the string
}

// Rust types

#[repr(C)]
struct DynArg {
    arg: *mut core::ffi::c_void,
    trait_impls: *const extern "C" fn(*mut core::ffi::c_void) -> DynArg,
    trait_impls_len: size_t,
    cleanup: *const extern "C" fn(*mut core::ffi::c_void),
}

impl<'a> DynArg {
    pub fn call_dyn_fn(&self, idx: usize) -> DynArg {
        if idx < self.trait_impls_len as usize {
            let trait_impl: *const extern "C" fn(*mut std::ffi::c_void) -> DynArg =
                unsafe { self.trait_impls.add(idx) };
            unsafe { (*trait_impl)(self.arg) }
        } else {
            panic!("Index out of bounds");
        }
    }
}

impl<'a> Drop for DynArg {
    fn drop(&mut self) {
        if !self.cleanup.is_null() {
            unsafe { (*self.cleanup)(self.arg) };
        }
    }
}

trait DisplayRsAdapter : Display {
    fn to_string_idx() -> usize;
    fn get_dyn_arg(&self) -> &DynArg;
    fn adapt_fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        let arg: &DynArg = self.get_dyn_arg();
        let idx = Self::to_string_idx();
        let result = arg.call_dyn_fn(idx);
        unsafe {
            // Generator logic will need to support C++ string -> Rust conversion
            let result = std::ffi::CStr::from_ptr(result.arg as *const i8);
            let fmt_output = write!(f, "{}", result.to_str().unwrap());
            fmt_output
        }
    }
}

impl Display for GeneratedAdapter1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        self.adapt_fmt(f)
    }
}

struct GeneratedAdapter1 {
    arg: DynArg,
}

impl DisplayRsAdapter for GeneratedAdapter1 {
    fn to_string_idx() -> usize {
        0
    }

    fn get_dyn_arg(&self) -> &DynArg {
        &self.arg
    }
}

// "C++" types

struct Matrix {
    data: Vec<f64>,
    rows: usize,
    cols: usize,
}

struct Matrix3D {
    data: Vec<f64>,
    rows: usize,
    cols: usize,
    depth: usize,
}

// C++ interfaces

trait TensorMax {
    fn tmax(&self) -> f64;
}

impl Display for Matrix {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "Matrix({}, {})", self.rows, self.cols)
    }
}

impl TensorMax for Matrix {
    fn tmax(&self) -> f64 {
        self.data.iter().cloned().fold(f64::NEG_INFINITY, f64::max)
    }
}

impl Display for Matrix3D {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "Matrix3D({}, {}, {})", self.rows, self.cols, self.depth)
    }
}

impl TensorMax for Matrix3D {
    fn tmax(&self) -> f64 {
        self.data.iter().cloned().fold(f64::NEG_INFINITY, f64::max)
    }
}

// C++ generated code
extern "C" fn extern_matrix_cleanup_impl(arg: *mut core::ffi::c_void) {
    let arg = arg as *mut Matrix;
    let _box = unsafe { Box::from_raw(arg) };
    // Drop the box
}

extern "C" fn extern_matrix_display_impl(arg: *mut core::ffi::c_void) -> DynArg {
    let arg = arg as *mut Matrix;
    let matrix = unsafe { &*arg };
    let result = format!("{}", matrix);
    let result = std::ffi::CString::new(result).unwrap();
    let result = result.into_raw(); // Leak the memory

    let cleanup = &(extern_string_cleanup_impl as extern "C" fn(*mut core::ffi::c_void));

    DynArg {
        arg: result as *mut core::ffi::c_void,
        trait_impls: ptr::null(),
        trait_impls_len: 0,
        cleanup,
    }
}

extern "C" fn extern_double_cleanup_impl(arg: *mut core::ffi::c_void) {
    let arg = arg as *mut f64;
    let _double = unsafe { Box::from_raw(arg) };
    // Drop the double
}

extern "C" fn extern_matrix_tmax_impl(arg: *mut core::ffi::c_void) -> DynArg {
    let arg = arg as *mut Matrix;
    let matrix = unsafe { &*arg };
    let result = matrix.tmax();
    let result = Box::new(result);
    let result = Box::into_raw(result); // Leak the memory

    let cleanup = &(extern_double_cleanup_impl as extern "C" fn(*mut core::ffi::c_void));

    DynArg {
        arg: result as *mut core::ffi::c_void,
        trait_impls: ptr::null(),
        trait_impls_len: 0,
        cleanup,
    }
}

impl Into<DynArg> for Box<Matrix> {
    fn into(self) -> DynArg {
        let result = Box::into_raw(self);

        // Note: The explicit type is necessary here,
        // otherwise rust tries to be "helpful"
        // and makes the type specific to the function,
        // then optimizes away actually storing a value.
        let dynfns: Box<[extern "C" fn(*mut std::ffi::c_void) -> DynArg]> =
            Box::new([extern_matrix_display_impl, extern_matrix_tmax_impl]);
        let trait_impls_len = dynfns.len() as size_t;
        // let dynfns = dynfns.as_ptr();
        let dynfns = Box::into_raw(dynfns);
        let dynfns = unsafe { (*dynfns).as_ptr() };
        
        let cleanup = &(extern_matrix_cleanup_impl as extern "C" fn(*mut core::ffi::c_void));

        DynArg {
            arg: result as *mut core::ffi::c_void,
            trait_impls: dynfns,
            trait_impls_len,
            cleanup,
        }
    }
}

extern "C" fn extern_matrix3d_cleanup_impl(arg: *mut core::ffi::c_void) {
    let arg = arg as *mut Matrix3D;
    let _box = unsafe { Box::from_raw(arg) };
    // Drop the box
}

extern "C" fn extern_matrix3d_display_impl(arg: *mut core::ffi::c_void) -> DynArg {
    let arg = arg as *mut Matrix3D;
    let matrix = unsafe { &*arg };
    let result = format!("{}", matrix);
    let result = std::ffi::CString::new(result).unwrap();
    let result = result.into_raw(); // Leak the memory

    let cleanup = &(extern_string_cleanup_impl as extern "C" fn(*mut core::ffi::c_void));

    DynArg {
        arg: result as *mut core::ffi::c_void,
        trait_impls: ptr::null(),
        trait_impls_len: 0,
        cleanup,
    }
}

extern "C" fn extern_matrix3d_tmax_impl(arg: *mut core::ffi::c_void) -> DynArg {
    let arg = arg as *mut Matrix;
    let matrix = unsafe { &*arg };
    let result = matrix.tmax();
    let result = Box::new(result);
    let result = Box::into_raw(result); // Leak the memory

    let cleanup = &(extern_double_cleanup_impl as extern "C" fn(*mut core::ffi::c_void));

    DynArg {
        arg: result as *mut core::ffi::c_void,
        trait_impls: ptr::null(),
        trait_impls_len: 0,
        cleanup,
    }
}

impl Into<DynArg> for Box<Matrix3D> {
    fn into(self) -> DynArg {
        let result = Box::into_raw(self);

        // Note: The explicit type is necessary here,
        // otherwise rust tries to be "helpful"
        // and makes the type specific to the function,
        // then optimizes away actually storing a value.
        let dynfns: Box<[extern "C" fn(*mut std::ffi::c_void) -> DynArg]> =
            Box::new([extern_matrix3d_display_impl, extern_matrix3d_tmax_impl]);
        let trait_impls_len = dynfns.len() as size_t;
        // let dynfns = dynfns.as_ptr();
        let dynfns = Box::into_raw(dynfns);
        let dynfns = unsafe { (*dynfns).as_ptr() };
        
        let cleanup = &(extern_matrix3d_cleanup_impl as extern "C" fn(*mut core::ffi::c_void));

        DynArg {
            arg: result as *mut core::ffi::c_void,
            trait_impls: dynfns,
            trait_impls_len,
            cleanup,
        }
    }
}

// Rust generated code

struct MatrixAdapter {
    arg: DynArg,
}

impl Display for MatrixAdapter {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        self.adapt_fmt(f)
    }
}

impl DisplayRsAdapter for MatrixAdapter {
    fn to_string_idx() -> usize {
        0
    }

    fn get_dyn_arg(&self) -> &DynArg {
        &self.arg
    }
}

trait TensorMaxRsAdapter : TensorMax {
    fn tmax_idx() -> usize;
    fn get_dyn_arg(&self) -> &DynArg;
    fn adapt_tmax(&self) -> f64 {
        let arg: &DynArg = self.get_dyn_arg();
        let idx = Self::tmax_idx();
        let result = arg.call_dyn_fn(idx);
        unsafe { *(result.arg as *mut f64) }
    }
}

impl TensorMax for MatrixAdapter {
    fn tmax(&self) -> f64 {
        self.adapt_tmax()
    }
}

impl TensorMaxRsAdapter for MatrixAdapter {
    fn tmax_idx() -> usize {
        1
    }

    fn get_dyn_arg(&self) -> &DynArg {
        &self.arg
    }
}
