//! Code generator - emits x86/x64 assembly (NASM format)

use crate::ast::*;
use crate::error::Result;
use crate::semantic::AnnotatedProgram;
use std::sync::atomic::{AtomicU32, Ordering};

pub fn generate(program: &AnnotatedProgram) -> Result<String> {
    let mut out = String::new();

    // NASM Windows x64
    out.push_str("section .text\n");
    out.push_str("global main\n");
    out.push_str("extern printf\n");
    out.push_str("extern malloc\n");
    out.push_str("\n");

    for item in &program.program.items {
        emit_top_level(&mut out, item)?;
    }

    // Entry point
    out.push_str("main:\n");
    out.push_str("    sub rsp, 40\n"); // Shadow space + alignment
    out.push_str("    call _main\n");
    out.push_str("    add rsp, 40\n");
    out.push_str("    xor eax, eax\n");
    out.push_str("    ret\n");

    Ok(out)
}

fn emit_top_level(out: &mut String, item: &TopLevelItem) -> Result<()> {
    match item {
        TopLevelItem::Fn(f) => emit_fn(out, f)?,
        TopLevelItem::Struct(_) | TopLevelItem::Class(_) => {}
        TopLevelItem::Mod(m) => {
            for sub in &m.items {
                emit_top_level(out, sub)?;
            }
        }
        TopLevelItem::Import(_) => {}
    }
    Ok(())
}

fn emit_fn(out: &mut String, f: &FnDef) -> Result<()> {
    let name = mangle_fn(&f.name);
    out.push_str(&format!("{}:\n", name));
    out.push_str("    push rbp\n");
    out.push_str("    mov rbp, rsp\n");
    out.push_str("    sub rsp, 64\n"); // Local space

    for (i, _param) in f.params.iter().enumerate() {
        let reg = match i {
            0 => "rcx",
            1 => "rdx",
            2 => "r8",
            3 => "r9",
            _ => break,
        };
        out.push_str(&format!("    mov [rbp-{}], {}\n", 8 * (i + 1), reg));
    }

    for stmt in &f.body {
        emit_stmt(out, stmt)?;
    }

    out.push_str("    mov rsp, rbp\n");
    out.push_str("    pop rbp\n");
    out.push_str("    ret\n\n");
    Ok(())
}

fn emit_stmt(out: &mut String, stmt: &Stmt) -> Result<()> {
    match stmt {
        Stmt::VarDecl { name, init, .. } => {
            let _ = name;
            emit_expr(out, init)?;
            out.push_str("    push rax\n");
        }
        Stmt::Assign { target, value } => {
            emit_expr(out, value)?;
            if let AssignTarget::Ident(name) = target {
                out.push_str(&format!("    mov [rbp-{}], rax\n", 8)); // Simplified - need proper slot
                let _ = name;
            }
        }
        Stmt::If { cond, then_body, else_body } => {
            let then_label = new_label();
            let end_label = new_label();
            emit_expr(out, cond)?;
            out.push_str("    test rax, rax\n");
            out.push_str(&format!("    jnz {}\n", then_label));
            if let Some(else_b) = else_body {
                for s in else_b {
                    emit_stmt(out, s)?;
                }
            }
            out.push_str(&format!("    jmp {}\n", end_label));
            out.push_str(&format!("{}:\n", then_label));
            for s in then_body {
                emit_stmt(out, s)?;
            }
            out.push_str(&format!("{}:\n", end_label));
        }
        Stmt::For { init, cond, step, body } => {
            let loop_label = new_label();
            let end_label = new_label();
            if let Some(i) = init {
                emit_stmt(out, i)?;
            }
            out.push_str(&format!("{}:\n", loop_label));
            if let Some(c) = cond {
                emit_expr(out, c)?;
                out.push_str("    test rax, rax\n");
                out.push_str(&format!("    jz {}\n", end_label));
            }
            for s in body {
                emit_stmt(out, s)?;
            }
            if let Some(s) = step {
                emit_stmt(out, s)?;
            }
            out.push_str(&format!("    jmp {}\n", loop_label));
            out.push_str(&format!("{}:\n", end_label));
        }
        Stmt::While { cond, body } => {
            let loop_label = new_label();
            let end_label = new_label();
            out.push_str(&format!("{}:\n", loop_label));
            emit_expr(out, cond)?;
            out.push_str("    test rax, rax\n");
            out.push_str(&format!("    jz {}\n", end_label));
            for s in body {
                emit_stmt(out, s)?;
            }
            out.push_str(&format!("    jmp {}\n", loop_label));
            out.push_str(&format!("{}:\n", end_label));
        }
        Stmt::ExprStmt(e) => {
            emit_expr(out, e)?;
        }
        Stmt::Return(expr) => {
            if let Some(e) = expr {
                emit_expr(out, e)?;
            }
            out.push_str("    mov rsp, rbp\n");
            out.push_str("    pop rbp\n");
            out.push_str("    ret\n");
        }
        Stmt::TryCatch { try_body, .. } => {
            for s in try_body {
                emit_stmt(out, s)?;
            }
        }
    }
    Ok(())
}

fn emit_expr(out: &mut String, expr: &Expr) -> Result<()> {
    match expr {
        Expr::Literal(Literal::Int(n)) => {
            out.push_str(&format!("    mov rax, {}\n", n));
        }
        Expr::Literal(Literal::Float(_)) => {
            out.push_str("    mov rax, 0\n"); // TODO: float support
        }
        Expr::Literal(Literal::Bool(b)) => {
            out.push_str(&format!("    mov rax, {}\n", if *b { 1 } else { 0 }));
        }
        Expr::Literal(Literal::Str(_)) => {
            out.push_str("    mov rax, 0\n"); // TODO: string support
        }
        Expr::Ident(_) => {
            out.push_str("    mov rax, [rbp-8]\n"); // Simplified
        }
        Expr::Binary { op, left, right } => {
            emit_expr(out, right)?;
            out.push_str("    push rax\n");
            emit_expr(out, left)?;
            out.push_str("    pop rcx\n");
            match op {
                BinOp::Add => out.push_str("    add rax, rcx\n"),
                BinOp::Sub => out.push_str("    sub rax, rcx\n"),
                BinOp::Mul => out.push_str("    imul rax, rcx\n"),
                BinOp::Div => out.push_str("    cqo\n    idiv rcx\n"),
                BinOp::Mod => out.push_str("    cqo\n    idiv rcx\n    mov rax, rdx\n"),
                BinOp::Eq => {
                    out.push_str("    cmp rax, rcx\n");
                    out.push_str("    sete al\n");
                    out.push_str("    movzx rax, al\n");
                }
                BinOp::Ne => {
                    out.push_str("    cmp rax, rcx\n");
                    out.push_str("    setne al\n");
                    out.push_str("    movzx rax, al\n");
                }
                BinOp::Lt => {
                    out.push_str("    cmp rax, rcx\n");
                    out.push_str("    setl al\n");
                    out.push_str("    movzx rax, al\n");
                }
                BinOp::Le => {
                    out.push_str("    cmp rax, rcx\n");
                    out.push_str("    setle al\n");
                    out.push_str("    movzx rax, al\n");
                }
                BinOp::Gt => {
                    out.push_str("    cmp rax, rcx\n");
                    out.push_str("    setg al\n");
                    out.push_str("    movzx rax, al\n");
                }
                BinOp::Ge => {
                    out.push_str("    cmp rax, rcx\n");
                    out.push_str("    setge al\n");
                    out.push_str("    movzx rax, al\n");
                }
                BinOp::And => {
                    out.push_str("    and rax, rcx\n");
                }
                BinOp::Or => {
                    out.push_str("    or rax, rcx\n");
                }
            }
        }
        Expr::Unary { op, operand } => {
            emit_expr(out, operand)?;
            match op {
                UnOp::Neg => out.push_str("    neg rax\n"),
                UnOp::Not => {
                    out.push_str("    test rax, rax\n");
                    out.push_str("    sete al\n");
                    out.push_str("    movzx rax, al\n");
                }
            }
        }
        Expr::Call { callee, args } => {
            for (i, arg) in args.iter().enumerate().take(4) {
                emit_expr(out, arg)?;
                let reg = match i {
                    0 => "rcx",
                    1 => "rdx",
                    2 => "r8",
                    3 => "r9",
                    _ => break,
                };
                out.push_str(&format!("    mov {}, rax\n", reg));
            }
            if let Expr::Ident(name) = callee.as_ref() {
                let name = mangle_fn(name);
                out.push_str(&format!("    call {}\n", name));
            }
        }
        Expr::Index { base, index } => {
            emit_expr(out, index)?;
            out.push_str("    push rax\n");
            emit_expr(out, base)?;
            out.push_str("    pop rcx\n");
            out.push_str("    lea rax, [rax + rcx*8]\n");
            out.push_str("    mov rax, [rax]\n");
        }
        Expr::Field { base, field: _ } => {
            emit_expr(out, base)?;
            out.push_str("    mov rax, [rax]\n"); // Simplified
        }
        Expr::New { class, args } => {
            let _ = class;
            let _ = args;
            out.push_str("    mov ecx, 16\n");
            out.push_str("    call malloc\n");
        }
    }
    Ok(())
}

fn mangle_fn(name: &str) -> String {
    format!("_{}", name.replace('.', "_"))
}

static LABEL_COUNTER: AtomicU32 = AtomicU32::new(0);

fn new_label() -> String {
    let n = LABEL_COUNTER.fetch_add(1, Ordering::Relaxed);
    format!(".L{}", n)
}
