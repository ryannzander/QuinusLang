//! Semantic analysis for QuinusLang

use crate::ast::*;
use crate::error::{semantic_err, semantic_err_hint, Result};
use std::collections::{HashMap, HashSet};

fn find_similar(name: &str, symbol_table: &SymbolTable) -> Option<String> {
    let mut candidates: Vec<&str> = Vec::new();
    for scope in &symbol_table.scopes {
        for k in scope.vars.keys() {
            candidates.push(k);
        }
        for k in scope.funcs.keys() {
            candidates.push(k);
        }
    }
    let mut best: Option<(&str, usize)> = None;
    for c in candidates {
        let d = edit_distance(name, c);
        if d <= 3 && (best.is_none() || d < best.unwrap().1) {
            best = Some((c, d));
        }
    }
    best.map(|(s, _)| format!("Did you mean `{}`?", s))
}

fn edit_distance(a: &str, b: &str) -> usize {
    let a: Vec<char> = a.chars().collect();
    let b: Vec<char> = b.chars().collect();
    let mut dp = vec![vec![0; b.len() + 1]; a.len() + 1];
    for i in 0..=a.len() {
        dp[i][0] = i;
    }
    for j in 0..=b.len() {
        dp[0][j] = j;
    }
    for i in 1..=a.len() {
        for j in 1..=b.len() {
            let cost = if a[i - 1] == b[j - 1] { 0 } else { 1 };
            dp[i][j] = (dp[i - 1][j] + 1).min(dp[i][j - 1] + 1).min(dp[i - 1][j - 1] + cost);
        }
    }
    dp[a.len()][b.len()]
}

#[derive(Debug, Clone)]
pub struct AnnotatedProgram {
    pub program: Program,
    pub symbol_table: SymbolTable,
}

#[derive(Debug, Clone, Default)]
pub struct SymbolTable {
    pub scopes: Vec<Scope>,
    pub mod_functions: HashMap<String, HashMap<String, FuncSig>>,
}

#[derive(Debug, Clone, Default)]
pub struct Scope {
    pub vars: HashMap<String, Type>,
    pub mutable_vars: HashSet<String>,
    pub funcs: HashMap<String, FuncSig>,
    pub structs: HashMap<String, StructDef>,
    pub classes: HashMap<String, ClassDef>,
    pub enums: HashMap<String, EnumDef>,
    pub unions: HashMap<String, UnionDef>,
    pub modules: HashSet<String>,
    pub aliases: HashMap<String, Type>,
}

#[derive(Debug, Clone)]
pub struct FuncSig {
    pub params: Vec<Type>,
    pub return_type: Option<Type>,
    pub c_name: Option<String>,
}

pub fn analyze(program: &Program) -> Result<AnnotatedProgram> {
    let mut symbol_table = SymbolTable::default();
    symbol_table.scopes.push(Scope::default());

    // Register builtins
    let scope = symbol_table.scopes.last_mut().unwrap();
    scope.funcs.insert("print".to_string(), FuncSig { params: vec![], return_type: Some(Type::Void), c_name: None });
    scope.funcs.insert("write".to_string(), FuncSig { params: vec![], return_type: Some(Type::Void), c_name: None });
    scope.funcs.insert("writeln".to_string(), FuncSig { params: vec![], return_type: Some(Type::Void), c_name: None });
    scope.funcs.insert("read".to_string(), FuncSig { params: vec![], return_type: Some(Type::I32), c_name: None });
    scope.funcs.insert("len".to_string(), FuncSig { params: vec![Type::Array(Box::new(Type::Int))], return_type: Some(Type::Usize), c_name: None });
    scope.funcs.insert("strlen".to_string(), FuncSig { params: vec![Type::Str], return_type: Some(Type::Usize), c_name: None });
    scope.funcs.insert("panic".to_string(), FuncSig { params: vec![], return_type: Some(Type::Void), c_name: None });
    scope.funcs.insert("assert".to_string(), FuncSig { params: vec![Type::Bool], return_type: Some(Type::Void), c_name: None });
    scope.funcs.insert("__ql_null_at".to_string(), FuncSig { params: vec![Type::Ptr(Box::new(Type::Void)), Type::Usize], return_type: Some(Type::Void), c_name: None });

    for item in &program.items {
        register_top_level(&mut symbol_table, item)?;
    }

    for item in &program.items {
        check_top_level(&mut symbol_table, item)?;
    }

    Ok(AnnotatedProgram {
        program: program.clone(),
        symbol_table,
    })
}

fn register_top_level(symbol_table: &mut SymbolTable, item: &TopLevelItem) -> Result<()> {
    register_top_level_with_prefix(symbol_table, item, None)
}

fn register_top_level_with_prefix(symbol_table: &mut SymbolTable, item: &TopLevelItem, mod_prefix: Option<&str>) -> Result<()> {
    let scope = symbol_table.scopes.last_mut().unwrap();
    match item {
        TopLevelItem::Fn(f) => {
            let params: Vec<Type> = f.params.iter().map(|p| p.ty.clone()).collect();
            let c_name = mod_prefix.map(|p| format!("{}_{}", p, f.name));
            scope.funcs.insert(
                f.name.clone(),
                FuncSig {
                    params,
                    return_type: f.return_type.clone(),
                    c_name,
                },
            );
        }
        TopLevelItem::Struct(s) => {
            scope.structs.insert(s.name.clone(), s.clone());
        }
        TopLevelItem::Const(c) => {
            scope.vars.insert(c.name.clone(), c.ty.clone());
        }
        TopLevelItem::Static(s) => {
            scope.vars.insert(s.name.clone(), s.ty.clone());
        }
        TopLevelItem::Enum(e) => {
            scope.enums.insert(e.name.clone(), e.clone());
        }
        TopLevelItem::Union(u) => {
            scope.unions.insert(u.name.clone(), u.clone());
        }
        TopLevelItem::Class(c) => {
            scope.classes.insert(c.name.clone(), c.clone());
        }
        TopLevelItem::Mod(m) => {
            scope.modules.insert(m.name.clone());
            let mut mod_funcs = HashMap::new();
            for sub in &m.items {
                if let TopLevelItem::Fn(f) = sub {
                    let params: Vec<Type> = f.params.iter().map(|p| p.ty.clone()).collect();
                    let sig = FuncSig {
                        params: params.clone(),
                        return_type: f.return_type.clone(),
                        c_name: Some(format!("{}_{}", m.name, f.name)),
                    };
                    mod_funcs.insert(f.name.clone(), sig);
                }
                register_top_level_with_prefix(symbol_table, sub, Some(&m.name))?;
            }
            symbol_table.mod_functions.insert(m.name.clone(), mod_funcs);
        }
        TopLevelItem::Import(_) => {}
        TopLevelItem::Alias(a) => {
            scope.aliases.insert(a.name.clone(), a.ty.clone());
        }
        TopLevelItem::Extern(e) => {
            let params: Vec<Type> = e.params.iter().map(|p| p.ty.clone()).collect();
            scope.funcs.insert(
                e.name.clone(),
                FuncSig {
                    params,
                    return_type: e.return_type.clone(),
                    c_name: None,
                },
            );
        }
        TopLevelItem::Impl(impl_def) => {
            for m in &impl_def.methods {
                scope.funcs.insert(
                    format!("{}_{}", impl_def.struct_name, m.name),
                    FuncSig {
                        params: m.params.iter().map(|p| p.ty.clone()).collect(),
                        return_type: m.return_type.clone(),
                        c_name: None,
                    },
                );
            }
        }
    }
    Ok(())
}

fn check_top_level(symbol_table: &mut SymbolTable, item: &TopLevelItem) -> Result<()> {
    match item {
        TopLevelItem::Fn(f) => {
            symbol_table.scopes.push(Scope::default());
            for p in &f.params {
                let scope = symbol_table.scopes.last_mut().unwrap();
                scope.vars.insert(p.name.clone(), p.ty.clone());
                // Params are immutable by default
            }
            for stmt in &f.body {
                check_stmt(symbol_table, stmt)?;
            }
            symbol_table.scopes.pop();
        }
        TopLevelItem::Const(c) => {
            let init_ty = check_expr(symbol_table, &c.init)?;
            if !is_assignable(&init_ty, &c.ty) {
                return Err(semantic_err(format!("Cannot assign {} to constant {}", init_ty, c.ty)));
            }
        }
        TopLevelItem::Static(s) => {
            if let Some(init) = &s.init {
                let init_ty = check_expr(symbol_table, init)?;
                if !is_assignable(&init_ty, &s.ty) {
                    return Err(semantic_err(format!("Cannot assign {} to static {}", init_ty, s.ty)));
                }
            }
        }
        TopLevelItem::Struct(_) | TopLevelItem::Class(_) | TopLevelItem::Enum(_) | TopLevelItem::Union(_) | TopLevelItem::Import(_) | TopLevelItem::Alias(_) | TopLevelItem::Extern(_) => {}
        TopLevelItem::Impl(impl_def) => {
            symbol_table.scopes.push(Scope::default());
            for m in &impl_def.methods {
                for p in &m.params {
                    let scope = symbol_table.scopes.last_mut().unwrap();
                    scope.vars.insert(p.name.clone(), p.ty.clone());
                }
                for stmt in &m.body {
                    check_stmt(symbol_table, stmt)?;
                }
            }
            symbol_table.scopes.pop();
        }
        TopLevelItem::Mod(m) => {
            for sub in &m.items {
                check_top_level(symbol_table, sub)?;
            }
        }
    }
    Ok(())
}

fn is_assignable(from: &Type, to: &Type) -> bool {
    if from == to {
        return true;
    }
    if let (Type::Named(from_name), Type::Named(to_name)) = (from, to) {
        if from_name == to_name {
            return true;
        }
    }
    if let (Type::ArraySized(from_inner, from_n), Type::ArraySized(to_inner, to_n)) = (from, to) {
        return from_n == to_n && is_assignable(from_inner, to_inner);
    }
    match (from, to) {
        (Type::Int, Type::U32 | Type::U64 | Type::I32 | Type::I64 | Type::Usize) => true,
        (Type::Int, Type::Float | Type::F32 | Type::F64) => true,
        (Type::Float, Type::F32 | Type::F64) => true,
        _ => false,
    }
}

fn check_stmt(symbol_table: &mut SymbolTable, stmt: &Stmt) -> Result<()> {
    match stmt {
        Stmt::VarDecl { name, ty, init, mutable } => {
            let init_ty = check_expr(symbol_table, init)?;
            let var_ty = ty.clone().unwrap_or_else(|| init_ty.clone());
            if let Some(decl_ty) = ty {
                if !is_assignable(&init_ty, &decl_ty) {
                    return Err(semantic_err(format!("Cannot assign {} to {}", init_ty, decl_ty)));
                }
            }
            let scope = symbol_table.scopes.last_mut().unwrap();
            scope.vars.insert(name.clone(), var_ty);
            if *mutable {
                scope.mutable_vars.insert(name.clone());
            }
        }
        Stmt::VarDeclTuple { names, init, mutable } => {
            let init_ty = check_expr(symbol_table, init)?;
            let elem_tys = match &init_ty {
                Type::Tuple(inner) => inner,
                _ => {
                    return Err(semantic_err(format!("Tuple destructuring requires tuple type, got {}", init_ty)));
                }
            };
            if names.len() != elem_tys.len() {
                return Err(semantic_err(format!("Tuple has {} elements but {} variables", elem_tys.len(), names.len())));
            }
            let scope = symbol_table.scopes.last_mut().unwrap();
            for (name, ty) in names.iter().zip(elem_tys.iter()) {
                scope.vars.insert(name.clone(), ty.clone());
                if *mutable {
                    scope.mutable_vars.insert(name.clone());
                }
            }
        }
        Stmt::Assign { target, value } => {
            check_expr(symbol_table, value)?;
            match target {
                AssignTarget::Ident(name) => {
                    let mut found = false;
                    let mut is_mutable = false;
                    for scope in symbol_table.scopes.iter().rev() {
                        if scope.vars.contains_key(name) {
                            found = true;
                            is_mutable = scope.mutable_vars.contains(name);
                            break;
                        }
                    }
                    if !found {
                        let hint = find_similar(name, symbol_table);
                        return Err(if let Some(h) = hint {
                            semantic_err_hint(format!("Undefined variable: {}", name), h)
                        } else {
                            semantic_err(format!("Undefined variable: {}", name))
                        });
                    }
                    if !is_mutable {
                        return Err(semantic_err(format!("Cannot assign to immutable variable: {}", name)));
                    }
                }
                AssignTarget::Deref(operand) => {
                    let ty = check_expr(symbol_table, operand)?;
                    if !matches!(ty, Type::Ptr(_)) {
                        return Err(semantic_err("Cannot assign through non-pointer"));
                    }
                }
                _ => {}
            }
        }
        Stmt::If { cond, then_body, else_body } => {
            check_expr(symbol_table, cond)?;
            symbol_table.scopes.push(Scope::default());
            for s in then_body {
                check_stmt(symbol_table, s)?;
            }
            symbol_table.scopes.pop();
            if let Some(else_b) = else_body {
                symbol_table.scopes.push(Scope::default());
                for s in else_b {
                    check_stmt(symbol_table, s)?;
                }
                symbol_table.scopes.pop();
            }
        }
        Stmt::For { init, cond, step, body } => {
            symbol_table.scopes.push(Scope::default());
            if let Some(i) = init {
                check_stmt(symbol_table, i)?;
            }
            if let Some(c) = cond {
                check_expr(symbol_table, c)?;
            }
            if let Some(s) = step {
                check_stmt(symbol_table, s)?;
            }
            for st in body {
                check_stmt(symbol_table, st)?;
            }
            symbol_table.scopes.pop();
        }
        Stmt::While { cond, body } => {
            check_expr(symbol_table, cond)?;
            symbol_table.scopes.push(Scope::default());
            for s in body {
                check_stmt(symbol_table, s)?;
            }
            symbol_table.scopes.pop();
        }
        Stmt::Foreach { var, iter, body } => {
            let iter_ty = check_expr(symbol_table, iter)?;
            symbol_table.scopes.push(Scope::default());
            if let Type::Array(elem) = &iter_ty {
                symbol_table.scopes.last_mut().unwrap().vars.insert(var.clone(), *elem.clone());
            } else {
                symbol_table.scopes.last_mut().unwrap().vars.insert(var.clone(), Type::Int);
            }
            for s in body {
                check_stmt(symbol_table, s)?;
            }
            symbol_table.scopes.pop();
        }
        Stmt::Break | Stmt::Continue => {}
        Stmt::Hazard { body } => {
            for s in body {
                check_stmt(symbol_table, s)?;
            }
        }
        Stmt::InlineAsm { .. } => {}
        Stmt::ExprStmt(e) => {
            check_expr(symbol_table, e)?;
        }
        Stmt::Return(expr) => {
            if let Some(e) = expr {
                check_expr(symbol_table, e)?;
            }
        }
        Stmt::Defer { body } => {
            for s in body {
                check_stmt(symbol_table, s)?;
            }
        }
        Stmt::Choose { expr, arms } => {
            check_expr(symbol_table, expr)?;
            for arm in arms {
                for s in &arm.body {
                    check_stmt(symbol_table, s)?;
                }
            }
        }
        Stmt::TryCatch { try_body, catch_body, .. } => {
            symbol_table.scopes.push(Scope::default());
            for s in try_body {
                check_stmt(symbol_table, s)?;
            }
            symbol_table.scopes.pop();
            symbol_table.scopes.push(Scope::default());
            for s in catch_body {
                check_stmt(symbol_table, s)?;
            }
            symbol_table.scopes.pop();
        }
    }
    Ok(())
}

/// Get the type of an expression, if type-checking succeeds.
pub fn type_of_expr(symbol_table: &SymbolTable, expr: &Expr) -> Option<Type> {
    check_expr(symbol_table, expr).ok()
}

fn check_expr(symbol_table: &SymbolTable, expr: &Expr) -> Result<Type> {
    match expr {
        Expr::Literal(l) => Ok(match l {
            Literal::Int(_) => Type::Int,
            Literal::Float(_) => Type::Float,
            Literal::Bool(_) => Type::Bool,
            Literal::Str(_) => Type::Str,
        }),
        Expr::Ident(name) => {
            for scope in symbol_table.scopes.iter().rev() {
                if let Some(ty) = scope.vars.get(name) {
                    return Ok(ty.clone());
                }
                if scope.funcs.contains_key(name) {
                    return Ok(Type::Void); // Function reference (used in calls)
                }
                if scope.modules.contains(name) {
                    return Ok(Type::Void); // Module reference (used in mod.fn calls)
                }
                for (enum_name, e) in &scope.enums {
                    if e.variants.iter().any(|v| match v {
                        crate::ast::EnumVariant::Unit(n) => n == name,
                        crate::ast::EnumVariant::Tuple(n, _) => n == name,
                    }) {
                        return Ok(Type::Named(enum_name.clone()));
                    }
                }
            }
            Err(if let Some(h) = find_similar(name, symbol_table) {
                semantic_err_hint(format!("Undefined variable: {}", name), h)
            } else {
                semantic_err(format!("Undefined variable: {}", name))
            })
        }
        Expr::Binary { op, left, right } => {
            let lt = check_expr(symbol_table, left)?;
            let rt = check_expr(symbol_table, right)?;
            match op {
                BinOp::Add | BinOp::Sub | BinOp::Mul | BinOp::Div | BinOp::Mod => {
                    if op == &BinOp::Add && lt == Type::Str && rt == Type::Str {
                        return Ok(Type::Str);
                    }
                    // Allow int/usize mix for index arithmetic (usize wins)
                    // Allow int/i32/i64 mix (prefer the explicit type)
                    let unified = if (lt == Type::Usize && matches!(rt, Type::Int | Type::I32 | Type::I64))
                        || (rt == Type::Usize && matches!(lt, Type::Int | Type::I32 | Type::I64))
                    {
                        Type::Usize
                    } else if (matches!(lt, Type::Int | Type::I32 | Type::I64)
                        && matches!(rt, Type::Int | Type::I32 | Type::I64))
                    {
                        // Prefer i64 > i32 > int for result
                        if lt == Type::I64 || rt == Type::I64 {
                            Type::I64
                        } else if lt == Type::I32 || rt == Type::I32 {
                            Type::I32
                        } else {
                            Type::Int
                        }
                    } else if lt != rt {
                        return Err(semantic_err("Type mismatch in arithmetic"));
                    } else {
                        lt.clone()
                    };
                    Ok(unified)
                }
                BinOp::Eq | BinOp::Ne | BinOp::Lt | BinOp::Le | BinOp::Gt | BinOp::Ge => {
                    Ok(Type::Bool)
                }
                BinOp::And | BinOp::Or => Ok(Type::Bool),
            }
        }
        Expr::AddrOf(operand) => {
            let inner = check_expr(symbol_table, operand)?;
            Ok(Type::Ptr(Box::new(inner)))
        }
        Expr::Deref(operand) => {
            let inner = check_expr(symbol_table, operand)?;
            match inner {
                Type::Ptr(t) => Ok(*t),
                _ => Err(semantic_err("Cannot dereference non-pointer")),
            }
        }
        Expr::Unary { op, operand } => {
            check_expr(symbol_table, operand)?;
            match op {
                UnOp::Neg => Ok(Type::Int), // or Float
                UnOp::Not => Ok(Type::Bool),
            }
        }
        Expr::Call { callee, args } => {
            if let Expr::Ident(name) = callee.as_ref() {
                if name == "len" {
                    if args.len() != 1 {
                        return Err(semantic_err("len() takes exactly 1 argument"));
                    }
                    let arg_ty = check_expr(symbol_table, &args[0])?;
                    if !matches!(arg_ty, Type::Array(_) | Type::ArraySized(_, _)) {
                        return Err(semantic_err("len() requires an array argument"));
                    }
                    return Ok(Type::Usize);
                }
                if name == "strlen" {
                    if args.len() != 1 {
                        return Err(semantic_err("strlen() takes exactly 1 argument"));
                    }
                    let arg_ty = check_expr(symbol_table, &args[0])?;
                    if arg_ty != Type::Str {
                        return Err(semantic_err("strlen() requires a string argument"));
                    }
                    return Ok(Type::Usize);
                }
                if name == "panic" {
                    for arg in args {
                        check_expr(symbol_table, arg)?;
                    }
                    return Ok(Type::Void);
                }
                if name == "assert" {
                    if args.len() != 1 {
                        return Err(semantic_err("assert() takes exactly 1 argument"));
                    }
                    let arg_ty = check_expr(symbol_table, &args[0])?;
                    if arg_ty != Type::Bool {
                        return Err(semantic_err("assert() requires a bool argument"));
                    }
                    return Ok(Type::Void);
                }
                if name == "read" {
                    if !args.is_empty() {
                        return Err(semantic_err("read() takes no arguments"));
                    }
                    return Ok(Type::I32);
                }
                for arg in args {
                    check_expr(symbol_table, arg)?;
                }
                for scope in symbol_table.scopes.iter().rev() {
                    if let Some(sig) = scope.funcs.get(name) {
                        return Ok(sig.return_type.clone().unwrap_or(Type::Void));
                    }
                }
            } else if let Expr::Field { base, field } = callee.as_ref() {
                if let Expr::Ident(mod_name) = base.as_ref() {
                    if let Some(mod_funcs) = symbol_table.mod_functions.get(mod_name) {
                        if let Some(sig) = mod_funcs.get(field) {
                            for arg in args {
                                check_expr(symbol_table, arg)?;
                            }
                            return Ok(sig.return_type.clone().unwrap_or(Type::Void));
                        }
                    }
                }
                let _ = check_expr(symbol_table, base)?;
                for arg in args {
                    check_expr(symbol_table, arg)?;
                }
            } else {
                for arg in args {
                    check_expr(symbol_table, arg)?;
                }
            }
            Ok(Type::Void)
        }
        Expr::Index { base, index } => {
            let base_ty = check_expr(symbol_table, base)?;
            let _ = check_expr(symbol_table, index)?;
            match base_ty {
                Type::Array(inner) | Type::ArraySized(inner, _) => Ok(*inner),
                _ => Ok(Type::Int),
            }
        }
        Expr::Slice { base, start, end } => {
            let base_ty = check_expr(symbol_table, base)?;
            if let Some(s) = start {
                check_expr(symbol_table, s)?;
            }
            if let Some(e) = end {
                check_expr(symbol_table, e)?;
            }
            match base_ty {
                Type::Array(inner) | Type::ArraySized(inner, _) => Ok(Type::Array(inner)),
                _ => Err(semantic_err("Slice requires array type")),
            }
        }
        Expr::ArrayInit(elems) => {
            if elems.is_empty() {
                return Err(semantic_err("Array initializer cannot be empty"));
            }
            let first_ty = check_expr(symbol_table, &elems[0])?;
            for e in elems.iter().skip(1) {
                let t = check_expr(symbol_table, e)?;
                if !is_assignable(&t, &first_ty) && !is_assignable(&first_ty, &t) {
                    return Err(semantic_err("Array elements must have compatible types"));
                }
            }
            Ok(Type::ArraySized(Box::new(first_ty), elems.len() as u32))
        }
        Expr::Field { base, field: _ } => {
            let _ = check_expr(symbol_table, base)?;
            Ok(Type::Int) // Simplified
        }
        Expr::Cast { operand, target_ty } => {
            let _ = check_expr(symbol_table, operand)?;
            Ok(target_ty.clone())
        }
        Expr::Range { start, end } => {
            let _ = check_expr(symbol_table, start)?;
            let _ = check_expr(symbol_table, end)?;
            Ok(Type::Int)
        }
        Expr::Tuple(elems) => {
            let mut tys = Vec::new();
            for e in elems {
                tys.push(check_expr(symbol_table, e)?);
            }
            Ok(Type::Tuple(tys))
        }
        Expr::Interpolate(parts) => {
            for p in parts {
                if let InterpolatePart::Expr(e) = p {
                    check_expr(symbol_table, e)?;
                }
            }
            Ok(Type::Void)
        }
        Expr::New { class, args: _ } => {
            for scope in symbol_table.scopes.iter().rev() {
                if scope.classes.contains_key(class) {
                    return Ok(Type::Named(class.clone()));
                }
            }
            Err(semantic_err(format!("Unknown class: {}", class)))
        }
    }
}
