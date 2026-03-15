//! C code generator - transpiles to C, compiled with system compiler

use crate::ast::*;
use crate::error::Result;
use crate::semantic::AnnotatedProgram;
use std::collections::HashMap;

#[derive(Clone, Default)]
struct Ctx {
    vars: HashMap<String, String>,
    var_types: HashMap<String, Type>,
}

fn type_to_c(ty: &Type) -> &'static str {
    match ty {
        Type::Int => "long",
        Type::Float => "double",
        Type::Bool => "int",
        Type::Str => "char*",
        Type::Void => "void",
        Type::Array(_) => "long*",
        Type::Named(name) => match name.as_str() {
            "Point" => "Point*",
            _ => "void*",
        },
    }
}

fn type_to_printf(ty: &Type) -> &'static str {
    match ty {
        Type::Int => "%ld",
        Type::Float => "%f",
        Type::Bool => "%d",
        Type::Str => "%s",
        Type::Void => "%s",
        Type::Array(_) => "%p",
        Type::Named(_) => "%p",
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
        _ => None,
    }
}

pub fn generate(program: &AnnotatedProgram) -> Result<String> {
    let mut out = String::new();
    out.push_str("#include <stdlib.h>\n");
    out.push_str("#include <stdio.h>\n");

    let mut ctx = Ctx::default();
    for item in &program.program.items {
        emit_top_level(&mut out, item, &mut ctx)?;
    }
    Ok(out)
}

fn emit_top_level(out: &mut String, item: &TopLevelItem, ctx: &mut Ctx) -> Result<()> {
    match item {
        TopLevelItem::Fn(f) => emit_fn(out, f, ctx)?,
        TopLevelItem::Struct(s) => emit_struct(out, s)?,
        TopLevelItem::Class(c) => emit_class(out, c, ctx)?,
        TopLevelItem::Mod(m) => {
            for sub in &m.items {
                emit_top_level(out, sub, ctx)?;
            }
        }
        TopLevelItem::Import(_) => {}
    }
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
        let ret = m.return_type.as_ref().map(type_to_c).unwrap_or("void");
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

fn emit_fn(out: &mut String, f: &FnDef, _ctx: &mut Ctx) -> Result<()> {
    let ret = f.return_type.as_ref().map(type_to_c).unwrap_or("void");
    out.push_str(&format!("{} {}(", ret, f.name));
    for (i, p) in f.params.iter().enumerate() {
        if i > 0 {
            out.push_str(", ");
        }
        out.push_str(&format!("{} {}", type_to_c(&p.ty), p.name));
    }
    out.push_str(") {\n");
    let mut fn_ctx = Ctx::default();
    for p in &f.params {
        fn_ctx.vars.insert(p.name.clone(), p.name.clone());
    }
    for stmt in &f.body {
        emit_stmt(out, stmt, &mut fn_ctx)?;
    }
    out.push_str("}\n\n");
    Ok(())
}

fn emit_stmt(out: &mut String, stmt: &Stmt, ctx: &mut Ctx) -> Result<()> {
    match stmt {
        Stmt::VarDecl { name, ty, init } => {
            let cty = ty.as_ref().map(type_to_c).unwrap_or("long");
            ctx.vars.insert(name.clone(), name.clone());
            if let Some(t) = ty.as_ref() {
                ctx.var_types.insert(name.clone(), t.clone());
            } else {
                ctx.var_types.insert(name.clone(), Type::Int);
            }
            out.push_str(&format!("    {} {} = ", cty, name));
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
    }
    Ok(())
}

fn emit_for_init(out: &mut String, stmt: &Stmt, ctx: &mut Ctx) -> Result<()> {
    match stmt {
        Stmt::VarDecl { name, ty, init } => {
            let cty = ty.as_ref().map(type_to_c).unwrap_or("long");
            ctx.vars.insert(name.clone(), name.clone());
            if let Some(t) = ty.as_ref() {
                ctx.var_types.insert(name.clone(), t.clone());
            } else {
                ctx.var_types.insert(name.clone(), Type::Int);
            }
            out.push_str(&format!("{} {} = ", cty, name));
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
        Expr::Unary { op, operand } => {
            out.push_str(match op {
                UnOp::Neg => "(-",
                UnOp::Not => "(!",
            });
            emit_expr(out, operand, ctx)?;
            out.push_str(")");
        }
        Expr::Call { callee, args } => {
            let is_print = matches!(callee.as_ref(), Expr::Ident(n) if n == "print");
            if is_print {
                let mut fmt_parts = Vec::new();
                for arg in args {
                    let fmt = expr_type(arg, ctx)
                        .as_ref()
                        .map(type_to_printf)
                        .unwrap_or("%ld");
                    fmt_parts.push(fmt);
                }
                let fmt_str = fmt_parts.join(" ");
                out.push_str(&format!("printf(\"{}\\n\"", fmt_str));
                for arg in args {
                    out.push_str(", ");
                    emit_expr(out, arg, ctx)?;
                }
                out.push_str(")");
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
        Expr::Field { base, field } => {
            emit_expr(out, base, ctx)?;
            out.push_str(&format!("->{}", field));
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
