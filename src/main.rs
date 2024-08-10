use std::fmt::{Display, Formatter};

fn main() {
    // C++ statically available
    let extern_display1_cleanup_impl = |arg: *mut core::ffi::c_void| {
        let arg = arg as *mut i8;
        let _string = unsafe { std::ffi::CString::from_raw(arg) };
        // Drop the string
    };
    let extern_display1_impl1 = |arg: *mut core::ffi::c_void| -> DynArg {
        let arg = arg as *mut String;
        let string = unsafe { &*arg };
        let result = format!("{}", string);
        let result = std::ffi::CString::new(result).unwrap();
        let result = result.into_raw(); // Leak the memory

        DynArg {
            arg: result as *mut core::ffi::c_void,
            trait_impls: vec![],
            cleanup: &extern_display1_cleanup_impl,
        }
    };
    let extern_val1_cleanup_impl = |arg: *mut core::ffi::c_void| {
        let arg = arg as *mut String;
        let _string = unsafe { Box::from_raw(arg) };
        // Drop the string
    };

    {
        let adapter = {
            let arg = {
                // C++ constructs this at call time:
                let extern_val1 = Box::new("Hello, world!".to_string());
                let extern_val1 = Box::into_raw(extern_val1);

                // C++ sends this to Rust
                DynArg {
                    arg: extern_val1 as *mut core::ffi::c_void,
                    trait_impls: vec![&extern_display1_impl1],
                    cleanup: &extern_val1_cleanup_impl,
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

trait DisplayRsAdapter<'a>: Display {
    fn to_string_idx() -> usize;
    fn get_dyn_arg(&self) -> &DynArg<'a>;
    fn adapt_fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        let arg = self.get_dyn_arg();
        let idx = Self::to_string_idx();
        let result = arg.call_dyn_fn(idx);
        unsafe {
            let result = std::ffi::CString::from_raw(result.arg as *mut i8);
            let fmt_output = write!(f, "{}", result.to_str().unwrap());
            std::mem::forget(result); // Don't drop
            fmt_output
        }
    }
}

impl<'a> Display for GeneratedAdapter1<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        self.adapt_fmt(f)
    }
}

struct GeneratedAdapter1<'a> {
    arg: DynArg<'a>,
}

impl<'a> DisplayRsAdapter<'a> for GeneratedAdapter1<'a> {
    fn to_string_idx() -> usize {
        0
    }

    fn get_dyn_arg(&self) -> &DynArg<'a> {
        &self.arg
    }
}

// Must not impl copy or clone on this directly
// Rust Clone/copy would cause double free on drop
// Clone/copy can be one of the extern trait implementations
struct DynArg<'a> {
    arg: *mut core::ffi::c_void,
    trait_impls: Vec<&'a dyn Fn(*mut core::ffi::c_void) -> DynArg<'a>>,
    cleanup: &'a dyn Fn(*mut core::ffi::c_void),
}

impl<'a> DynArg<'a> {
    pub fn call_dyn_fn(&self, idx: usize) -> DynArg<'a> {
        let trait_impl = self.trait_impls[idx];
        trait_impl(self.arg)
    }
}

impl<'a> Drop for DynArg<'a> {
    fn drop(&mut self) {
        (self.cleanup)(self.arg);
    }
}
