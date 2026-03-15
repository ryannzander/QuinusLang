//! Basic formatter for QuinusLang

use crate::ast::*;
use std::fmt::Write;

/// Format a program to a string with consistent style.
pub fn format_program(program: &Program) -> String {
    let mut out = String::new();
    for (i, item) in program.items.iter().enumerate() {
        if i > 0 {
            out.push_str("\n");
        }
        format_top_level(&mut out, item);
    }
    if !out.is_empty() && !out.ends_with('\n') {
        out.push('\n');
    }
    out
}

fn format_top_level(out: &mut String, item: &TopLevelItem) {
    match item {
        TopLevelItem::Fn(f) => format_fn(out, f, false),
        TopLevelItem::Const(c) => {
            let _ = write!(out, "eternal {}: {} = ", c.name, c.ty);
            format_expr(out, &c.init);
            let _ = writeln!(out, ";");
        }
        TopLevelItem::Static(s) => {
            let _ = write!(out, "anchor {}: {}", s.name, s.ty);
            if let Some(init) = &s.init {
                let _ = write!(out, " = ");
                format_expr(out, init);
            }
            let _ = writeln!(out, ";");
        }
        TopLevelItem::Struct(s) => {
            let _ = writeln!(out, "form {} {{", s.name);
            for f in &s.fields {
                let _ = writeln!(out, "    {}: {},", f.name, f.ty);
            }
            let _ = writeln!(out, "}}");
        }
        TopLevelItem::Mod(m) => {
            let _ = writeln!(out, "realm {} {{", m.name);
            for sub in &m.items {
                format_top_level_indent(out, sub, 1);
            }
            let _ = writeln!(out, "}}");
        }
        TopLevelItem::Import(i) => {
            let _ = writeln!(out, "bring {};", i.path.join("."));
        }
        TopLevelItem::Extern(e) => {
            let _ = write!(out, "extern craft {}(", e.name);
            for (i, p) in e.params.iter().enumerate() {
                if i > 0 {
                    out.push_str(", ");
                }
                let _ = write!(out, "{}: {}", p.name, p.ty);
            }
            let _ = write!(out, ")");
            if let Some(rt) = &e.return_type {
                let _ = write!(out, " -> {}", rt);
            }
            let _ = writeln!(out, ";");
        }
        _ => {}
    }
}

fn format_top_level_indent(out: &mut String, item: &TopLevelItem, indent: usize) {
    let pad = "    ".repeat(indent);
    match item {
        TopLevelItem::Fn(f) => {
            let _ = write!(out, "{}", pad);
            format_fn(out, f, false);
        }
        _ => format_top_level(out, item),
    }
}

fn format_fn(out: &mut String, f: &FnDef, _open: bool) {
    let _ = write!(out, "craft {}(", f.name);
    for (i, p) in f.params.iter().enumerate() {
        if i > 0 {
            out.push_str(", ");
        }
        let _ = write!(out, "{}: {}", p.name, p.ty);
    }
    let _ = write!(out, ")");
    if let Some(rt) = &f.return_type {
        let _ = write!(out, " -> {}", rt);
    }
    let _ = writeln!(out, " {{");
    for stmt in &f.body {
        format_stmt(out, stmt, 1);
    }
    let _ = writeln!(out, "}}");
}

fn format_stmt(out: &mut String, stmt: &Stmt, indent: usize) {
    let pad = "    ".repeat(indent);
    match stmt {
        Stmt::VarDecl {
            name,
            ty,
            init,
            mutable,
        } => {
            let _ = write!(out, "{}make ", pad);
            if *mutable {
                out.push_str("shift ");
            }
            let _ = write!(out, "{}", name);
            if let Some(t) = ty {
                let _ = write!(out, ": {}", t);
            }
            let _ = write!(out, " = ");
            format_expr(out, init);
            let _ = writeln!(out, ";");
        }
        Stmt::VarDeclTuple {
            names,
            init,
            mutable,
        } => {
            let _ = write!(out, "{}make ", pad);
            if *mutable {
                out.push_str("shift ");
            }
            let _ = write!(out, "(");
            for (i, n) in names.iter().enumerate() {
                if i > 0 {
                    out.push_str(", ");
                }
                let _ = write!(out, "{}", n);
            }
            let _ = write!(out, ") = ");
            format_expr(out, init);
            let _ = writeln!(out, ";");
        }
        Stmt::Return(expr) => {
            let _ = write!(out, "{}send", pad);
            if let Some(e) = expr {
                out.push_str(" ");
                format_expr(out, e);
            }
            let _ = writeln!(out, ";");
        }
        Stmt::ExprStmt(e) => {
            let _ = write!(out, "{}", pad);
            format_expr(out, e);
            let _ = writeln!(out, ";");
        }
        Stmt::If {
            cond,
            then_body,
            else_body,
        } => {
            let _ = write!(out, "{}check (", pad);
            format_expr(out, cond);
            let _ = writeln!(out, ") {{");
            for s in then_body {
                format_stmt(out, s, indent + 1);
            }
            if let Some(else_b) = else_body {
                let _ = writeln!(out, "{}otherwise {{", pad);
                for s in else_b {
                    format_stmt(out, s, indent + 1);
                }
                let _ = writeln!(out, "{}}}", pad);
            } else {
                let _ = writeln!(out, "{}}}", pad);
            }
        }
        Stmt::While { cond, body } => {
            let _ = write!(out, "{}loopwhile (", pad);
            format_expr(out, cond);
            let _ = writeln!(out, ") {{");
            for s in body {
                format_stmt(out, s, indent + 1);
            }
            let _ = writeln!(out, "{}}}", pad);
        }
        Stmt::Break => {
            let _ = writeln!(out, "{}stop;", pad);
        }
        Stmt::Continue => {
            let _ = writeln!(out, "{}skip;", pad);
        }
        Stmt::For {
            init,
            cond,
            step,
            body,
        } => {
            let _ = write!(out, "{}for (", pad);
            if let Some(i) = init {
                format_stmt(out, i, 0);
                out.pop();
                out.pop();
            }
            out.push_str("; ");
            if let Some(c) = cond {
                format_expr(out, c);
            }
            out.push_str("; ");
            if let Some(s) = step {
                format_stmt(out, s, 0);
                // Keep semicolon for step (parser expects it)
            }
            let _ = writeln!(out, ") {{");
            for s in body {
                format_stmt(out, s, indent + 1);
            }
            let _ = writeln!(out, "{}}}", pad);
        }
        Stmt::Foreach { var, iter, body } => {
            let _ = write!(out, "{}foreach {} in ", pad, var);
            format_expr(out, iter);
            let _ = writeln!(out, " {{");
            for s in body {
                format_stmt(out, s, indent + 1);
            }
            let _ = writeln!(out, "{}}}", pad);
        }
        Stmt::Defer { body } => {
            let _ = writeln!(out, "{}defer {{", pad);
            for s in body {
                format_stmt(out, s, indent + 1);
            }
            let _ = writeln!(out, "{}}}", pad);
        }
        Stmt::Hazard { body } => {
            let _ = writeln!(out, "{}hazard {{", pad);
            for s in body {
                format_stmt(out, s, indent + 1);
            }
            let _ = writeln!(out, "{}}}", pad);
        }
        Stmt::Choose { expr, arms } => {
            let _ = write!(out, "{}choose (", pad);
            format_expr(out, expr);
            let _ = writeln!(out, ") {{");
            for arm in arms {
                let _ = write!(out, "{}    ", pad);
                match &arm.pattern {
                    ChoosePattern::UnitVariant(v) => {
                        let _ = write!(out, "{} => ", v);
                    }
                    ChoosePattern::TupleVariant(v, fields) => {
                        let _ = write!(out, "{}({}) => ", v, fields.join(", "));
                    }
                    ChoosePattern::Ident(n) => {
                        let _ = write!(out, "{} => ", n);
                    }
                }
                if arm.body.is_empty() {
                    let _ = write!(out, "{{}}");
                } else {
                    for s in &arm.body {
                        format_stmt(out, s, indent + 2);
                    }
                }
            }
            let _ = writeln!(out, "{}}}", pad);
        }
        Stmt::TryCatch {
            try_body,
            catch_param,
            catch_body,
        } => {
            let _ = writeln!(out, "{}try {{", pad);
            for s in try_body {
                format_stmt(out, s, indent + 1);
            }
            let _ = write!(out, "{}catch", pad);
            if let Some(p) = catch_param {
                let _ = write!(out, " ({})", p);
            }
            let _ = writeln!(out, " {{");
            for s in catch_body {
                format_stmt(out, s, indent + 1);
            }
            let _ = writeln!(out, "{}}}", pad);
        }
        Stmt::InlineAsm { instructions } => {
            let _ = writeln!(out, "{}machine {{", pad);
            for instr in instructions {
                let _ = writeln!(
                    out,
                    "{}    \"{}\"",
                    pad,
                    instr.replace('\\', "\\\\").replace('"', "\\\"")
                );
            }
            let _ = writeln!(out, "{}}}", pad);
        }
        Stmt::Assign { target, value } => {
            let _ = write!(out, "{}", pad);
            format_assign_target(out, target);
            let _ = write!(out, " = ");
            format_expr(out, value);
            let _ = writeln!(out, ";");
        }
    }
}

fn format_assign_target(out: &mut String, target: &AssignTarget) {
    match target {
        AssignTarget::Ident(name) => {
            let _ = write!(out, "{}", name);
        }
        AssignTarget::Index { base, index } => {
            format_expr(out, base);
            out.push('[');
            format_expr(out, index);
            out.push(']');
        }
        AssignTarget::Field { base, field } => {
            format_expr(out, base);
            let _ = write!(out, ".{}", field);
        }
        AssignTarget::Deref(operand) => {
            out.push_str("reach ");
            format_expr(out, operand);
        }
    }
}

fn format_expr(out: &mut String, expr: &Expr) {
    match expr {
        Expr::Literal(Literal::Int(n)) => {
            let _ = write!(out, "{}", n);
        }
        Expr::Literal(Literal::Float(f)) => {
            let _ = write!(out, "{}", f);
        }
        Expr::Literal(Literal::Bool(b)) => {
            let _ = write!(out, "{}", b);
        }
        Expr::Literal(Literal::Str(s)) => {
            let _ = write!(out, "\"{}\"", s.replace('\\', "\\\\").replace('"', "\\\""));
        }
        Expr::Ident(name) => {
            let _ = write!(out, "{}", name);
        }
        Expr::Binary { op, left, right } => {
            out.push('(');
            format_expr(out, left);
            out.push_str(match op {
                BinOp::Add => " + ",
                BinOp::Sub => " - ",
                BinOp::Mul => " * ",
                BinOp::Div => " / ",
                BinOp::Mod => " % ",
                BinOp::Eq => " == ",
                BinOp::Ne => " != ",
                BinOp::Lt => " < ",
                BinOp::Le => " <= ",
                BinOp::Gt => " > ",
                BinOp::Ge => " >= ",
                BinOp::And => " && ",
                BinOp::Or => " || ",
            });
            format_expr(out, right);
            out.push(')');
        }
        Expr::Call { callee, args } => {
            format_expr(out, callee);
            out.push('(');
            for (i, a) in args.iter().enumerate() {
                if i > 0 {
                    out.push_str(", ");
                }
                format_expr(out, a);
            }
            out.push(')');
        }
        Expr::ArrayInit(elems) => {
            out.push('{');
            for (i, e) in elems.iter().enumerate() {
                if i > 0 {
                    out.push_str(", ");
                }
                format_expr(out, e);
            }
            out.push('}');
        }
        Expr::AddrOf(operand) => {
            out.push_str("mark ");
            format_expr(out, operand);
        }
        Expr::Deref(operand) => {
            out.push_str("reach ");
            format_expr(out, operand);
        }
        Expr::Index { base, index } => {
            format_expr(out, base);
            out.push('[');
            format_expr(out, index);
            out.push(']');
        }
        Expr::Slice { base, start, end } => {
            format_expr(out, base);
            out.push_str("[");
            if let Some(s) = start {
                format_expr(out, s);
            }
            out.push_str("..");
            if let Some(e) = end {
                format_expr(out, e);
            }
            out.push_str("]");
        }
        Expr::Cast { operand, target_ty } => {
            format_expr(out, operand);
            let _ = write!(out, " as {}", target_ty);
        }
        Expr::Interpolate(parts) => {
            out.push('`');
            for p in parts {
                match p {
                    InterpolatePart::Str(s) => {
                        let escaped = s.replace('\\', "\\\\").replace('`', "\\`");
                        out.push_str(&escaped);
                    }
                    InterpolatePart::Expr(e) => {
                        out.push_str("${");
                        format_expr(out, e);
                        out.push('}');
                    }
                }
            }
            out.push('`');
        }
        _ => {
            out.push_str("/* expr */");
        }
    }
}
