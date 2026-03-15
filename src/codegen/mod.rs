//! Code generator - emits x86/x64 assembly (NASM format)

use crate::ast::*;
use crate::error::Result;
use crate::semantic::AnnotatedProgram;
use std::collections::HashMap;
use std::sync::atomic::{AtomicU32, Ordering};

#[derive(Clone, Default)]
struct CodegenContext {
    var_slots: HashMap<String, i32>,
    next_slot: i32,
}

impl CodegenContext {
    fn add_param(&mut self, name: &str) {
        self.next_slot += 1;
        self.var_slots.insert(name.to_string(), self.next_slot * 8);
    }
    fn add_var(&mut self, name: &str) {
        self.next_slot += 1;
        self.var_slots.insert(name.to_string(), self.next_slot * 8);
    }
    fn get_slot(&self, name: &str) -> i32 {
        *self.var_slots.get(name).unwrap_or(&8)
    }
}

pub fn generate(program: &AnnotatedProgram) -> Result<String> {
    let mut out = String::new();

    // NASM Windows x64
    out.push_str("section .text\n");
    out.push_str("global main\n");
    out.push_str("extern printf\n");
    out.push_str("extern malloc\n");
    out.push_str("\n");

    let mut ctx = CodegenContext::default();
    for item in &program.program.items {
        emit_top_level(&mut out, item, &mut ctx)?;
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

fn emit_top_level(out: &mut String, item: &TopLevelItem, ctx: &mut CodegenContext) -> Result<()> {
    match item {
        TopLevelItem::Fn(f) => emit_fn(out, f, ctx)?,
        TopLevelItem::Struct(_) => {}
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

fn emit_class(out: &mut String, c: &ClassDef, ctx: &mut CodegenContext) -> Result<()> {
    if let Some(init) = &c.init {
        let mut init_ctx = CodegenContext::default();
        init_ctx.add_param("this");
        for p in &init.params {
            init_ctx.add_param(&p.name);
        }
        let name = mangle_init(&c.name);
        out.push_str(&format!("{}:\n", name));
        out.push_str("    push rbp\n");
        out.push_str("    mov rbp, rsp\n");
        out.push_str("    sub rsp, 64\n");
        for (i, _p) in init.params.iter().enumerate() {
            let reg = match i {
                0 => "rdx",
                1 => "r8",
                2 => "r9",
                _ => break,
            };
            out.push_str(&format!("    mov [rbp-{}], {}\n", 8 * (i + 2), reg));
        }
        out.push_str("    mov [rbp-8], rcx\n");
        for stmt in &init.body {
            emit_stmt(out, stmt, &mut init_ctx)?;
        }
        out.push_str("    mov rsp, rbp\n");
        out.push_str("    pop rbp\n");
        out.push_str("    ret\n\n");
    }
    for m in &c.methods {
        emit_method(out, &c.name, m, ctx)?;
    }
    Ok(())
}

fn emit_method(out: &mut String, class_name: &str, m: &MethodDef, _ctx: &mut CodegenContext) -> Result<()> {
    let mut method_ctx = CodegenContext::default();
    method_ctx.add_param("this");
    for p in &m.params {
        method_ctx.add_param(&p.name);
    }
    let name = mangle_method(class_name, &m.name);
    out.push_str(&format!("{}:\n", name));
    out.push_str("    push rbp\n");
    out.push_str("    mov rbp, rsp\n");
    out.push_str("    sub rsp, 64\n");
    out.push_str("    mov [rbp-8], rcx\n");
    for (i, _p) in m.params.iter().enumerate() {
        let reg = match i {
            0 => "rdx",
            1 => "r8",
            2 => "r9",
            _ => break,
        };
        out.push_str(&format!("    mov [rbp-{}], {}\n", 8 * (i + 2), reg));
    }
    for stmt in &m.body {
        emit_stmt(out, stmt, &mut method_ctx)?;
    }
    out.push_str("    mov rsp, rbp\n");
    out.push_str("    pop rbp\n");
    out.push_str("    ret\n\n");
    Ok(())
}

fn mangle_init(class: &str) -> String {
    format!("_{}_init", class)
}

fn mangle_method(class: &str, method: &str) -> String {
    format!("_{}_{}", class, method)
}

fn field_offset(field: &str) -> usize {
    match field {
        "x" => 8,
        "y" => 16,
        "color" => 24,
        _ => 8,
    }
}

fn emit_fn(out: &mut String, f: &FnDef, _ctx: &mut CodegenContext) -> Result<()> {
    let mut fn_ctx = CodegenContext::default();
    for p in &f.params {
        fn_ctx.add_param(&p.name);
    }
    let name = mangle_fn(&f.name);
    out.push_str(&format!("{}:\n", name));
    out.push_str("    push rbp\n");
    out.push_str("    mov rbp, rsp\n");
    out.push_str("    sub rsp, 64\n");

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
        emit_stmt(out, stmt, &mut fn_ctx)?;
    }

    out.push_str("    mov rsp, rbp\n");
    out.push_str("    pop rbp\n");
    out.push_str("    ret\n\n");
    Ok(())
}

fn emit_stmt(out: &mut String, stmt: &Stmt, ctx: &mut CodegenContext) -> Result<()> {
    match stmt {
        Stmt::VarDecl { name, init, .. } => {
            ctx.add_var(name);
            emit_expr(out, init, ctx)?;
            let slot = ctx.get_slot(name);
            out.push_str(&format!("    mov [rbp-{}], rax\n", slot));
        }
        Stmt::Assign { target, value } => {
            emit_expr(out, value, ctx)?;
            match target {
                AssignTarget::Ident(name) => {
                    let slot = ctx.get_slot(name);
                    out.push_str(&format!("    mov [rbp-{}], rax\n", slot));
                }
                AssignTarget::Field { base, field } => {
                    emit_expr(out, base, ctx)?;
                    out.push_str("    push rax\n");
                    emit_expr(out, value, ctx)?;
                    out.push_str("    pop rcx\n");
                    let offset = field_offset(field);
                    out.push_str(&format!("    mov [rcx+{}], rax\n", offset));
                }
                AssignTarget::Index { base, index } => {
                    emit_expr(out, index, ctx)?;
                    out.push_str("    push rax\n");
                    emit_expr(out, base, ctx)?;
                    out.push_str("    pop rcx\n");
                    out.push_str("    lea rcx, [rax + rcx*8]\n");
                    emit_expr(out, value, ctx)?;
                    out.push_str("    mov [rcx], rax\n");
                }
            }
        }
        Stmt::If { cond, then_body, else_body } => {
            let then_label = new_label();
            let end_label = new_label();
            emit_expr(out, cond, ctx)?;
            out.push_str("    test rax, rax\n");
            out.push_str(&format!("    jnz {}\n", then_label));
            if let Some(else_b) = else_body {
                for s in else_b {
                    emit_stmt(out, s, ctx)?;
                }
            }
            out.push_str(&format!("    jmp {}\n", end_label));
            out.push_str(&format!("{}:\n", then_label));
            for s in then_body {
                emit_stmt(out, s, ctx)?;
            }
            out.push_str(&format!("{}:\n", end_label));
        }
        Stmt::For { init, cond, step, body } => {
            let loop_label = new_label();
            let end_label = new_label();
            if let Some(i) = init {
                emit_stmt(out, i, ctx)?;
            }
            out.push_str(&format!("{}:\n", loop_label));
            if let Some(c) = cond {
                emit_expr(out, c, ctx)?;
                out.push_str("    test rax, rax\n");
                out.push_str(&format!("    jz {}\n", end_label));
            }
            for s in body {
                emit_stmt(out, s, ctx)?;
            }
            if let Some(s) = step {
                emit_stmt(out, s, ctx)?;
            }
            out.push_str(&format!("    jmp {}\n", loop_label));
            out.push_str(&format!("{}:\n", end_label));
        }
        Stmt::While { cond, body } => {
            let loop_label = new_label();
            let end_label = new_label();
            out.push_str(&format!("{}:\n", loop_label));
            emit_expr(out, cond, ctx)?;
            out.push_str("    test rax, rax\n");
            out.push_str(&format!("    jz {}\n", end_label));
            for s in body {
                emit_stmt(out, s, ctx)?;
            }
            out.push_str(&format!("    jmp {}\n", loop_label));
            out.push_str(&format!("{}:\n", end_label));
        }
        Stmt::ExprStmt(e) => {
            emit_expr(out, e, ctx)?;
        }
        Stmt::Return(expr) => {
            if let Some(e) = expr {
                emit_expr(out, e, ctx)?;
            }
            out.push_str("    mov rsp, rbp\n");
            out.push_str("    pop rbp\n");
            out.push_str("    ret\n");
        }
        Stmt::TryCatch { try_body, .. } => {
            for s in try_body {
                emit_stmt(out, s, ctx)?;
            }
        }
    }
    Ok(())
}

fn emit_expr(out: &mut String, expr: &Expr, ctx: &CodegenContext) -> Result<()> {
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
        Expr::Ident(name) => {
            let slot = ctx.get_slot(name);
            out.push_str(&format!("    mov rax, [rbp-{}]\n", slot));
        }
        Expr::Binary { op, left, right } => {
            emit_expr(out, right, ctx)?;
            out.push_str("    push rax\n");
            emit_expr(out, left, ctx)?;
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
            emit_expr(out, operand, ctx)?;
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
                emit_expr(out, arg, ctx)?;
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
            emit_expr(out, index, ctx)?;
            out.push_str("    push rax\n");
            emit_expr(out, base, ctx)?;
            out.push_str("    pop rcx\n");
            out.push_str("    lea rax, [rax + rcx*8]\n");
            out.push_str("    mov rax, [rax]\n");
        }
        Expr::Field { base, field } => {
            emit_expr(out, base, ctx)?;
            let offset = field_offset(field);
            out.push_str(&format!("    mov rax, [rax+{}]\n", offset));
        }
        Expr::New { class, args } => {
            let size = 8 + 16; // vtable ptr + 2 fields for Point
            out.push_str(&format!("    mov ecx, {}\n", size));
            out.push_str("    call malloc\n");
            out.push_str("    push rax\n");
            for (i, arg) in args.iter().enumerate() {
                emit_expr(out, arg, ctx)?;
                let reg = match i {
                    0 => "rdx",
                    1 => "r8",
                    2 => "r9",
                    _ => break,
                };
                out.push_str(&format!("    mov {}, rax\n", reg));
            }
            out.push_str("    pop rcx\n");
            out.push_str(&format!("    call _{}_init\n", class));
            out.push_str("    mov rax, rcx\n");
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
