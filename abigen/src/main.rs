mod cpp_gen;
mod cpp_spec;

use cpp_gen::CppHeaderGen;
use cpp_spec::{
    CppClassDecl, CppConstructorDeclOrDef, CppConstructorDef, CppConstructorVarInit, CppHeaderDecl,
    CppMethodDeclOrDef, CppMethodDef, CppRefSpecifier, CppTypeSpecifier, CppVariableDecl,
};

fn gen_cpp_class1_header() {
    let class1 = CppClassDecl {
        name: "CppClass1".to_string(),
        variables: vec![CppVariableDecl {
            name: "field1".to_string(),
            ty: CppTypeSpecifier {
                name: "int".to_string(),
                is_const: false,
                ref_spec: CppRefSpecifier::None,
            },
        }],
        constructors: vec![CppConstructorDeclOrDef::Def(CppConstructorDef {
            args: vec![CppVariableDecl {
                name: "field1".to_string(),
                ty: CppTypeSpecifier {
                    name: "int".to_string(),
                    is_const: false,
                    ref_spec: CppRefSpecifier::None,
                },
            }],
            var_init: vec![CppConstructorVarInit {
                var_name: "field1".to_string(),
                expr: "field1".to_string(),
            }],
        })],
        methods: vec![
            CppMethodDeclOrDef::Def(CppMethodDef {
                name: "getField1".to_string(),
                args: vec![],
                ret_ty: CppTypeSpecifier {
                    name: "int".to_string(),
                    is_const: false,
                    ref_spec: CppRefSpecifier::None,
                },
                is_const: true,
                body: "return field1;".to_string(),
            }),
            CppMethodDeclOrDef::Def(CppMethodDef {
                name: "printField1".to_string(),
                args: vec![],
                ret_ty: CppTypeSpecifier {
                    name: "void".to_string(),
                    is_const: false,
                    ref_spec: CppRefSpecifier::None,
                },
                is_const: true,
                body: "std::cout << getField1() << std::endl;".to_string(),
            }),
        ],
    };

    let header = CppHeaderDecl {
        includes: vec!["<iostream>".to_string()],
        classes: vec![class1],
    };

    println!("{}", header.gen_cpp_header(0));
}

fn main() {
    gen_cpp_class1_header();
}
