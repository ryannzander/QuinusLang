//! C code generator - transpiles to C, compiled with system compiler

use crate::ast::{EnumVariant, *};
use crate::error::Result;
use crate::semantic::AnnotatedProgram;
use std::collections::HashMap;

#[derive(Clone)]
struct Ctx {
    vars: HashMap<String, String>,
    var_types: HashMap<String, Type>,
    program: Option<std::sync::Arc<crate::ast::Program>>,
}

impl Default for Ctx {
    fn default() -> Self {
        Self { vars: HashMap::new(), var_types: HashMap::new(), program: None }
    }
}

fn type_to_c(ty: &Type) -> String {
    match ty {
        Type::Int => "long".to_string(),
        Type::Float => "double".to_string(),
        Type::Bool => "int".to_string(),
        Type::Str => "char*".to_string(),
        Type::Void => "void".to_string(),
        Type::Array(_) => "long*".to_string(),
        Type::ArraySized(inner, _) => type_to_c(inner).trim_end_matches('*').to_string(),
        Type::Named(name) => match name.as_str() {
            "Point" => "Point*".to_string(),
            _ => "void*".to_string(),
        },
        Type::U8 => "uint8_t".to_string(),
        Type::U16 => "uint16_t".to_string(),
        Type::U32 => "uint32_t".to_string(),
        Type::U64 => "uint64_t".to_string(),
        Type::I8 => "int8_t".to_string(),
        Type::I16 => "int16_t".to_string(),
        Type::I32 => "int32_t".to_string(),
        Type::I64 => "int64_t".to_string(),
        Type::Usize => "size_t".to_string(),
        Type::F32 => "float".to_string(),
        Type::F64 => "double".to_string(),
        Type::Ptr(inner) => format!("{}*", type_to_c(inner).trim_end_matches('*')),
        Type::Tuple(inner) => {
            let parts: Vec<String> = inner.iter().enumerate()
                .map(|(i, t)| format!("{} _{}", type_to_c(t), i))
                .collect();
            format!("struct {{ {} }}", parts.join("; "))
        }
    }
}

fn decl_to_c(ty: &Type, name: &str) -> String {
    match ty {
        Type::ArraySized(inner, n) => format!("{} {}[{}]", type_to_c(inner).trim_end_matches('*'), name, n),
        _ => format!("{} {}", type_to_c(ty), name),
    }
}

fn type_to_printf(ty: &Type) -> &'static str {
    match ty {
        Type::Int | Type::I32 | Type::I64 => "%ld",
        Type::U8 | Type::U16 | Type::U32 | Type::U64 | Type::Usize => "%lu",
        Type::Float | Type::F32 | Type::F64 => "%f",
        Type::Bool => "%d",
        Type::Str => "%s",
        Type::Void => "%s",
        Type::Array(_) | Type::ArraySized(_, _) | Type::Named(_) => "%p",
        Type::I8 | Type::I16 => "%d",
        Type::Ptr(_) => "%p",
        Type::Tuple(_) => "%p",
    }
}

fn expr_type(expr: &Expr, ctx: &Ctx) -> Option<Type> {
    match expr {
        Expr::Literal(l) => Some(match l {
            Literal::Int(_) => Type::Int,
            Literal::Float(_) => Type::Float,
            Literal::Bool(_) => Type::Bool,
            Literal::Str(_) => Type::Str,
        }),
        Expr::Ident(name) => ctx.var_types.get(name).cloned(),
        Expr::AddrOf(operand) => expr_type(operand, ctx).map(|t| Type::Ptr(Box::new(t))),
        Expr::Deref(operand) => match expr_type(operand, ctx) {
            Some(Type::Ptr(t)) => Some(*t),
            _ => None,
        },
        Expr::Field { base, .. } => expr_type(base, ctx),
        _ => None,
    }
}

pub fn generate(program: &AnnotatedProgram) -> Result<String> {
    let mut out = String::new();
    out.push_str("#include <stdlib.h>\n");
    out.push_str("#include <stdint.h>\n");
    out.push_str("#include <stdio.h>\n");
    out.push_str("#include <string.h>\n");

    let mut ctx = Ctx::default();
    ctx.program = Some(std::sync::Arc::new(program.program.clone()));
    for item in &program.program.items {
        emit_top_level(&mut out, item, &mut ctx)?;
    }
    Ok(out)
}

fn find_method_struct(program: &crate::ast::Program, method_name: &str) -> Option<String> {
    for item in &program.items {
        if let TopLevelItem::Impl(impl_def) = item {
            if impl_def.methods.iter().any(|m| m.name == method_name) {
                return Some(impl_def.struct_name.clone());
            }
        }
    }
    None
}

fn emit_top_level(out: &mut String, item: &TopLevelItem, ctx: &mut Ctx) -> Result<()> {
    match item {
        TopLevelItem::Const(c) => {
            out.push_str(&format!("static const {} = ", decl_to_c(&c.ty, &c.name)));
            emit_expr(out, &c.init, ctx)?;
            out.push_str(";\n\n");
        }
        TopLevelItem::Static(s) => {
            out.push_str(&format!("static {} ", decl_to_c(&s.ty, &s.name)));
            if let Some(init) = &s.init {
                out.push_str(" = ");
                emit_expr(out, init, ctx)?;
            }
            out.push_str(";\n\n");
        }
        TopLevelItem::Fn(f) => emit_fn(out, f, ctx)?,
        TopLevelItem::Struct(s) => emit_struct(out, s)?,
        TopLevelItem::Class(c) => emit_class(out, c, ctx)?,
        TopLevelItem::Enum(e) => emit_enum(out, e)?,
        TopLevelItem::Union(u) => emit_union(out, u)?,
        TopLevelItem::Mod(m) => {
            for sub in &m.items {
                emit_top_level(out, sub, ctx)?;
            }
        }
        TopLevelItem::Import(_) => {}
        TopLevelItem::Alias(_) => {}
        TopLevelItem::Impl(impl_def) => {
            for m in &impl_def.methods {
                let ret = m.return_type.as_ref().map(type_to_c).unwrap_or_else(|| "void".to_string());
                let struct_ty = impl_def.struct_name.clone();
                out.push_str(&format!("{} {}_{}(", ret, struct_ty, m.name));
                out.push_str(&format!("{}* self", struct_ty));
                for p in &m.params {
                    if p.name != "self" {
                        out.push_str(&format!(", {} {}", type_to_c(&p.ty), p.name));
                    }
                }
                out.push_str(") {\n");
                let mut fn_ctx = Ctx::default();
                fn_ctx.program = ctx.program.clone();
                fn_ctx.vars.insert("self".to_string(), "self".to_string());
                fn_ctx.var_types.insert("self".to_string(), Type::Ptr(Box::new(Type::Named(impl_def.struct_name.clone()))));
                for p in &m.params {
                    if p.name != "self" {
                        fn_ctx.vars.insert(p.name.clone(), p.name.clone());
                        fn_ctx.var_types.insert(p.name.clone(), p.ty.clone());
                    }
                }
                for stmt in &m.body {
                    emit_stmt(out, stmt, &mut fn_ctx)?;
                }
                out.push_str("}\n\n");
            }
        }
    }
    Ok(())
}

fn emit_enum(out: &mut String, e: &EnumDef) -> Result<()> {
    out.push_str(&format!("typedef enum {{\n"));
    for (i, v) in e.variants.iter().enumerate() {
        let vname = match v {
            EnumVariant::Unit(n) => n.clone(),
            EnumVariant::Tuple(n, _) => n.clone(),
        };
        out.push_str(&format!("    {} = {}", vname, i));
        if i < e.variants.len() - 1 {
            out.push_str(",");
        }
        out.push_str("\n");
    }
    out.push_str(&format!("}} {};\n\n", e.name));
    Ok(())
}

fn emit_union(out: &mut String, u: &UnionDef) -> Result<()> {
    out.push_str(&format!("typedef union {{\n"));
    for f in &u.fields {
        out.push_str(&format!("    {} {};\n", type_to_c(&f.ty), f.name));
    }
    out.push_str(&format!("}} {};\n\n", u.name));
    Ok(())
}

fn emit_struct(out: &mut String, s: &StructDef) -> Result<()> {
    out.push_str("typedef struct {\n");
    for f in &s.fields {
        out.push_str(&format!("    {} {};\n", type_to_c(&f.ty), f.name));
    }
    out.push_str("} ");
    out.push_str(&s.name);
    out.push_str(";\n\n");
    Ok(())
}

fn emit_class(out: &mut String, c: &ClassDef, _ctx: &mut Ctx) -> Result<()> {
    out.push_str("typedef struct {\n");
    for f in &c.fields {
        out.push_str(&format!("    {} {};\n", type_to_c(&f.ty), f.name));
    }
    out.push_str("} ");
    out.push_str(&c.name);
    out.push_str(";\n\n");

    if let Some(init) = &c.init {
        out.push_str(&format!("void {}_init({}* this", c.name, c.name));
        for p in &init.params {
            out.push_str(&format!(", {} {}", type_to_c(&p.ty), p.name));
        }
        out.push_str(") {\n");
        let mut init_ctx = Ctx::default();
        init_ctx.vars.insert("this".to_string(), "this".to_string());
        for p in &init.params {
            init_ctx.vars.insert(p.name.clone(), p.name.clone());
        }
        for stmt in &init.body {
            emit_stmt(out, stmt, &mut init_ctx)?;
        }
        out.push_str("}\n\n");
    }

    for m in &c.methods {
        let ret = m.return_type.as_ref().map(type_to_c).unwrap_or_else(|| "void".to_string());
        out.push_str(&format!("{} {}_{}({}* this", ret, c.name, m.name, c.name));
        for p in &m.params {
            out.push_str(&format!(", {} {}", type_to_c(&p.ty), p.name));
        }
        out.push_str(") {\n");
        let mut method_ctx = Ctx::default();
        method_ctx.vars.insert("this".to_string(), "this".to_string());
        for p in &m.params {
            method_ctx.vars.insert(p.name.clone(), p.name.clone());
        }
        for stmt in &m.body {
            emit_stmt(out, stmt, &mut method_ctx)?;
        }
        out.push_str("}\n\n");
    }
    Ok(())
}

fn emit_fn(out: &mut String, f: &FnDef, ctx: &mut Ctx) -> Result<()> {
    let ret = f.return_type.as_ref().map(type_to_c).unwrap_or_else(|| "void".to_string());
    out.push_str(&format!("{} {}(", ret, f.name));
    for (i, p) in f.params.iter().enumerate() {
        if i > 0 {
            out.push_str(", ");
        }
        out.push_str(&format!("{} {}", type_to_c(&p.ty), p.name));
    }
    out.push_str(") {\n");
    let mut fn_ctx = Ctx::default();
    fn_ctx.program = ctx.program.clone();
    for p in &f.params {
        fn_ctx.vars.insert(p.name.clone(), p.name.clone());
        fn_ctx.var_types.insert(p.name.clone(), p.ty.clone());
    }
    for stmt in &f.body {
        emit_stmt(out, stmt, &mut fn_ctx)?;
    }
    out.push_str("}\n\n");
    Ok(())
}

fn emit_stmt(out: &mut String, stmt: &Stmt, ctx: &mut Ctx) -> Result<()> {
    match stmt {
        Stmt::VarDecl { name, ty, init, mutable: _ } => {
            let decl_ty = ty.clone().unwrap_or(Type::Int);
            ctx.vars.insert(name.clone(), name.clone());
            ctx.var_types.insert(name.clone(), decl_ty.clone());
            let decl = decl_to_c(&decl_ty, name);
            out.push_str(&format!("    {} = ", decl));
            emit_expr(out, init, ctx)?;
            out.push_str(";\n");
        }
        Stmt::Assign { target, value } => {
            out.push_str("    ");
            emit_assign_target(out, target, ctx)?;
            out.push_str(" = ");
            emit_expr(out, value, ctx)?;
            out.push_str(";\n");
        }
        Stmt::If { cond, then_body, else_body } => {
            out.push_str("    if (");
            emit_expr(out, cond, ctx)?;
            out.push_str(") {\n");
            for s in then_body {
                emit_stmt(out, s, ctx)?;
            }
            out.push_str("    }");
            if let Some(else_b) = else_body {
                out.push_str(" else {\n");
                for s in else_b {
                    emit_stmt(out, s, ctx)?;
                }
                out.push_str("    }");
            }
            out.push_str("\n");
        }
        Stmt::For { init, cond, step, body } => {
            out.push_str("    for (");
            if let Some(i) = init {
                emit_for_init(out, i, ctx)?;
            }
            out.push_str("; ");
            if let Some(c) = cond {
                emit_expr(out, c, ctx)?;
            }
            out.push_str("; ");
            if let Some(s) = step {
                emit_for_step(out, s, ctx)?;
            }
            out.push_str(") {\n");
            for s in body {
                emit_stmt(out, s, ctx)?;
            }
            out.push_str("    }\n");
        }
        Stmt::While { cond, body } => {
            out.push_str("    while (");
            emit_expr(out, cond, ctx)?;
            out.push_str(") {\n");
            for s in body {
                emit_stmt(out, s, ctx)?;
            }
            out.push_str("    }\n");
        }
        Stmt::Foreach { var, iter, body } => {
            if let Expr::Range { start, end } = iter.as_ref() {
                out.push_str("    {\n");
                out.push_str(&format!("    int {};\n", var));
                out.push_str(&format!("    for ({} = ", var));
                emit_expr(out, start, ctx)?;
                out.push_str(&format!("; {} < ", var));
                emit_expr(out, end, ctx)?;
                out.push_str(&format!("; {}++) {{\n", var));
                ctx.vars.insert(var.clone(), var.clone());
                ctx.var_types.insert(var.clone(), Type::I32);
                for s in body {
                    emit_stmt(out, s, ctx)?;
                }
                out.push_str("    }\n");
                out.push_str("    }\n");
            } else {
                let idx = "_foreach_i";
                out.push_str("    {\n");
                out.push_str(&format!("    size_t {};\n", idx));
                out.push_str(&format!("    for ({} = 0; {} < sizeof(", idx, idx));
                emit_expr(out, iter, ctx)?;
                out.push_str(")/sizeof((");
                emit_expr(out, iter, ctx)?;
                out.push_str(")[0]); ");
                out.push_str(&format!("{}++) {{\n", idx));
                out.push_str("        long ");
                out.push_str(var);
                out.push_str(" = (");
                emit_expr(out, iter, ctx)?;
                out.push_str(")[");
                out.push_str(idx);
                out.push_str("];\n");
                ctx.vars.insert(var.clone(), var.clone());
                ctx.var_types.insert(var.clone(), Type::Int);
                for s in body {
                    emit_stmt(out, s, ctx)?;
                }
                out.push_str("    }\n");
                out.push_str("    }\n");
            }
        }
        Stmt::Break => out.push_str("    break;\n"),
        Stmt::Continue => out.push_str("    continue;\n"),
        Stmt::Hazard { body } => {
            for s in body {
                emit_stmt(out, s, ctx)?;
            }
        }
        Stmt::InlineAsm { instructions } => {
            for instr in instructions {
                let escaped = instr.replace('\\', "\\\\").replace('"', "\\\"");
                out.push_str(&format!(
                    "    #if defined(__GNUC__) || defined(__clang__)\n    __asm__ __volatile__(\"{}\");\n    #else\n    /* MSVC x64: inline asm not supported - use MinGW/Clang for machine blocks */\n    #endif\n",
                    escaped
                ));
            }
        }
        Stmt::ExprStmt(e) => {
            out.push_str("    ");
            emit_expr(out, e, ctx)?;
            out.push_str(";\n");
        }
        Stmt::Return(expr) => {
            out.push_str("    return");
            if let Some(e) = expr {
                out.push_str(" ");
                emit_expr(out, e, ctx)?;
            }
            out.push_str(";\n");
        }
        Stmt::TryCatch { try_body, .. } => {
            for s in try_body {
                emit_stmt(out, s, ctx)?;
            }
        }
        Stmt::Defer { body } => {
            for s in body {
                emit_stmt(out, s, ctx)?;
            }
        }
        Stmt::Choose { expr, arms } => {
            out.push_str("    {\n    int _choose_val = (int)(");
            emit_expr(out, expr, ctx)?;
            out.push_str(");\n");
            for (i, arm) in arms.iter().enumerate() {
                let prefix = if i == 0 { "    if " } else { "    else if " };
                let cond = match &arm.pattern {
                    ChoosePattern::UnitVariant(_v) => format!("_choose_val == {}", i),
                    ChoosePattern::TupleVariant(_, _) => format!("_choose_val == {}", i),
                    ChoosePattern::Ident(_) => "1".to_string(),
                };
                out.push_str(&format!("{}({}) {{\n", prefix, cond));
                for s in &arm.body {
                    emit_stmt(out, s, ctx)?;
                }
                out.push_str("    }\n");
            }
            out.push_str("    }\n");
        }
    }
    Ok(())
}

fn emit_for_init(out: &mut String, stmt: &Stmt, ctx: &mut Ctx) -> Result<()> {
    match stmt {
        Stmt::VarDecl { name, ty, init, mutable: _ } => {
            let decl_ty = ty.clone().unwrap_or(Type::Int);
            ctx.vars.insert(name.clone(), name.clone());
            ctx.var_types.insert(name.clone(), decl_ty.clone());
            let decl = decl_to_c(&decl_ty, name);
            out.push_str(&format!("{} = ", decl));
            emit_expr(out, init, ctx)?;
        }
        _ => {}
    }
    Ok(())
}

fn emit_for_step(out: &mut String, stmt: &Stmt, ctx: &Ctx) -> Result<()> {
    match stmt {
        Stmt::Assign { target, value } => {
            emit_assign_target(out, target, ctx)?;
            out.push_str(" = ");
            emit_expr(out, value, ctx)?;
        }
        _ => {}
    }
    Ok(())
}

fn emit_assign_target(out: &mut String, target: &AssignTarget, ctx: &Ctx) -> Result<()> {
    match target {
        AssignTarget::Ident(name) => out.push_str(name),
        AssignTarget::Field { base, field } => {
            emit_expr(out, base, ctx)?;
            out.push_str(&format!("->{}", field));
        }
        AssignTarget::Index { base, index } => {
            emit_expr(out, base, ctx)?;
            out.push_str("[");
            emit_expr(out, index, ctx)?;
            out.push_str("]");
        }
        AssignTarget::Deref(operand) => {
            out.push_str("*");
            emit_expr(out, operand, ctx)?;
        }
    }
    Ok(())
}

fn emit_expr(out: &mut String, expr: &Expr, ctx: &Ctx) -> Result<()> {
    match expr {
        Expr::Literal(Literal::Int(n)) => out.push_str(&n.to_string()),
        Expr::Literal(Literal::Float(_)) => out.push_str("0.0"),
        Expr::Literal(Literal::Bool(b)) => out.push_str(if *b { "1" } else { "0" }),
        Expr::Literal(Literal::Str(s)) => {
            out.push_str(&format!("\"{}\"", s.replace('\\', "\\\\").replace('"', "\\\"")));
        }
        Expr::Ident(name) => out.push_str(name),
        Expr::Binary { op, left, right } => {
            let lt = expr_type(left, ctx);
            let rt = expr_type(right, ctx);
            if *op == BinOp::Add && lt == Some(Type::Str) && rt == Some(Type::Str) {
                out.push_str("({ const char* _a = ");
                emit_expr(out, left, ctx)?;
                out.push_str("; const char* _b = ");
                emit_expr(out, right, ctx)?;
                out.push_str("; size_t _la = strlen(_a); size_t _lb = strlen(_b); char* _r = (char*)malloc(_la + _lb + 1); memcpy(_r, _a, _la + 1); strcat(_r, _b); _r; })");
            } else {
            out.push_str("(");
            emit_expr(out, left, ctx)?;
            out.push_str(" ");
            out.push_str(match op {
                BinOp::Add => "+",
                BinOp::Sub => "-",
                BinOp::Mul => "*",
                BinOp::Div => "/",
                BinOp::Mod => "%",
                BinOp::Eq => "==",
                BinOp::Ne => "!=",
                BinOp::Lt => "<",
                BinOp::Le => "<=",
                BinOp::Gt => ">",
                BinOp::Ge => ">=",
                BinOp::And => "&&",
                BinOp::Or => "||",
            });
            out.push_str(" ");
            emit_expr(out, right, ctx)?;
            out.push_str(")");
            }
        }
        Expr::Unary { op, operand } => {
            out.push_str(match op {
                UnOp::Neg => "(-",
                UnOp::Not => "(!",
            });
            emit_expr(out, operand, ctx)?;
            out.push_str(")");
        }
        Expr::Call { callee, args } => {
            if let Expr::Field { base, field } = callee.as_ref() {
                if let Some(ref prog) = ctx.program {
                    if let Some(struct_name) = find_method_struct(prog, field) {
                        out.push_str(&format!("{}_{}(", struct_name, field));
                        let base_ty = expr_type(base, ctx);
                        let need_addr = matches!(base_ty, Some(Type::Named(_)));
                        if need_addr {
                            out.push_str("&(");
                        }
                        emit_expr(out, base, ctx)?;
                        if need_addr {
                            out.push_str(")");
                        }
                        for arg in args {
                            out.push_str(", ");
                            emit_expr(out, arg, ctx)?;
                        }
                        out.push_str(")");
                        return Ok(());
                    }
                }
            }
            let callee_name = callee.as_ref();
            let is_print = matches!(callee_name, Expr::Ident(n) if n == "print");
            let is_writeln = matches!(callee_name, Expr::Ident(n) if n == "writeln");
            let is_write = matches!(callee_name, Expr::Ident(n) if n == "write");
            let is_read = matches!(callee_name, Expr::Ident(n) if n == "read");
            let is_len = matches!(callee_name, Expr::Ident(n) if n == "len");
            let is_strlen = matches!(callee_name, Expr::Ident(n) if n == "strlen");
            let is_panic = matches!(callee_name, Expr::Ident(n) if n == "panic");
            let is_assert = matches!(callee_name, Expr::Ident(n) if n == "assert");
            if is_print || is_writeln {
                let mut fmt_parts = Vec::new();
                for arg in args {
                    let fmt = expr_type(arg, ctx)
                        .as_ref()
                        .map(type_to_printf)
                        .unwrap_or("%ld");
                    fmt_parts.push(fmt);
                }
                let fmt_str = fmt_parts.join(" ");
                let suffix = if is_writeln || is_print { "\\n\"" } else { "\"" };
                out.push_str(&format!("printf(\"{}{}", fmt_str, suffix));
                for arg in args {
                    out.push_str(", ");
                    emit_expr(out, arg, ctx)?;
                }
                out.push_str(")");
            } else if is_write {
                let mut fmt_parts = Vec::new();
                for arg in args {
                    let fmt = expr_type(arg, ctx)
                        .as_ref()
                        .map(type_to_printf)
                        .unwrap_or("%ld");
                    fmt_parts.push(fmt);
                }
                let fmt_str = fmt_parts.join(" ");
                out.push_str(&format!("printf(\"{}\"", fmt_str));
                for arg in args {
                    out.push_str(", ");
                    emit_expr(out, arg, ctx)?;
                }
                out.push_str(")");
            } else if is_read {
                out.push_str("({ int _r; scanf(\"%d\", &_r); (int32_t)_r; })");
            } else if is_strlen {
                if let Some(arg) = args.first() {
                    out.push_str("((size_t)strlen(");
                    emit_expr(out, arg, ctx)?;
                    out.push_str("))");
                }
            } else if is_panic {
                out.push_str("(fprintf(stderr, \"panic\\n\"), exit(1), (void)0)");
            } else if is_assert {
                if let Some(arg) = args.first() {
                    out.push_str("((");
                    emit_expr(out, arg, ctx)?;
                    out.push_str(") ? (void)0 : (fprintf(stderr, \"assertion failed\\n\"), exit(1)))");
                }
            } else if is_len {
                if let Some(arg) = args.first() {
                    if let Some(ty) = expr_type(arg, ctx) {
                        match ty {
                            Type::ArraySized(_, n) => out.push_str(&n.to_string()),
                            _ => {
                                out.push_str("(sizeof(");
                                emit_expr(out, arg, ctx)?;
                                out.push_str(") / sizeof((");
                                emit_expr(out, arg, ctx)?;
                                out.push_str(")[0]))");
                            }
                        }
                    } else {
                        out.push_str("(sizeof(");
                        emit_expr(out, arg, ctx)?;
                        out.push_str(") / sizeof((");
                        emit_expr(out, arg, ctx)?;
                        out.push_str(")[0]))");
                    }
                }
            } else if let Expr::Ident(name) = callee.as_ref() {
                out.push_str(name);
                out.push_str("(");
                for (i, arg) in args.iter().enumerate() {
                    if i > 0 {
                        out.push_str(", ");
                    }
                    emit_expr(out, arg, ctx)?;
                }
                out.push_str(")");
            } else {
                emit_expr(out, callee, ctx)?;
                out.push_str("(");
                for (i, arg) in args.iter().enumerate() {
                    if i > 0 {
                        out.push_str(", ");
                    }
                    emit_expr(out, arg, ctx)?;
                }
                out.push_str(")");
            }
        }
        Expr::Index { base, index } => {
            emit_expr(out, base, ctx)?;
            out.push_str("[");
            emit_expr(out, index, ctx)?;
            out.push_str("]");
        }
        Expr::Slice { base, start, end: _ } => {
            out.push_str("(&(");
            emit_expr(out, base, ctx)?;
            out.push_str(")[");
            if let Some(s) = start {
                emit_expr(out, s, ctx)?;
            } else {
                out.push_str("0");
            }
            out.push_str("])");
        }
        Expr::Field { base, field } => {
            emit_expr(out, base, ctx)?;
            out.push_str(&format!("->{}", field));
        }
        Expr::AddrOf(operand) => {
            out.push_str("&");
            emit_expr(out, operand, ctx)?;
        }
        Expr::Deref(operand) => {
            out.push_str("*");
            emit_expr(out, operand, ctx)?;
        }
        Expr::ArrayInit(elems) => {
            out.push_str("{ ");
            for (i, e) in elems.iter().enumerate() {
                if i > 0 {
                    out.push_str(", ");
                }
                emit_expr(out, e, ctx)?;
            }
            out.push_str(" }");
        }
        Expr::Range { start, end } => {
            out.push_str("((");
            emit_expr(out, start, ctx)?;
            out.push_str(") <= (");
            emit_expr(out, end, ctx)?;
            out.push_str("))");
        }
        Expr::Tuple(elems) => {
            out.push_str("(");
            for (i, e) in elems.iter().enumerate() {
                if i > 0 {
                    out.push_str(", ");
                }
                emit_expr(out, e, ctx)?;
            }
            out.push_str(")");
        }
        Expr::Interpolate(parts) => {
            let mut fmt_parts = Vec::new();
            let mut args = Vec::new();
            for p in parts {
                match p {
                    InterpolatePart::Str(s) => fmt_parts.push(s.replace('%', "%%").replace('\\', "\\\\").replace('"', "\\\"")),
                    InterpolatePart::Expr(e) => {
                        fmt_parts.push("%ld".to_string());
                        args.push(e.clone());
                    }
                }
            }
            out.push_str("(printf(\"");
            for fp in &fmt_parts {
                out.push_str(fp);
            }
            out.push_str("\\n\"");
            for a in &args {
                out.push_str(", ");
                emit_expr(out, a, ctx)?;
            }
            out.push_str("), (void)0)");
        }
        Expr::New { class, args } => {
            out.push_str(&format!(
                "(({{ {}* _p = ({}*)malloc(sizeof({})); ",
                class, class, class
            ));
            if !args.is_empty() {
                out.push_str(&format!("{}_init(_p", class));
                for arg in args {
                    out.push_str(", ");
                    emit_expr(out, arg, ctx)?;
                }
                out.push_str("); ");
            }
            out.push_str("_p; }))");
        }
    }
    Ok(())
}
