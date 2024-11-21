// TODO: Indentation support
pub trait CppHeaderGen {
    fn gen_cpp_header(&self, indent: usize) -> String;
}

pub const INDENT_SIZE: usize = 4;
