//! LLVM backend - compiles QuinusLang to machine code via LLVM IR
//! Requires LLVM 17 dev libraries installed

use crate::ast::*;
use crate::error::Result;
use crate::semantic::AnnotatedProgram;
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::targets::{
    CodeModel, FileType, InitializationConfig, RelocMode, Target, TargetMachine,
};
use inkwell::types::{BasicMetadataTypeEnum, BasicType};
use inkwell::values::BasicValueEnum;
use inkwell::OptimizationLevel;
use std::collections::HashMap;
use std::path::Path;
use std::sync::atomic::{AtomicU32, Ordering};

/// Compile the program to an object file at the given path.
pub fn compile_to_object(program: &AnnotatedProgram, obj_path: &Path) -> Result<()> {
    Target::initialize_native(&InitializationConfig::default())?;

    let context = Context::create();
    let module = context.create_module("quinus");
    let builder = context.create_builder();

    declare_builtins(&context, &module);

    let mut ctx = LlvmCtx {
        context: &context,
        module,
        builder,
        vars: HashMap::new(),
        var_types: HashMap::new(),
        symbol_table: Some(std::sync::Arc::new(program.symbol_table.clone())),
    };
    for item in &program.program.items {
        if let TopLevelItem::Fn(f) = item {
            emit_fn(&mut ctx, f)?;
        }
    }

    // Emit main wrapper: i32 main() { call void @_ql_main(); ret i32 0 }
    let i32_type = context.i32_type();
    let main_type = i32_type.fn_type(&[], false);
    let main_fn = ctx.module.add_function("main", main_type, None);
    let entry = context.append_basic_block(main_fn, "entry");
    ctx.builder.position_at_end(entry);

    let ql_main = ctx
        .module
        .get_function("_ql_main")
        .ok_or_else(|| crate::error::semantic_err("main() not found"))?;
    ctx.builder.build_call(ql_main, &[], "call_main")?;
    ctx.builder
        .build_return(Some(&i32_type.const_int(0, false)))?;

    // Verify and write object file
    ctx.module.verify()?;
    let triple = TargetMachine::get_default_triple();
    let target = Target::from_triple(&triple)?;
    let target_machine = target
        .create_target_machine(
            &triple,
            "generic",
            "",
            OptimizationLevel::Default,
            RelocMode::Default,
            CodeModel::Default,
        )
        .ok_or_else(|| crate::error::semantic_err("Failed to create target machine"))?;

    target_machine.write_to_file(&ctx.module, FileType::Object, obj_path)?;
    Ok(())
}

/// Compile the program to LLVM IR and write to the given path.
pub fn compile_to_ir(program: &AnnotatedProgram, ir_path: &Path) -> Result<()> {
    Target::initialize_native(&InitializationConfig::default())?;

    let context = Context::create();
    let module = context.create_module("quinus");
    let builder = context.create_builder();

    declare_builtins(&context, &module);

    let mut ctx = LlvmCtx {
        context: &context,
        module,
        builder,
        vars: HashMap::new(),
        var_types: HashMap::new(),
        symbol_table: Some(std::sync::Arc::new(program.symbol_table.clone())),
    };
    for item in &program.program.items {
        if let TopLevelItem::Fn(f) = item {
            emit_fn(&mut ctx, f)?;
        }
    }

    let i32_type = context.i32_type();
    let main_type = i32_type.fn_type(&[], false);
    let main_fn = ctx.module.add_function("main", main_type, None);
    let entry = context.append_basic_block(main_fn, "entry");
    ctx.builder.position_at_end(entry);

    let ql_main = ctx
        .module
        .get_function("_ql_main")
        .ok_or_else(|| crate::error::semantic_err("main() not found"))?;
    ctx.builder.build_call(ql_main, &[], "call_main")?;
    ctx.builder
        .build_return(Some(&i32_type.const_int(0, false)))?;

    ctx.module.verify()?;
    ctx.module
        .print_to_file(ir_path)
        .map_err(|e| crate::error::semantic_err(format!("Failed to write IR: {}", e)))?;
    Ok(())
}

/// Compile the program to LLVM IR and return as string (for testing).
pub fn compile_to_ir_string(program: &AnnotatedProgram) -> Result<String> {
    Target::initialize_native(&InitializationConfig::default())?;

    let context = Context::create();
    let module = context.create_module("quinus");
    let builder = context.create_builder();

    declare_builtins(&context, &module);

    let mut ctx = LlvmCtx {
        context: &context,
        module,
        builder,
        vars: HashMap::new(),
        var_types: HashMap::new(),
        symbol_table: Some(std::sync::Arc::new(program.symbol_table.clone())),
    };
    for item in &program.program.items {
        if let TopLevelItem::Fn(f) = item {
            emit_fn(&mut ctx, f)?;
        }
    }

    let i32_type = context.i32_type();
    let main_type = i32_type.fn_type(&[], false);
    let main_fn = ctx.module.add_function("main", main_type, None);
    let entry = context.append_basic_block(main_fn, "entry");
    ctx.builder.position_at_end(entry);

    let ql_main = ctx
        .module
        .get_function("_ql_main")
        .ok_or_else(|| crate::error::semantic_err("main() not found"))?;
    ctx.builder.build_call(ql_main, &[], "call_main")?;
    ctx.builder
        .build_return(Some(&i32_type.const_int(0, false)))?;

    ctx.module.verify()?;
    Ok(ctx.module.print_to_string().to_string())
}

fn declare_builtins<'ctx>(context: &'ctx Context, module: &Module<'ctx>) {
    let i8_ptr = context.ptr_type(inkwell::AddressSpace::default());
    let i32 = context.i32_type();
    let i64 = context.i64_type();

    // printf(i8* fmt, ...) -> i32
    let printf_ty = i32.fn_type(&[i8_ptr.into()], true);
    module.add_function("printf", printf_ty, None);

    // malloc(size_t) -> i8*
    let malloc_ty = i8_ptr.fn_type(&[i64.into()], false);
    module.add_function("malloc", malloc_ty, None);

    // strlen(i8*) -> i64
    let strlen_ty = i64.fn_type(&[i8_ptr.into()], false);
    module.add_function("strlen", strlen_ty, None);

    // fprintf(i8*, i8*, ...) -> i32
    let fprintf_ty = i32.fn_type(&[i8_ptr.into(), i8_ptr.into()], true);
    module.add_function("fprintf", fprintf_ty, None);

    // exit(i32) -> noreturn
    let exit_ty = context.void_type().fn_type(&[i32.into()], false);
    module.add_function("exit", exit_ty, None);
}

static STRING_COUNTER: AtomicU32 = AtomicU32::new(0);

struct LlvmCtx<'ctx> {
    context: &'ctx Context,
    module: Module<'ctx>,
    builder: Builder<'ctx>,
    vars: HashMap<String, BasicValueEnum<'ctx>>,
    var_types: HashMap<String, Type>,
    symbol_table: Option<std::sync::Arc<crate::semantic::SymbolTable>>,
}

fn type_to_llvm<'ctx>(ctx: &'ctx Context, ty: &Type) -> inkwell::types::BasicTypeEnum<'ctx> {
    match ty {
        Type::Int | Type::I64 => ctx.i64_type().into(),
        Type::I32 => ctx.i32_type().into(),
        Type::Float | Type::F64 => ctx.f64_type().into(),
        Type::F32 => ctx.f32_type().into(),
        Type::Bool => ctx.bool_type().into(),
        Type::Str => ctx.ptr_type(inkwell::AddressSpace::default()).into(),
        Type::U8 => ctx.i8_type().into(),
        Type::U16 => ctx.i16_type().into(),
        Type::U32 => ctx.i32_type().into(),
        Type::U64 => ctx.i64_type().into(),
        Type::Usize => ctx.i64_type().into(),  // assume 64-bit
        Type::Void => ctx.i8_type().into(),    // placeholder
        Type::Ptr(_) => ctx.i64_type().into(), // pointers as i64 for now
        _ => ctx.i64_type().into(),
    }
}

fn emit_fn<'ctx>(ctx: &mut LlvmCtx<'ctx>, f: &FnDef) -> Result<()> {
    let name = if f.name == "main" {
        "_ql_main".to_string()
    } else {
        f.name.clone()
    };
    let param_tys: Vec<BasicMetadataTypeEnum> = f
        .params
        .iter()
        .map(|p| type_to_llvm(ctx.context, &p.ty).into())
        .collect();
    let fn_type = match f.return_type.as_ref() {
        Some(Type::Void) | None => ctx.context.void_type().fn_type(&param_tys, false),
        Some(t) => type_to_llvm(ctx.context, t).fn_type(&param_tys, false),
    };
    let function = ctx.module.add_function(&name, fn_type, None);

    let entry = ctx.context.append_basic_block(function, "entry");
    ctx.builder.position_at_end(entry);

    for (i, param) in f.params.iter().enumerate() {
        let val = function.get_nth_param(i as u32).unwrap();
        ctx.vars.insert(param.name.clone(), val.into());
        ctx.var_types.insert(param.name.clone(), param.ty.clone());
    }

    for stmt in &f.body {
        emit_stmt(ctx, stmt)?;
    }

    // If no explicit return and return type is void
    if f.return_type
        .as_ref()
        .map_or(true, |t| matches!(t, Type::Void))
    {
        if ctx
            .builder
            .get_insert_block()
            .map(|b| b.get_terminator().is_none())
            .unwrap_or(true)
        {
            ctx.builder.build_return(None)?;
        }
    }

    Ok(())
}

fn emit_stmt<'ctx>(ctx: &mut LlvmCtx<'ctx>, stmt: &Stmt) -> Result<()> {
    match stmt {
        Stmt::VarDecl {
            name,
            ty,
            init,
            mutable: _,
        } => {
            let ty_resolved = ty
                .clone()
                .unwrap_or_else(|| expr_type(init, ctx).unwrap_or(Type::Int));
            let llvm_ty = type_to_llvm(ctx.context, &ty_resolved);
            let val = emit_expr(ctx, init)?;
            let alloca = ctx.builder.build_alloca(llvm_ty, name)?;
            ctx.builder.build_store(alloca, val)?;
            ctx.vars.insert(name.clone(), alloca.into());
            ctx.var_types.insert(name.clone(), ty_resolved);
        }
        Stmt::Assign { target, value } => {
            let val = emit_expr(ctx, value)?;
            if let AssignTarget::Ident(name) = target {
                if let Some(ptr) = ctx.vars.get(name) {
                    if let BasicValueEnum::PointerValue(p) = ptr {
                        ctx.builder.build_store(*p, val)?;
                    }
                }
            }
        }
        Stmt::Return(expr) => {
            if let Some(e) = expr {
                let val = emit_expr(ctx, e)?;
                ctx.builder.build_return(Some(&val))?;
            } else {
                ctx.builder.build_return(None)?;
            }
        }
        Stmt::ExprStmt(e) => {
            let _ = emit_expr(ctx, e)?;
        }
        Stmt::If {
            cond,
            then_body,
            else_body,
        } => {
            let cond_val = emit_expr(ctx, cond)?;
            let cond_bool = match cond_val {
                BasicValueEnum::IntValue(i) => ctx.builder.build_int_compare(
                    inkwell::IntPredicate::NE,
                    i,
                    i.get_type().const_zero(),
                    "cond",
                )?,
                _ => return Err(crate::error::semantic_err("Condition must be integer")),
            };

            let then_bb = ctx.context.append_basic_block(
                ctx.builder
                    .get_insert_block()
                    .unwrap()
                    .get_parent()
                    .unwrap(),
                "then",
            );
            let else_bb = ctx.context.append_basic_block(
                ctx.builder
                    .get_insert_block()
                    .unwrap()
                    .get_parent()
                    .unwrap(),
                "else",
            );
            let merge_bb = ctx.context.append_basic_block(
                ctx.builder
                    .get_insert_block()
                    .unwrap()
                    .get_parent()
                    .unwrap(),
                "merge",
            );

            if else_body.is_some() {
                ctx.builder
                    .build_conditional_branch(cond_bool, then_bb, else_bb)?;
            } else {
                ctx.builder
                    .build_conditional_branch(cond_bool, then_bb, merge_bb)?;
            }

            ctx.builder.position_at_end(then_bb);
            for s in then_body {
                emit_stmt(ctx, s)?;
            }
            if ctx
                .builder
                .get_insert_block()
                .unwrap()
                .get_terminator()
                .is_none()
            {
                ctx.builder.build_unconditional_branch(merge_bb)?;
            }

            if let Some(else_b) = else_body {
                ctx.builder.position_at_end(else_bb);
                for s in else_b {
                    emit_stmt(ctx, s)?;
                }
                if ctx
                    .builder
                    .get_insert_block()
                    .unwrap()
                    .get_terminator()
                    .is_none()
                {
                    ctx.builder.build_unconditional_branch(merge_bb)?;
                }
            } else {
                ctx.builder.position_at_end(else_bb);
                ctx.builder.build_unconditional_branch(merge_bb)?;
            }

            ctx.builder.position_at_end(merge_bb);
        }
        Stmt::For {
            init,
            cond,
            step,
            body,
        } => {
            if let Some(ref i) = init {
                emit_stmt(ctx, i)?;
            }
            let func = ctx
                .builder
                .get_insert_block()
                .unwrap()
                .get_parent()
                .unwrap();
            let loop_bb = ctx.context.append_basic_block(func, "for_loop");
            let body_bb = ctx.context.append_basic_block(func, "for_body");
            let step_bb = ctx.context.append_basic_block(func, "for_step");
            let end_bb = ctx.context.append_basic_block(func, "for_end");

            ctx.builder.build_unconditional_branch(loop_bb)?;

            ctx.builder.position_at_end(loop_bb);
            if let Some(ref c) = cond {
                let cond_val = emit_expr(ctx, c)?;
                let cond_bool = match cond_val {
                    BasicValueEnum::IntValue(i) => ctx.builder.build_int_compare(
                        inkwell::IntPredicate::NE,
                        i,
                        i.get_type().const_zero(),
                        "for_cond",
                    )?,
                    _ => return Err(crate::error::semantic_err("For condition must be integer")),
                };
                ctx.builder
                    .build_conditional_branch(cond_bool, body_bb, end_bb)?;
            } else {
                ctx.builder.build_unconditional_branch(body_bb)?;
            }

            ctx.builder.position_at_end(body_bb);
            for s in body {
                emit_stmt(ctx, s)?;
            }
            if ctx
                .builder
                .get_insert_block()
                .unwrap()
                .get_terminator()
                .is_none()
            {
                ctx.builder.build_unconditional_branch(step_bb)?;
            }

            ctx.builder.position_at_end(step_bb);
            if let Some(ref s) = step {
                emit_stmt(ctx, s)?;
            }
            if ctx
                .builder
                .get_insert_block()
                .unwrap()
                .get_terminator()
                .is_none()
            {
                ctx.builder.build_unconditional_branch(loop_bb)?;
            }

            ctx.builder.position_at_end(end_bb);
        }
        Stmt::While { cond, body } => {
            let func = ctx
                .builder
                .get_insert_block()
                .unwrap()
                .get_parent()
                .unwrap();
            let loop_bb = ctx.context.append_basic_block(func, "while_loop");
            let body_bb = ctx.context.append_basic_block(func, "while_body");
            let end_bb = ctx.context.append_basic_block(func, "while_end");

            ctx.builder.build_unconditional_branch(loop_bb)?;

            ctx.builder.position_at_end(loop_bb);
            let cond_val = emit_expr(ctx, cond)?;
            let cond_bool = match cond_val {
                BasicValueEnum::IntValue(i) => ctx.builder.build_int_compare(
                    inkwell::IntPredicate::NE,
                    i,
                    i.get_type().const_zero(),
                    "while_cond",
                )?,
                _ => {
                    return Err(crate::error::semantic_err(
                        "While condition must be integer",
                    ))
                }
            };
            ctx.builder
                .build_conditional_branch(cond_bool, body_bb, end_bb)?;

            ctx.builder.position_at_end(body_bb);
            for s in body {
                emit_stmt(ctx, s)?;
            }
            if ctx
                .builder
                .get_insert_block()
                .unwrap()
                .get_terminator()
                .is_none()
            {
                ctx.builder.build_unconditional_branch(loop_bb)?;
            }

            ctx.builder.position_at_end(end_bb);
        }
        Stmt::Defer { body } => {
            for s in body {
                emit_stmt(ctx, s)?;
            }
        }
        Stmt::With { expr, body, .. } => {
            let _ = emit_expr(ctx, expr)?;
            for s in body {
                emit_stmt(ctx, s)?;
            }
        }
        Stmt::VarDeclTuple { .. }
        | Stmt::Foreach { .. }
        | Stmt::Break
        | Stmt::Continue
        | Stmt::Hazard { .. }
        | Stmt::InlineAsm { .. }
        | Stmt::InlineC { .. }
        | Stmt::TryCatch { .. }
        | Stmt::Choose { .. } => {
            return Err(crate::error::semantic_err(format!(
                "LLVM backend: unsupported statement {:?}",
                stmt
            )));
        }
    }
    Ok(())
}

fn emit_builtin_call<'ctx>(
    ctx: &mut LlvmCtx<'ctx>,
    name: &str,
    args: &[Expr],
) -> Result<Option<BasicValueEnum<'ctx>>> {
    let printf_fn = ctx.module.get_function("printf").unwrap();
    let i8_ptr = ctx.context.ptr_type(inkwell::AddressSpace::default());

    match name {
        "print" | "write" => {
            if args.is_empty() {
                return Ok(Some(ctx.context.i64_type().const_zero().into()));
            }
            let fmt = match &args[0] {
                Expr::Literal(Literal::Str(_)) => "%s\0",
                Expr::Literal(Literal::Int(_)) => "%ld\0",
                Expr::Literal(Literal::Float(_)) => "%f\0",
                Expr::Literal(Literal::Bool(_)) => "%d\0",
                _ => "%s\0",
            };
            let fmt_global = ctx.module.add_global(
                i8_ptr,
                None,
                &format!("fmt_{}", STRING_COUNTER.fetch_add(1, Ordering::Relaxed)),
            );
            fmt_global.set_constant(true);
            fmt_global.set_initializer(&ctx.context.const_string(fmt.as_bytes(), true));
            let fmt_ptr = fmt_global.as_pointer_value();

            let mut call_args: Vec<inkwell::values::BasicMetadataValueEnum> = vec![fmt_ptr.into()];
            for a in args {
                let v = emit_expr(ctx, a)?;
                call_args.push(v.into());
            }
            ctx.builder.build_call(printf_fn, &call_args, "print")?;
            Ok(Some(ctx.context.i64_type().const_zero().into()))
        }
        "writeln" => {
            let (fmt, num_args) = if args.is_empty() {
                ("\n\0", 0)
            } else {
                let f = match &args[0] {
                    Expr::Literal(Literal::Str(_)) => "%s\n\0",
                    Expr::Literal(Literal::Int(_)) => "%ld\n\0",
                    Expr::Literal(Literal::Float(_)) => "%f\n\0",
                    Expr::Literal(Literal::Bool(_)) => "%d\n\0",
                    _ => "%s\n\0",
                };
                (f, 1)
            };
            let n = STRING_COUNTER.fetch_add(1, Ordering::Relaxed);
            let fmt_global = ctx.module.add_global(i8_ptr, None, &format!("fmt_{}", n));
            fmt_global.set_constant(true);
            fmt_global.set_initializer(&ctx.context.const_string(fmt.as_bytes(), true));
            let fmt_ptr = fmt_global.as_pointer_value();

            let mut call_args: Vec<inkwell::values::BasicMetadataValueEnum> = vec![fmt_ptr.into()];
            for a in args.iter().take(num_args) {
                let v = emit_expr(ctx, a)?;
                call_args.push(v.into());
            }
            ctx.builder.build_call(printf_fn, &call_args, "writeln")?;
            Ok(Some(ctx.context.i64_type().const_zero().into()))
        }
        _ => Ok(None),
    }
}

fn emit_expr<'ctx>(ctx: &mut LlvmCtx<'ctx>, expr: &Expr) -> Result<BasicValueEnum<'ctx>> {
    match expr {
        Expr::Literal(Literal::Int(n)) => {
            Ok(ctx.context.i64_type().const_int(*n as u64, *n < 0).into())
        }
        Expr::Literal(Literal::Float(f)) => Ok(ctx.context.f64_type().const_float(*f).into()),
        Expr::Literal(Literal::Bool(b)) => {
            Ok(ctx.context.bool_type().const_int(*b as u64, false).into())
        }
        Expr::Literal(Literal::Str(s)) => {
            let s_nul = format!("{}\0", s);
            let n = STRING_COUNTER.fetch_add(1, Ordering::Relaxed);
            let name = format!("str_{}", n);
            let global = ctx.module.add_global(
                ctx.context.ptr_type(inkwell::AddressSpace::default()),
                None,
                &name,
            );
            global.set_constant(true);
            global.set_unnamed_address(inkwell::values::UnnamedAddress::Global);
            global.set_initializer(&ctx.context.const_string(s_nul.as_bytes(), true));
            let ptr = ctx.builder.build_pointer_cast(
                global.as_pointer_value(),
                ctx.context.ptr_type(inkwell::AddressSpace::default()),
                "str_ptr",
            )?;
            Ok(ptr.into())
        }
        Expr::Ident(name) => {
            if let Some(ptr) = ctx.vars.get(name) {
                if let BasicValueEnum::PointerValue(p) = ptr {
                    let loaded = ctx.builder.build_load(
                        type_to_llvm(ctx.context, ctx.var_types.get(name).unwrap_or(&Type::Int)),
                        *p,
                        name,
                    )?;
                    return Ok(loaded);
                }
            }
            Err(crate::error::semantic_err(format!(
                "Undefined variable: {}",
                name
            )))
        }
        Expr::Binary { op, left, right } => {
            let l = emit_expr(ctx, left)?;
            let r = emit_expr(ctx, right)?;
            match op {
                BinOp::Add => {
                    if let (BasicValueEnum::IntValue(a), BasicValueEnum::IntValue(b)) = (l, r) {
                        Ok(ctx.builder.build_int_add(a, b, "add")?.into())
                    } else if let (BasicValueEnum::FloatValue(a), BasicValueEnum::FloatValue(b)) =
                        (l, r)
                    {
                        Ok(ctx.builder.build_float_add(a, b, "add")?.into())
                    } else {
                        Err(crate::error::semantic_err("Type mismatch in add"))
                    }
                }
                BinOp::Sub => {
                    if let (BasicValueEnum::IntValue(a), BasicValueEnum::IntValue(b)) = (l, r) {
                        Ok(ctx.builder.build_int_sub(a, b, "sub")?.into())
                    } else if let (BasicValueEnum::FloatValue(a), BasicValueEnum::FloatValue(b)) =
                        (l, r)
                    {
                        Ok(ctx.builder.build_float_sub(a, b, "sub")?.into())
                    } else {
                        Err(crate::error::semantic_err("Type mismatch in sub"))
                    }
                }
                BinOp::Mul => {
                    if let (BasicValueEnum::IntValue(a), BasicValueEnum::IntValue(b)) = (l, r) {
                        Ok(ctx.builder.build_int_mul(a, b, "mul")?.into())
                    } else if let (BasicValueEnum::FloatValue(a), BasicValueEnum::FloatValue(b)) =
                        (l, r)
                    {
                        Ok(ctx.builder.build_float_mul(a, b, "mul")?.into())
                    } else {
                        Err(crate::error::semantic_err("Type mismatch in mul"))
                    }
                }
                BinOp::Div => {
                    if let (BasicValueEnum::IntValue(a), BasicValueEnum::IntValue(b)) = (l, r) {
                        Ok(ctx.builder.build_int_signed_div(a, b, "div")?.into())
                    } else if let (BasicValueEnum::FloatValue(a), BasicValueEnum::FloatValue(b)) =
                        (l, r)
                    {
                        Ok(ctx.builder.build_float_div(a, b, "div")?.into())
                    } else {
                        Err(crate::error::semantic_err("Type mismatch in div"))
                    }
                }
                BinOp::Eq => {
                    if let (BasicValueEnum::IntValue(a), BasicValueEnum::IntValue(b)) = (l, r) {
                        Ok(ctx
                            .builder
                            .build_int_compare(inkwell::IntPredicate::EQ, a, b, "eq")?
                            .into())
                    } else {
                        Err(crate::error::semantic_err("Type mismatch in =="))
                    }
                }
                BinOp::Ne => {
                    if let (BasicValueEnum::IntValue(a), BasicValueEnum::IntValue(b)) = (l, r) {
                        Ok(ctx
                            .builder
                            .build_int_compare(inkwell::IntPredicate::NE, a, b, "ne")?
                            .into())
                    } else {
                        Err(crate::error::semantic_err("Type mismatch in !="))
                    }
                }
                BinOp::Lt => {
                    if let (BasicValueEnum::IntValue(a), BasicValueEnum::IntValue(b)) = (l, r) {
                        Ok(ctx
                            .builder
                            .build_int_compare(inkwell::IntPredicate::SLT, a, b, "lt")?
                            .into())
                    } else {
                        Err(crate::error::semantic_err("Type mismatch in <"))
                    }
                }
                BinOp::Le => {
                    if let (BasicValueEnum::IntValue(a), BasicValueEnum::IntValue(b)) = (l, r) {
                        Ok(ctx
                            .builder
                            .build_int_compare(inkwell::IntPredicate::SLE, a, b, "le")?
                            .into())
                    } else {
                        Err(crate::error::semantic_err("Type mismatch in <="))
                    }
                }
                BinOp::Gt => {
                    if let (BasicValueEnum::IntValue(a), BasicValueEnum::IntValue(b)) = (l, r) {
                        Ok(ctx
                            .builder
                            .build_int_compare(inkwell::IntPredicate::SGT, a, b, "gt")?
                            .into())
                    } else {
                        Err(crate::error::semantic_err("Type mismatch in >"))
                    }
                }
                BinOp::Ge => {
                    if let (BasicValueEnum::IntValue(a), BasicValueEnum::IntValue(b)) = (l, r) {
                        Ok(ctx
                            .builder
                            .build_int_compare(inkwell::IntPredicate::SGE, a, b, "ge")?
                            .into())
                    } else {
                        Err(crate::error::semantic_err("Type mismatch in >="))
                    }
                }
                BinOp::Mod => {
                    if let (BasicValueEnum::IntValue(a), BasicValueEnum::IntValue(b)) = (l, r) {
                        Ok(ctx.builder.build_int_signed_rem(a, b, "mod")?.into())
                    } else {
                        Err(crate::error::semantic_err("Type mismatch in %"))
                    }
                }
                BinOp::And => {
                    if let (BasicValueEnum::IntValue(a), BasicValueEnum::IntValue(b)) = (l, r) {
                        Ok(ctx.builder.build_and(a, b, "and")?.into())
                    } else {
                        Err(crate::error::semantic_err("Type mismatch in &&"))
                    }
                }
                BinOp::Or => {
                    if let (BasicValueEnum::IntValue(a), BasicValueEnum::IntValue(b)) = (l, r) {
                        Ok(ctx.builder.build_or(a, b, "or")?.into())
                    } else {
                        Err(crate::error::semantic_err("Type mismatch in ||"))
                    }
                }
            }
        }
        Expr::Unary { op, operand } => {
            let val = emit_expr(ctx, operand)?;
            match op {
                UnOp::Neg => {
                    if let BasicValueEnum::IntValue(i) = val {
                        Ok(ctx.builder.build_int_neg(i, "neg")?.into())
                    } else if let BasicValueEnum::FloatValue(f) = val {
                        Ok(ctx.builder.build_float_neg(f, "neg")?.into())
                    } else {
                        Err(crate::error::semantic_err("Type mismatch in unary -"))
                    }
                }
                UnOp::Not => {
                    if let BasicValueEnum::IntValue(i) = val {
                        let zero = i.get_type().const_zero();
                        let cmp = ctx.builder.build_int_compare(
                            inkwell::IntPredicate::EQ,
                            i,
                            zero,
                            "not_cmp",
                        )?;
                        Ok(ctx
                            .builder
                            .build_int_z_extend(cmp, i.get_type(), "not")?
                            .into())
                    } else {
                        Err(crate::error::semantic_err("Type mismatch in !"))
                    }
                }
            }
        }
        Expr::Call { callee, args } => {
            if let Expr::Ident(name) = callee.as_ref() {
                if let Some(builtin) = emit_builtin_call(ctx, name, args)? {
                    return Ok(builtin);
                }
            }

            let fn_name = match callee.as_ref() {
                Expr::Ident(n) => n.clone(),
                Expr::Field { base, field } => {
                    if let Expr::Ident(mod_name) = base.as_ref() {
                        format!("{}_{}", mod_name, field)
                    } else {
                        return Err(crate::error::semantic_err("Unsupported call"));
                    }
                }
                _ => return Err(crate::error::semantic_err("Unsupported call")),
            };
            let fn_val = ctx.module.get_function(&fn_name).ok_or_else(|| {
                crate::error::semantic_err(format!("Function not found: {}", fn_name))
            })?;
            let arg_vals: Vec<inkwell::values::BasicMetadataValueEnum> = args
                .iter()
                .map(|a| emit_expr(ctx, a))
                .collect::<Result<Vec<_>>>()?
                .into_iter()
                .map(|v| v.into())
                .collect();
            let call = ctx.builder.build_call(fn_val, &arg_vals, "call")?;
            Ok(call
                .try_as_basic_value()
                .basic()
                .map(|v| v.into())
                .unwrap_or_else(|| ctx.context.i64_type().const_zero().into()))
        }
        _ => Err(crate::error::semantic_err(format!(
            "LLVM backend: unsupported expression {:?}",
            expr
        ))),
    }
}

fn expr_type(expr: &Expr, ctx: &LlvmCtx) -> Option<Type> {
    match expr {
        Expr::Literal(Literal::Int(_)) => Some(Type::I64),
        Expr::Literal(Literal::Float(_)) => Some(Type::F64),
        Expr::Literal(Literal::Bool(_)) => Some(Type::Bool),
        Expr::Literal(Literal::Str(_)) => Some(Type::Str),
        Expr::Ident(name) => ctx.var_types.get(name).cloned(),
        Expr::Move(inner) => expr_type(inner, ctx),
        _ => None,
    }
}

/// Returns extra libraries to link when the program uses certain modules (e.g. gui -> raylib).
pub fn required_link_libs(program: &crate::ast::Program) -> Vec<&'static str> {
    let mut libs = Vec::new();
    if program_uses_module(program, "gui") {
        libs.push("raylib");
    }
    libs
}

fn program_uses_module(program: &crate::ast::Program, name: &str) -> bool {
    for item in &program.items {
        match item {
            crate::ast::TopLevelItem::Import(i)
                if i.path.first().map(|s| s.as_str()) == Some(name) =>
            {
                return true;
            }
            crate::ast::TopLevelItem::Mod(m) if m.name == name => return true,
            _ => {}
        }
    }
    false
}
