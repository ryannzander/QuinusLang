//! Semantic analysis for QuinusLang

use crate::ast::*;
use crate::error::{Error, Result};
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone)]
pub struct AnnotatedProgram {
    pub program: Program,
    pub symbol_table: SymbolTable,
}

#[derive(Debug, Clone, Default)]
pub struct SymbolTable {
    pub scopes: Vec<Scope>,
}

#[derive(Debug, Clone, Default)]
pub struct Scope {
    pub vars: HashMap<String, Type>,
    pub mutable_vars: HashSet<String>,
    pub funcs: HashMap<String, FuncSig>,
    pub structs: HashMap<String, StructDef>,
    pub classes: HashMap<String, ClassDef>,
}

#[derive(Debug, Clone)]
pub struct FuncSig {
    pub params: Vec<Type>,
    pub return_type: Option<Type>,
}

pub fn analyze(program: &Program) -> Result<AnnotatedProgram> {
    let mut symbol_table = SymbolTable::default();
    symbol_table.scopes.push(Scope::default());

    // Register builtin: print (accepts any args, returns void)
    symbol_table.scopes.last_mut().unwrap().funcs.insert(
        "print".to_string(),
        FuncSig {
            params: vec![],
            return_type: Some(Type::Void),
        },
    );

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
    let scope = symbol_table.scopes.last_mut().unwrap();
    match item {
        TopLevelItem::Fn(f) => {
            let params: Vec<Type> = f.params.iter().map(|p| p.ty.clone()).collect();
            scope.funcs.insert(
                f.name.clone(),
                FuncSig {
                    params,
                    return_type: f.return_type.clone(),
                },
            );
        }
        TopLevelItem::Struct(s) => {
            scope.structs.insert(s.name.clone(), s.clone());
        }
        TopLevelItem::Class(c) => {
            scope.classes.insert(c.name.clone(), c.clone());
        }
        TopLevelItem::Mod(m) => {
            for sub in &m.items {
                register_top_level(symbol_table, sub)?;
            }
        }
        TopLevelItem::Import(_) => {}
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
        TopLevelItem::Struct(_) | TopLevelItem::Class(_) | TopLevelItem::Import(_) => {}
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
                    return Err(Error::Semantic {
                        message: format!("Cannot assign {} to {}", init_ty, decl_ty),
                    });
                }
            }
            let scope = symbol_table.scopes.last_mut().unwrap();
            scope.vars.insert(name.clone(), var_ty);
            if *mutable {
                scope.mutable_vars.insert(name.clone());
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
                        return Err(Error::Semantic {
                            message: format!("Undefined variable: {}", name),
                        });
                    }
                    if !is_mutable {
                        return Err(Error::Semantic {
                            message: format!("Cannot assign to immutable variable: {}", name),
                        });
                    }
                }
                AssignTarget::Deref(operand) => {
                    let ty = check_expr(symbol_table, operand)?;
                    if !matches!(ty, Type::Ptr(_)) {
                        return Err(Error::Semantic {
                            message: "Cannot assign through non-pointer".to_string(),
                        });
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
        Stmt::ExprStmt(e) => {
            check_expr(symbol_table, e)?;
        }
        Stmt::Return(expr) => {
            if let Some(e) = expr {
                check_expr(symbol_table, e)?;
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
            }
            Err(Error::Semantic {
                message: format!("Undefined variable: {}", name),
            })
        }
        Expr::Binary { op, left, right } => {
            let lt = check_expr(symbol_table, left)?;
            let rt = check_expr(symbol_table, right)?;
            match op {
                BinOp::Add | BinOp::Sub | BinOp::Mul | BinOp::Div | BinOp::Mod => {
                    if lt != rt {
                        return Err(Error::Semantic {
                            message: "Type mismatch in arithmetic".to_string(),
                        });
                    }
                    Ok(lt)
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
                _ => Err(Error::Semantic {
                    message: "Cannot dereference non-pointer".to_string(),
                }),
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
            for arg in args {
                check_expr(symbol_table, arg)?;
            }
            if let Expr::Ident(name) = callee.as_ref() {
                for scope in symbol_table.scopes.iter().rev() {
                    if let Some(sig) = scope.funcs.get(name) {
                        return Ok(sig.return_type.clone().unwrap_or(Type::Void));
                    }
                }
            }
            Ok(Type::Void)
        }
        Expr::Index { base, index } => {
            let _ = check_expr(symbol_table, base)?;
            check_expr(symbol_table, index)?;
            Ok(Type::Int) // Simplified - array element type
        }
        Expr::Field { base, field: _ } => {
            let _ = check_expr(symbol_table, base)?;
            Ok(Type::Int) // Simplified
        }
        Expr::New { class, args: _ } => {
            for scope in symbol_table.scopes.iter().rev() {
                if scope.classes.contains_key(class) {
                    return Ok(Type::Named(class.clone()));
                }
            }
            Err(Error::Semantic {
                message: format!("Unknown class: {}", class),
            })
        }
    }
}
