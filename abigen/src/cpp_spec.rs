use crate::cpp_gen::{CppHeaderGen, INDENT_SIZE};

pub struct CppHeaderDecl {
    pub includes: Vec<String>, // TODO: Model includes
    pub classes: Vec<CppClassDecl>,
}

pub struct CppClassDecl {
    pub name: String,
    // TODO: Generics
    // TODO: Inheritance
    pub variables: Vec<CppVariableDecl>,
    pub constructors: Vec<CppConstructorDeclOrDef>,
    pub methods: Vec<CppMethodDeclOrDef>,
}

pub struct CppVariableDecl {
    pub name: String,
    pub ty: CppTypeSpecifier,
}

pub enum CppConstructorDeclOrDef {
    Decl(CppConstructorDecl),
    Def(CppConstructorDef),
}

pub struct CppConstructorDecl {
    pub args: Vec<CppVariableDecl>,
}

pub struct CppConstructorDef {
    pub args: Vec<CppVariableDecl>,
    pub var_init: Vec<CppConstructorVarInit>,
}

pub struct CppConstructorVarInit {
    pub var_name: String,
    pub expr: String, // TODO: Model expressions
}

pub enum CppMethodDeclOrDef {
    Decl(CppMethodDecl),
    Def(CppMethodDef),
}

pub struct CppMethodDecl {
    pub name: String,
    pub args: Vec<CppVariableDecl>,
    pub ret_ty: CppTypeSpecifier,
    pub is_const: bool,
}

pub struct CppMethodDef {
    pub name: String,
    pub args: Vec<CppVariableDecl>,
    pub ret_ty: CppTypeSpecifier,
    pub is_const: bool,
    pub body: String, // TODO: Model expressions
}

pub enum CppRefSpecifier {
    None,
    Pointer,
    LValue,
    RValue,
}

pub struct CppTypeSpecifier {
    pub name: String,
    pub is_const: bool,
    pub ref_spec: CppRefSpecifier,
    // TODO: is_array: bool,
    // TODO: Generics
    // TODO: array_size: Option<usize>,
}

impl CppHeaderGen for CppHeaderDecl {
    fn gen_cpp_header(&self, indent: usize) -> String {
        let mut header = String::new();
        header.push_str("#pragma once\n\n");
        for include in &self.includes {
            header.push_str(&format!("#include {}\n", include));
        }
        for class in &self.classes {
            header.push_str("\n");
            header.push_str(&class.gen_cpp_header(indent));
            header.push_str("\n");
        }
        header
    }
}

impl CppHeaderGen for CppClassDecl {
    fn gen_cpp_header(&self, indent: usize) -> String {
        let mut header = String::new();
        header.push_str(&" ".repeat(indent));
        header.push_str(&format!("class {} {{\n", self.name));

        // TODO: Specified access specifiers
        header.push_str(&" ".repeat(indent + (INDENT_SIZE / 2) as usize));
        header.push_str("public:\n");

        for var in &self.variables {
            header.push_str(&" ".repeat(indent + INDENT_SIZE));
            header.push_str(&format!("{};\n", var.gen_cpp_header(indent + INDENT_SIZE)));
        }
        header.push_str("\n");

        for constructor in &self.constructors {
            header.push_str(&" ".repeat(indent + INDENT_SIZE));
            header.push_str(&format!(
                "{}{}\n",
                self.name,
                constructor.gen_cpp_header(indent + INDENT_SIZE)
            ));
            header.push_str("\n");
        }

        for method in &self.methods {
            header.push_str(&" ".repeat(indent + INDENT_SIZE));
            header.push_str(&format!(
                "{}\n",
                method.gen_cpp_header(indent + INDENT_SIZE)
            ));
            header.push_str("\n");
        }
        header
    }
}

impl CppHeaderGen for CppVariableDecl {
    fn gen_cpp_header(&self, indent: usize) -> String {
        format!("{} {}", self.ty.gen_cpp_header(indent), self.name)
    }
}

impl CppHeaderGen for CppConstructorDeclOrDef {
    fn gen_cpp_header(&self, indent: usize) -> String {
        match self {
            CppConstructorDeclOrDef::Decl(decl) => decl.gen_cpp_header(indent),
            CppConstructorDeclOrDef::Def(def) => def.gen_cpp_header(indent),
        }
    }
}

impl CppHeaderGen for CppConstructorDef {
    fn gen_cpp_header(&self, indent: usize) -> String {
        let mut header = String::new();
        header.push_str("(");
        for (i, arg) in self.args.iter().enumerate() {
            if i > 0 {
                header.push_str(",");
            }
            header.push_str("\n");
            header.push_str(&" ".repeat(indent + (2 * INDENT_SIZE)));
            header.push_str(&format!(
                "{}",
                arg.gen_cpp_header(indent + (2 * INDENT_SIZE))
            ));
        }
        header.push_str(")");
        if !self.var_init.is_empty() {
            header.push_str(" : ");
            for (i, init) in self.var_init.iter().enumerate() {
                if i > 0 {
                    header.push_str(",");
                }
                header.push_str("\n");
                header.push_str(&" ".repeat(indent + (3 * INDENT_SIZE)));
                header.push_str(&format!(
                    "{} {{}}",
                    init.gen_cpp_header(indent + (3 * INDENT_SIZE))
                ));
            }
        }
        header
    }
}

impl CppHeaderGen for CppConstructorDecl {
    fn gen_cpp_header(&self, indent: usize) -> String {
        let mut header = String::new();
        header.push_str("(");
        for (i, arg) in self.args.iter().enumerate() {
            if i > 0 {
                header.push_str(",");
            }
            header.push_str("\n");
            header.push_str(&" ".repeat(indent + (2 * INDENT_SIZE)));
            header.push_str(&format!(
                "{}",
                arg.gen_cpp_header(indent + (2 * INDENT_SIZE))
            ));
        }
        header.push_str(");");
        header
    }
}

impl CppHeaderGen for CppConstructorVarInit {
    fn gen_cpp_header(&self, _indent: usize) -> String {
        format!("{}({})", self.var_name, self.expr)
    }
}

impl CppHeaderGen for CppMethodDeclOrDef {
    fn gen_cpp_header(&self, indent: usize) -> String {
        match self {
            CppMethodDeclOrDef::Decl(decl) => decl.gen_cpp_header(indent),
            CppMethodDeclOrDef::Def(def) => def.gen_cpp_header(indent),
        }
    }
}

impl CppHeaderGen for CppMethodDecl {
    fn gen_cpp_header(&self, indent: usize) -> String {
        let mut header = String::new();
        header.push_str(&format!(
            "{} {}(",
            self.ret_ty.gen_cpp_header(indent),
            self.name
        ));
        for (i, arg) in self.args.iter().enumerate() {
            if i > 0 {
                header.push_str(",");
            }
            header.push_str("\n");
            header.push_str(&" ".repeat(indent + (2 * INDENT_SIZE)));
            header.push_str(&format!(
                "{}",
                arg.gen_cpp_header(indent + (2 * INDENT_SIZE))
            ));
        }
        header.push_str(")");
        if self.is_const {
            header.push_str(" const");
        }
        header.push_str(";");
        header
    }
}

impl CppHeaderGen for CppMethodDef {
    fn gen_cpp_header(&self, indent: usize) -> String {
        let mut header = String::new();
        header.push_str(&format!(
            "{} {}(",
            self.ret_ty.gen_cpp_header(indent),
            self.name
        ));
        for (i, arg) in self.args.iter().enumerate() {
            if i > 0 {
                header.push_str(",");
            }
            header.push_str("\n");
            header.push_str(&" ".repeat(indent + (2 * INDENT_SIZE)));
            header.push_str(&format!(
                "{}",
                arg.gen_cpp_header(indent + (2 * INDENT_SIZE))
            ));
        }
        header.push_str(") ");
        if self.is_const {
            header.push_str("const ");
        }
        header.push_str("{\n");
        header.push_str(&" ".repeat(indent + INDENT_SIZE));
        header.push_str(&format!("{}\n", self.body));
        header.push_str(&" ".repeat(indent));
        header.push_str("}");
        header
    }
}

impl CppHeaderGen for CppTypeSpecifier {
    fn gen_cpp_header(&self, indent: usize) -> String {
        let mut header = String::new();
        if self.is_const {
            header.push_str("const ");
        }
        header.push_str(&self.name);
        header.push_str(&self.ref_spec.gen_cpp_header(indent));
        header
    }
}

impl CppHeaderGen for CppRefSpecifier {
    fn gen_cpp_header(&self, _indent: usize) -> String {
        match self {
            CppRefSpecifier::None => "".to_string(),
            CppRefSpecifier::Pointer => "*".to_string(),
            CppRefSpecifier::LValue => "&".to_string(),
            CppRefSpecifier::RValue => "&&".to_string(),
        }
    }
}
