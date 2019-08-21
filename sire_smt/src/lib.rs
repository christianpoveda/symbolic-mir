use sire::sir::*;

use crate::smtlib::ToSmtlib;

pub mod smtlib;
mod z3;

#[derive(Debug, PartialEq, Eq)]
pub enum CheckResult {
    Sat,
    Unsat,
    Unknown,
}

pub fn check_equality(a: &FuncDef, b: &FuncDef) -> Result<CheckResult, Box<dyn std::error::Error>> {
    if let (Ty::Func(a_args_ty), Ty::Func(b_args_ty)) = (&a.ty, &b.ty) {
        if a_args_ty == b_args_ty {
            let mut args_with_ty = String::new();
            let mut args = String::new();

            for (i, ty) in a_args_ty.iter().enumerate().skip(1) {
                let smt_ty = ty.to_smtlib();
                args_with_ty += &format!("(x{} {}) ", i, smt_ty);
                args += &format!("x{} ", i);
            }

            let mut code = String::new();
            code += &a.to_smtlib();
            code += "\n";
            code += &b.to_smtlib();
            code += "\n";
            code += &format!(
                "(assert (forall ({}) (= ({} {}) ({} {}))))",
                args_with_ty,
                a.def_id.to_smtlib(),
                args,
                b.def_id.to_smtlib(),
                args
            );
            code += "\n";
            code += "(check-sat)";

            println!("code: {:?}", code);

            let result = z3::call(&code)?;
            Ok(if result == "sat\n" {
                CheckResult::Sat
            } else if result == "unsat\n" {
                CheckResult::Unsat
            } else if result == "unknown\n" {
                CheckResult::Unknown
            } else {
                panic!("Unknown z3 output: {:?}", result)
            })
        } else {
            Ok(CheckResult::Unsat)
        }
    } else {
        Ok(CheckResult::Unsat)
    }
}