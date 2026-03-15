//! Parser for QuinusLang

use crate::ast::{EnumVariant, *};
use crate::error::{Error, Result};
use crate::lexer::{Token, TokenStream};
use std::path::Path;

pub fn parse(source: &str) -> Result<Program> {
    let mut stream = crate::lexer::tokenize(source)?;
    parse_program(&mut stream)
}

pub fn parse_from_stream(stream: &mut TokenStream) -> Result<Program> {
    parse_program(stream)
}

/// Resolve imports and return a flattened program. base_dir is the directory
/// containing the entry file. packages is optional map of package name -> path for deps.
pub fn parse_with_imports(
    source: &str,
    base_dir: &Path,
    packages: &[(String, std::path::PathBuf)],
) -> Result<Program> {
    let program = parse(source)?;
    resolve_imports(program, base_dir, packages)
}

fn resolve_imports(
    program: Program,
    base_dir: &Path,
    packages: &[(String, std::path::PathBuf)],
) -> Result<Program> {
    let mut items = Vec::new();
    let mut seen = std::collections::HashSet::new();
    for item in program.items {
        match &item {
            TopLevelItem::Import(imp) => {
                let path_str = imp.path.join(".");
                if seen.contains(&path_str) {
                    continue;
                }
                seen.insert(path_str.clone());
                let file_path = resolve_import_path(base_dir, packages, &imp.path)?;
                let source = std::fs::read_to_string(&file_path).map_err(|e| Error::Package {
                    message: format!("Failed to read {}: {}", file_path.display(), e),
                })?;
                let sub_dir = file_path.parent().unwrap_or(base_dir);
                let sub_program = parse_with_imports(&source, sub_dir, packages)?;
                for sub in sub_program.items {
                    if !matches!(&sub, TopLevelItem::Import(_)) {
                        items.push(sub);
                    }
                }
            }
            _ => items.push(item),
        }
    }
    Ok(Program { items })
}

fn resolve_import_path(
    base_dir: &Path,
    packages: &[(String, std::path::PathBuf)],
    path: &[String],
) -> Result<std::path::PathBuf> {
    if let Some(first) = path.first() {
        for (pkg_name, pkg_path) in packages {
            if pkg_name == first {
                let rest: std::path::PathBuf = path.iter().skip(1).collect();
                let sub_path = if rest.as_os_str().is_empty() {
                    std::path::PathBuf::from("src/main.q")
                } else {
                    rest.with_extension("q")
                };
                let candidates = [
                    pkg_path.join(&sub_path),
                    pkg_path.join("src").join(sub_path.file_name().unwrap_or(std::ffi::OsStr::new("main.q"))),
                ];
                for p in &candidates {
                    if p.exists() {
                        return Ok(p.clone());
                    }
                }
            }
        }
    }
    let rel: std::path::PathBuf = path.iter().collect();
    let ext = rel.with_extension("q");
    let candidates = [
        base_dir.join(&ext),
        base_dir.join("src").join(&ext),
        base_dir.join(rel.join("mod.q")),
        base_dir.join("stdlib").join(&ext),
        base_dir.join("stdlib").join(rel.join("mod.q")),
    ];
    for p in &candidates {
        if p.exists() {
            return Ok(p.clone());
        }
    }
    let path_str = path.join(".");
    Err(Error::Package {
        message: format!("Module not found: {} (tried {:?})", path_str, candidates),
    })
}

fn parse_program(stream: &mut TokenStream) -> Result<Program> {
    let mut items = Vec::new();
    while !stream.is_at_end() {
        items.push(parse_top_level(stream)?);
    }
    Ok(Program { items })
}

fn parse_top_level(stream: &mut TokenStream) -> Result<TopLevelItem> {
    let (line, col) = stream.peek_pos().unwrap_or((1, 1));

    match stream.peek() {
        Some(Token::Eternal) => {
            stream.consume();
            Ok(TopLevelItem::Const(parse_const_def(stream)?))
        }
        Some(Token::Anchor) => {
            stream.consume();
            Ok(TopLevelItem::Static(parse_static_def(stream)?))
        }
        Some(Token::Open) => {
            stream.consume();
            let open = true;
            match stream.peek() {
                Some(Token::Craft) => {
                    stream.consume();
                    Ok(TopLevelItem::Fn(parse_fn_def_with_open(stream, open)?))
                }
                Some(Token::Form) => {
                    stream.consume();
                    Ok(TopLevelItem::Struct(parse_struct_def(stream)?))
                }
                _ => {
                    let (line, col) = stream.peek_pos().unwrap_or((1, 1));
                    Err(Error::Parse {
                        line,
                        col,
                        message: "Expected craft or form after open".to_string(),
                    })
                }
            }
        }
        Some(Token::Craft) => {
            stream.consume();
            Ok(TopLevelItem::Fn(parse_fn_def(stream)?))
        }
        Some(Token::Form) => {
            stream.consume();
            Ok(TopLevelItem::Struct(parse_struct_def(stream)?))
        }
        Some(Token::State) => {
            stream.consume();
            Ok(TopLevelItem::Enum(parse_enum_def(stream)?))
        }
        Some(Token::Fusion) => {
            stream.consume();
            Ok(TopLevelItem::Union(parse_union_def(stream)?))
        }
        Some(Token::Class) => {
            stream.consume();
            Ok(TopLevelItem::Class(parse_class_def(stream)?))
        }
        Some(Token::Realm) => {
            stream.consume();
            Ok(TopLevelItem::Mod(parse_mod_def(stream)?))
        }
        Some(Token::Import) | Some(Token::Bring) => {
            stream.consume();
            Ok(TopLevelItem::Import(parse_import(stream)?))
        }
        Some(Token::Alias) => {
            stream.consume();
            Ok(TopLevelItem::Alias(parse_alias_def(stream)?))
        }
        Some(Token::Impl) => {
            stream.consume();
            Ok(TopLevelItem::Impl(parse_impl_def(stream)?))
        }
        Some(Token::Extern) => {
            stream.consume();
            Ok(TopLevelItem::Extern(parse_extern_def(stream)?))
        }
        _ => Err(Error::Parse {
            line,
            col,
            message: "Expected eternal, anchor, craft, form, state, fusion, class, realm, import, bring, alias, impl, or extern".to_string(),
        }),
    }
}

fn parse_fn_def(stream: &mut TokenStream) -> Result<FnDef> {
    parse_fn_def_with_open(stream, false)
}

fn parse_fn_def_with_open(stream: &mut TokenStream, open: bool) -> Result<FnDef> {
    let name = expect_ident(stream)?;
    stream.expect("(")?;
    let params = parse_params(stream)?;
    stream.expect(")")?;
    let return_type = if stream.peek() == Some(&Token::Arrow) {
        stream.consume();
        Some(parse_type(stream)?)
    } else {
        None
    };
    stream.expect("{")?;
    let body = parse_block(stream)?;
    stream.expect("}")?;
    Ok(FnDef {
        name,
        params,
        return_type,
        body,
        open,
    })
}

fn parse_const_def(stream: &mut TokenStream) -> Result<ConstDef> {
    let name = expect_ident(stream)?;
    stream.expect(":")?;
    let ty = parse_type(stream)?;
    stream.expect("=")?;
    let init = parse_expr(stream)?;
    stream.expect(";")?;
    Ok(ConstDef { name, ty, init })
}

fn parse_static_def(stream: &mut TokenStream) -> Result<StaticDef> {
    let name = expect_ident(stream)?;
    stream.expect(":")?;
    let ty = parse_type(stream)?;
    let init = if stream.peek() == Some(&Token::Eq) {
        stream.consume();
        Some(parse_expr(stream)?)
    } else {
        None
    };
    stream.expect(";")?;
    Ok(StaticDef { name, ty, init })
}

fn parse_enum_def(stream: &mut TokenStream) -> Result<EnumDef> {
    let name = expect_ident(stream)?;
    stream.expect("{")?;
    let mut variants = Vec::new();
    while stream.peek() != Some(&Token::RBrace) {
        let vname = expect_ident(stream)?;
        let variant = if stream.peek() == Some(&Token::LParen) {
            stream.consume();
            let mut tys = Vec::new();
            while stream.peek() != Some(&Token::RParen) {
                tys.push(parse_type(stream)?);
                if stream.peek() == Some(&Token::Comma) {
                    stream.consume();
                }
            }
            stream.expect(")")?;
            EnumVariant::Tuple(vname, tys)
        } else {
            EnumVariant::Unit(vname)
        };
        variants.push(variant);
        if stream.peek() == Some(&Token::Comma) {
            stream.consume();
        }
    }
    stream.expect("}")?;
    Ok(EnumDef { name, variants })
}

fn parse_union_def(stream: &mut TokenStream) -> Result<UnionDef> {
    let name = expect_ident(stream)?;
    stream.expect("{")?;
    let mut fields = Vec::new();
    while stream.peek() != Some(&Token::RBrace) {
        let field_name = expect_ident(stream)?;
        stream.expect(":")?;
        let ty = parse_type(stream)?;
        fields.push(FieldDef {
            name: field_name,
            ty,
        });
        if stream.peek() == Some(&Token::Comma) {
            stream.consume();
        }
    }
    stream.expect("}")?;
    Ok(UnionDef { name, fields })
}

fn parse_struct_def(stream: &mut TokenStream) -> Result<StructDef> {
    let name = expect_ident(stream)?;
    stream.expect("{")?;
    let mut fields = Vec::new();
    while stream.peek() != Some(&Token::RBrace) {
        let field_name = expect_ident(stream)?;
        stream.expect(":")?;
        let ty = parse_type(stream)?;
        fields.push(FieldDef {
            name: field_name,
            ty,
        });
        if stream.peek() == Some(&Token::Comma) {
            stream.consume();
        }
    }
    stream.expect("}")?;
    Ok(StructDef { name, fields })
}

fn parse_class_def(stream: &mut TokenStream) -> Result<ClassDef> {
    let name = expect_ident(stream)?;
    let extends = if stream.peek() == Some(&Token::Extends) {
        stream.consume();
        Some(expect_ident(stream)?)
    } else {
        None
    };
    let implements = if stream.peek() == Some(&Token::Implements) {
        stream.consume();
        let mut ifaces = Vec::new();
        loop {
            ifaces.push(expect_ident(stream)?);
            if stream.peek() != Some(&Token::Comma) {
                break;
            }
            stream.consume();
        }
        ifaces
    } else {
        Vec::new()
    };
    stream.expect("{")?;

    let mut fields = Vec::new();
    let mut init = None;
    let mut methods = Vec::new();

    while stream.peek() != Some(&Token::RBrace) {
        if stream.peek() == Some(&Token::Init) {
            stream.consume();
            stream.expect("(")?;
            let params = parse_params(stream)?;
            stream.expect(")")?;
            stream.expect("{")?;
            let body = parse_block(stream)?;
            stream.expect("}")?;
            init = Some(InitDef { params, body });
        } else if stream.peek() == Some(&Token::Craft) {
            stream.consume();
            methods.push(parse_method_def(stream)?);
        } else if matches!(stream.peek(), Some(Token::Ident(_))) {
            let field_name = expect_ident(stream)?;
            stream.expect(":")?;
            let ty = parse_type(stream)?;
            fields.push(FieldDef {
                name: field_name,
                ty,
            });
            if stream.peek() == Some(&Token::Semicolon) {
                stream.consume();
            }
        } else {
            let (line, col) = stream.peek_pos().unwrap_or((1, 1));
            return Err(Error::Parse {
                line,
                col,
                message: "Expected field, init, or method in class".to_string(),
            });
        }
    }
    stream.expect("}")?;

    Ok(ClassDef {
        name,
        extends,
        implements,
        fields,
        init,
        methods,
    })
}

fn parse_method_def(stream: &mut TokenStream) -> Result<MethodDef> {
    let name = expect_ident(stream)?;
    stream.expect("(")?;
    let params = parse_params(stream)?;
    stream.expect(")")?;
    let return_type = if stream.peek() == Some(&Token::Arrow) {
        stream.consume();
        Some(parse_type(stream)?)
    } else {
        None
    };
    stream.expect("{")?;
    let body = parse_block(stream)?;
    stream.expect("}")?;
    Ok(MethodDef {
        name,
        params,
        return_type,
        body,
        is_virtual: false,
    })
}

fn parse_mod_def(stream: &mut TokenStream) -> Result<ModDef> {
    let name = expect_ident(stream)?;
    stream.expect("{")?;
    let mut items = Vec::new();
    while stream.peek() != Some(&Token::RBrace) {
        items.push(parse_top_level(stream)?);
    }
    stream.expect("}")?;
    Ok(ModDef { name, items })
}

fn parse_import(stream: &mut TokenStream) -> Result<Import> {
    let path_str = expect_str_or_ident(stream)?;
    let path: Vec<String> = path_str.split('.').map(String::from).collect();
    stream.expect(";")?;
    Ok(Import { path })
}

fn parse_choose_pattern(stream: &mut TokenStream) -> Result<ChoosePattern> {
    let name = expect_ident(stream)?;
    if stream.peek() == Some(&Token::LParen) {
        stream.consume();
        let mut bindings = Vec::new();
        while stream.peek() != Some(&Token::RParen) {
            bindings.push(expect_ident(stream)?);
            if stream.peek() == Some(&Token::Comma) {
                stream.consume();
            }
        }
        stream.expect(")")?;
        Ok(ChoosePattern::TupleVariant(name, bindings))
    } else if name == "_" {
        Ok(ChoosePattern::Ident("_".to_string()))
    } else {
        Ok(ChoosePattern::UnitVariant(name))
    }
}

fn parse_impl_def(stream: &mut TokenStream) -> Result<ImplDef> {
    let struct_name = expect_ident(stream)?;
    stream.expect("{")?;
    let mut methods = Vec::new();
    while stream.peek() != Some(&Token::RBrace) {
        if stream.peek() == Some(&Token::Craft) {
            stream.consume();
            methods.push(parse_method_def(stream)?);
        } else {
            let (line, col) = stream.peek_pos().unwrap_or((1, 1));
            return Err(Error::Parse { line, col, message: "Expected craft in impl block".to_string() });
        }
    }
    stream.expect("}")?;
    Ok(ImplDef { struct_name, methods })
}

fn parse_extern_def(stream: &mut TokenStream) -> Result<ExternDef> {
    if stream.peek() != Some(&Token::Craft) {
        let (line, col) = stream.peek_pos().unwrap_or((1, 1));
        return Err(Error::Parse { line, col, message: "Expected craft after extern".to_string() });
    }
    stream.consume();
    let name = expect_ident(stream)?;
    stream.expect("(")?;
    let params = parse_params(stream)?;
    stream.expect(")")?;
    let return_type = if stream.peek() == Some(&Token::Arrow) {
        stream.consume();
        Some(parse_type(stream)?)
    } else {
        None
    };
    stream.expect(";")?;
    Ok(ExternDef {
        name,
        params,
        return_type,
    })
}

fn parse_alias_def(stream: &mut TokenStream) -> Result<AliasDef> {
    let name = expect_ident(stream)?;
    stream.expect("=")?;
    let ty = parse_type(stream)?;
    stream.expect(";")?;
    Ok(AliasDef { name, ty })
}

fn parse_params(stream: &mut TokenStream) -> Result<Vec<Param>> {
    let mut params = Vec::new();
    while stream.peek() != Some(&Token::RParen) {
        let name = expect_ident(stream)?;
        stream.expect(":")?;
        let ty = parse_type(stream)?;
        let default = if stream.peek() == Some(&Token::Eq) {
            stream.consume();
            Some(parse_expr(stream)?)
        } else {
            None
        };
        params.push(Param { name, ty, default });
        if stream.peek() == Some(&Token::Comma) {
            stream.consume();
        }
    }
    Ok(params)
}

fn parse_type(stream: &mut TokenStream) -> Result<Type> {
    let (line, col) = stream.peek_pos().unwrap_or((1, 1));
    if stream.peek() == Some(&Token::Link) {
        stream.consume();
        let inner = parse_type(stream)?;
        return Ok(Type::Ptr(Box::new(inner)));
    }
    if stream.peek() == Some(&Token::LParen) {
        stream.consume();
        let first = parse_type(stream)?;
        if stream.peek() == Some(&Token::Comma) {
            let mut elems = vec![first];
            while stream.peek() == Some(&Token::Comma) {
                stream.consume();
                elems.push(parse_type(stream)?);
            }
            stream.expect(")")?;
            return Ok(Type::Tuple(elems));
        }
        stream.expect(")")?;
        return Ok(first);
    }
    match stream.consume() {
        Some((Token::Ident(s), _, _)) => {
            Ok(match s.as_str() {
                "int" => Type::Int,
                "float" => Type::Float,
                "bool" => Type::Bool,
                "str" => Type::Str,
                "void" => Type::Void,
                "u8" => Type::U8,
                "u16" => Type::U16,
                "u32" => Type::U32,
                "u64" => Type::U64,
                "i8" => Type::I8,
                "i16" => Type::I16,
                "i32" => Type::I32,
                "i64" => Type::I64,
                "usize" => Type::Usize,
                "f32" => Type::F32,
                "f64" => Type::F64,
                _ => Type::Named(s),
            })
        }
        Some((Token::LBracket, _, _)) => {
            let inner = parse_type(stream)?;
            if stream.peek() == Some(&Token::Semicolon) {
                stream.consume();
                let n = expect_int_literal(stream)?;
                stream.expect("]")?;
                Ok(Type::ArraySized(Box::new(inner), n as u32))
            } else {
                stream.expect("]")?;
                Ok(Type::Array(Box::new(inner)))
            }
        }
        _ => Err(Error::Parse {
            line,
            col,
            message: "Expected type".to_string(),
        }),
    }
}

fn parse_hazard_block(stream: &mut TokenStream) -> Result<Vec<Stmt>> {
    let mut stmts = Vec::new();
    while stream.peek() != Some(&Token::RBrace) {
        if stream.peek() == Some(&Token::Machine) {
            stream.consume();
            stream.expect("{")?;
            let mut instructions = Vec::new();
            while stream.peek() != Some(&Token::RBrace) {
                if let Some((Token::Str(s), _, _)) = stream.consume() {
                    instructions.push(s);
                } else {
                    let (line, col) = stream.peek_pos().unwrap_or((1, 1));
                    return Err(Error::Parse {
                        line,
                        col,
                        message: "Expected string in machine block".to_string(),
                    });
                }
            }
            stream.expect("}")?;
            stmts.push(Stmt::InlineAsm { instructions });
        } else {
            stmts.push(parse_stmt(stream)?);
        }
    }
    Ok(stmts)
}

fn parse_block(stream: &mut TokenStream) -> Result<Vec<Stmt>> {
    let mut stmts = Vec::new();
    while stream.peek() != Some(&Token::RBrace) {
        stmts.push(parse_stmt(stream)?);
    }
    Ok(stmts)
}

fn parse_stmt(stream: &mut TokenStream) -> Result<Stmt> {
    match stream.peek() {
        Some(Token::Check) => {
            stream.consume();
            stream.expect("(")?;
            let cond = parse_expr(stream)?;
            stream.expect(")")?;
            stream.expect("{")?;
            let then_body = parse_block(stream)?;
            stream.expect("}")?;
            let else_body = if stream.peek() == Some(&Token::Otherwise) {
                stream.consume();
                stream.expect("{")?;
                let body = parse_block(stream)?;
                stream.expect("}")?;
                Some(body)
            } else {
                None
            };
            Ok(Stmt::If {
                cond,
                then_body,
                else_body,
            })
        }
        Some(Token::For) => {
            stream.consume();
            stream.expect("(")?;
            let init = if stream.peek() != Some(&Token::Semicolon) {
                Some(Box::new(parse_stmt(stream)?))
            } else {
                None
            };
            stream.expect(";")?;
            let cond = if stream.peek() != Some(&Token::Semicolon) {
                Some(parse_expr(stream)?)
            } else {
                None
            };
            stream.expect(";")?;
            let step = if stream.peek() != Some(&Token::RParen) {
                Some(Box::new(parse_stmt(stream)?))
            } else {
                None
            };
            stream.expect(")")?;
            stream.expect("{")?;
            let body = parse_block(stream)?;
            stream.expect("}")?;
            Ok(Stmt::For {
                init,
                cond,
                step,
                body,
            })
        }
        Some(Token::Loopwhile) => {
            stream.consume();
            stream.expect("(")?;
            let cond = parse_expr(stream)?;
            stream.expect(")")?;
            stream.expect("{")?;
            let body = parse_block(stream)?;
            stream.expect("}")?;
            Ok(Stmt::While { cond, body })
        }
        Some(Token::Foreach) => {
            stream.consume();
            let var = expect_ident(stream)?;
            stream.expect("in")?;
            let iter = parse_expr(stream)?;
            stream.expect("{")?;
            let body = parse_block(stream)?;
            stream.expect("}")?;
            Ok(Stmt::Foreach {
                var,
                iter: Box::new(iter),
                body,
            })
        }
        Some(Token::Stop) => {
            stream.consume();
            stream.expect(";")?;
            Ok(Stmt::Break)
        }
        Some(Token::Skip) => {
            stream.consume();
            stream.expect(";")?;
            Ok(Stmt::Continue)
        }
        Some(Token::Hazard) => {
            stream.consume();
            stream.expect("{")?;
            let body = parse_hazard_block(stream)?;
            stream.expect("}")?;
            Ok(Stmt::Hazard { body })
        }
        Some(Token::Send) => {
            stream.consume();
            let expr = if stream.peek() != Some(&Token::Semicolon) {
                Some(parse_expr(stream)?)
            } else {
                None
            };
            stream.expect(";")?;
            Ok(Stmt::Return(expr))
        }
        Some(Token::Try) => {
            stream.consume();
            stream.expect("{")?;
            let try_body = parse_block(stream)?;
            stream.expect("}")?;
            stream.expect("catch")?;
            stream.expect("(")?;
            let catch_param = expect_ident(stream).ok();
            stream.expect(")")?;
            stream.expect("{")?;
            let catch_body = parse_block(stream)?;
            stream.expect("}")?;
            Ok(Stmt::TryCatch {
                try_body,
                catch_param,
                catch_body,
            })
        }
        Some(Token::Defer) => {
            stream.consume();
            stream.expect("{")?;
            let body = parse_block(stream)?;
            stream.expect("}")?;
            Ok(Stmt::Defer { body })
        }
        Some(Token::Choose) => {
            stream.consume();
            stream.expect("(")?;
            let expr = parse_expr(stream)?;
            stream.expect(")")?;
            stream.expect("{")?;
            let mut arms = Vec::new();
            while stream.peek() != Some(&Token::RBrace) {
                let pattern = parse_choose_pattern(stream)?;
                if stream.peek() != Some(&Token::FatArrow) {
                    let (line, col) = stream.peek_pos().unwrap_or((1, 1));
                    return Err(Error::Parse { line, col, message: "Expected =>".to_string() });
                }
                stream.consume();
                let arm_body = if stream.peek() == Some(&Token::LBrace) {
                    stream.consume();
                    let b = parse_block(stream)?;
                    stream.expect("}")?;
                    b
                } else {
                    vec![parse_stmt(stream)?]
                };
                if stream.peek() == Some(&Token::Comma) {
                    stream.consume();
                }
                arms.push(ChooseArm { pattern, body: arm_body });
            }
            stream.expect("}")?;
            Ok(Stmt::Choose { expr: Box::new(expr), arms })
        }
        Some(Token::Make) => {
            stream.consume();
            let mutable = stream.peek() == Some(&Token::Shift);
            if mutable {
                stream.consume();
            }
            if stream.peek() == Some(&Token::LParen) {
                stream.consume();
                let mut names = Vec::new();
                loop {
                    names.push(expect_ident(stream)?);
                    if stream.peek() == Some(&Token::Comma) {
                        stream.consume();
                    } else {
                        break;
                    }
                }
                stream.expect(")")?;
                stream.expect("=")?;
                let init = parse_expr(stream)?;
                stream.expect(";")?;
                Ok(Stmt::VarDeclTuple { names, init, mutable })
            } else {
                let name = expect_ident(stream)?;
                let ty = if stream.peek() == Some(&Token::Colon) {
                    stream.consume();
                    Some(parse_type(stream)?)
                } else {
                    None
                };
                stream.expect("=")?;
                let init = parse_expr(stream)?;
                stream.expect(";")?;
                Ok(Stmt::VarDecl {
                    name,
                    ty,
                    init,
                    mutable,
                })
            }
        }
        Some(Token::Ident(_)) | Some(Token::This) => {
            let expr = parse_postfix(stream)?;
            if stream.peek() == Some(&Token::Eq) {
                stream.consume();
                let value = parse_expr(stream)?;
                stream.expect(";")?;
                let target = expr_to_assign_target(expr)?;
                Ok(Stmt::Assign { target, value })
            } else {
                stream.expect(";")?;
                Ok(Stmt::ExprStmt(expr))
            }
        }
        _ => {
            let expr = parse_expr(stream)?;
            if stream.peek() == Some(&Token::Eq) {
                stream.consume();
                let value = parse_expr(stream)?;
                stream.expect(";")?;
                let target = expr_to_assign_target(expr)?;
                Ok(Stmt::Assign { target, value })
            } else {
                stream.expect(";")?;
                Ok(Stmt::ExprStmt(expr))
            }
        }
    }
}

fn parse_expr(stream: &mut TokenStream) -> Result<Expr> {
    parse_or(stream)
}

fn parse_or(stream: &mut TokenStream) -> Result<Expr> {
    let mut left = parse_and(stream)?;
    while stream.peek() == Some(&Token::OrOr) {
        stream.consume();
        let right = parse_and(stream)?;
        left = Expr::Binary {
            op: BinOp::Or,
            left: Box::new(left),
            right: Box::new(right),
        };
    }
    Ok(left)
}

fn parse_and(stream: &mut TokenStream) -> Result<Expr> {
    let mut left = parse_equality(stream)?;
    while stream.peek() == Some(&Token::AndAnd) {
        stream.consume();
        let right = parse_equality(stream)?;
        left = Expr::Binary {
            op: BinOp::And,
            left: Box::new(left),
            right: Box::new(right),
        };
    }
    Ok(left)
}

fn parse_equality(stream: &mut TokenStream) -> Result<Expr> {
    let mut left = parse_comparison(stream)?;
    loop {
        let op = match stream.peek() {
            Some(Token::EqEq) => BinOp::Eq,
            Some(Token::Ne) => BinOp::Ne,
            _ => break,
        };
        stream.consume();
        let right = parse_comparison(stream)?;
        left = Expr::Binary {
            op,
            left: Box::new(left),
            right: Box::new(right),
        };
    }
    Ok(left)
}

fn parse_comparison(stream: &mut TokenStream) -> Result<Expr> {
    let mut left = parse_term(stream)?;
    loop {
        let op = match stream.peek() {
            Some(Token::Lt) => BinOp::Lt,
            Some(Token::Le) => BinOp::Le,
            Some(Token::Gt) => BinOp::Gt,
            Some(Token::Ge) => BinOp::Ge,
            _ => break,
        };
        stream.consume();
        let right = parse_term(stream)?;
        left = Expr::Binary {
            op,
            left: Box::new(left),
            right: Box::new(right),
        };
    }
    Ok(left)
}

fn parse_term(stream: &mut TokenStream) -> Result<Expr> {
    let mut left = parse_factor(stream)?;
    loop {
        let op = match stream.peek() {
            Some(Token::Plus) => BinOp::Add,
            Some(Token::Minus) => BinOp::Sub,
            _ => break,
        };
        stream.consume();
        let right = parse_factor(stream)?;
        left = Expr::Binary {
            op,
            left: Box::new(left),
            right: Box::new(right),
        };
    }
    Ok(left)
}

fn parse_factor(stream: &mut TokenStream) -> Result<Expr> {
    let mut left = parse_unary(stream)?;
    loop {
        let op = match stream.peek() {
            Some(Token::Star) => BinOp::Mul,
            Some(Token::Slash) => BinOp::Div,
            Some(Token::Percent) => BinOp::Mod,
            _ => break,
        };
        stream.consume();
        let right = parse_unary(stream)?;
        left = Expr::Binary {
            op,
            left: Box::new(left),
            right: Box::new(right),
        };
    }
    Ok(left)
}

fn parse_unary(stream: &mut TokenStream) -> Result<Expr> {
    match stream.peek() {
        Some(Token::Mark) => {
            stream.consume();
            let operand = parse_unary(stream)?;
            Ok(Expr::AddrOf(Box::new(operand)))
        }
        Some(Token::Reach) => {
            stream.consume();
            let operand = parse_unary(stream)?;
            Ok(Expr::Deref(Box::new(operand)))
        }
        Some(Token::Minus) => {
            stream.consume();
            let operand = parse_unary(stream)?;
            Ok(Expr::Unary {
                op: UnOp::Neg,
                operand: Box::new(operand),
            })
        }
        Some(Token::Bang) => {
            stream.consume();
            let operand = parse_unary(stream)?;
            Ok(Expr::Unary {
                op: UnOp::Not,
                operand: Box::new(operand),
            })
        }
        _ => parse_postfix(stream),
    }
}

fn parse_postfix(stream: &mut TokenStream) -> Result<Expr> {
    let mut expr = parse_primary(stream)?;
    loop {
        match stream.peek() {
            Some(Token::LParen) => {
                stream.consume();
                let mut args = Vec::new();
                while stream.peek() != Some(&Token::RParen) {
                    args.push(parse_expr(stream)?);
                    if stream.peek() == Some(&Token::Comma) {
                        stream.consume();
                    }
                }
                stream.expect(")")?;
                expr = Expr::Call {
                    callee: Box::new(expr),
                    args,
                };
            }
            Some(Token::LBracket) => {
                stream.consume();
                if stream.peek() == Some(&Token::DotDot) {
                    stream.consume();
                    let end = if stream.peek() != Some(&Token::RBracket) {
                        Some(Box::new(parse_expr(stream)?))
                    } else {
                        None
                    };
                    stream.expect("]")?;
                    expr = Expr::Slice {
                        base: Box::new(expr),
                        start: None,
                        end,
                    };
                } else {
                    let start = parse_expr(stream)?;
                    if stream.peek() == Some(&Token::DotDot) {
                        stream.consume();
                        let end = if stream.peek() != Some(&Token::RBracket) {
                            Some(Box::new(parse_expr(stream)?))
                        } else {
                            None
                        };
                        stream.expect("]")?;
                        expr = Expr::Slice {
                            base: Box::new(expr),
                            start: Some(Box::new(start)),
                            end,
                        };
                    } else {
                        stream.expect("]")?;
                        expr = Expr::Index {
                            base: Box::new(expr),
                            index: Box::new(start),
                        };
                    }
                }
            }
            Some(Token::DotDot) => {
                stream.consume();
                let end = parse_expr(stream)?;
                expr = Expr::Range {
                    start: Box::new(expr),
                    end: Box::new(end),
                };
            }
            Some(Token::Dot) => {
                stream.consume();
                let field = expect_ident(stream)?;
                expr = Expr::Field {
                    base: Box::new(expr),
                    field,
                };
            }
            Some(Token::Ident(s)) if s == "as" => {
                stream.consume();
                let target_ty = parse_type(stream)?;
                expr = Expr::Cast {
                    operand: Box::new(expr),
                    target_ty,
                };
            }
            _ => break,
        }
    }
    Ok(expr)
}

fn parse_primary(stream: &mut TokenStream) -> Result<Expr> {
    let (line, col) = stream.peek_pos().unwrap_or((1, 1));

    match stream.consume() {
        Some((Token::Int(n), _, _)) => Ok(Expr::Literal(Literal::Int(n))),
        Some((Token::Float(n), _, _)) => Ok(Expr::Literal(Literal::Float(n))),
        Some((Token::Bool(b), _, _)) => Ok(Expr::Literal(Literal::Bool(b))),
        Some((Token::Str(s), _, _)) => Ok(Expr::Literal(Literal::Str(s))),
        Some((Token::Ident(s), _, _)) => Ok(Expr::Ident(s)),
        Some((Token::This, _, _)) => Ok(Expr::Ident("this".to_string())),
        Some((Token::New, _, _)) => {
            let class = expect_ident(stream)?;
            stream.expect("(")?;
            let mut args = Vec::new();
            while stream.peek() != Some(&Token::RParen) {
                args.push(parse_expr(stream)?);
                if stream.peek() == Some(&Token::Comma) {
                    stream.consume();
                }
            }
            stream.expect(")")?;
            Ok(Expr::New { class, args })
        }
        Some((Token::LParen, _, _)) => {
            let first = parse_expr(stream)?;
            if stream.peek() == Some(&Token::Comma) {
                let mut elems = vec![first];
                while stream.peek() == Some(&Token::Comma) {
                    stream.consume();
                    elems.push(parse_expr(stream)?);
                }
                stream.expect(")")?;
                Ok(Expr::Tuple(elems))
            } else {
                stream.expect(")")?;
                Ok(first)
            }
        }
        Some((Token::LBrace, _, _)) => {
            let mut elems = Vec::new();
            while stream.peek() != Some(&Token::RBrace) {
                elems.push(parse_expr(stream)?);
                if stream.peek() == Some(&Token::Comma) {
                    stream.consume();
                }
            }
            stream.expect("}")?;
            Ok(Expr::ArrayInit(elems))
        }
        _ => Err(Error::Parse {
            line,
            col,
            message: "Expected expression".to_string(),
        }),
    }
}

fn expr_to_assign_target(expr: Expr) -> Result<AssignTarget> {
    match expr {
        Expr::Ident(name) => Ok(AssignTarget::Ident(name)),
        Expr::Field { base, field } => Ok(AssignTarget::Field {
            base,
            field,
        }),
        Expr::Index { base, index } => Ok(AssignTarget::Index {
            base,
            index,
        }),
        Expr::Deref(operand) => Ok(AssignTarget::Deref(operand)),
        _ => Err(Error::Parse {
            line: 1,
            col: 1,
            message: "Invalid assignment target".to_string(),
        }),
    }
}

fn expect_ident(stream: &mut TokenStream) -> Result<String> {
    let (line, col) = stream.peek_pos().unwrap_or((1, 1));
    match stream.consume() {
        Some((Token::Ident(s), _, _)) => Ok(s),
        _ => Err(Error::Parse {
            line,
            col,
            message: "Expected identifier".to_string(),
        }),
    }
}

fn expect_int_literal(stream: &mut TokenStream) -> Result<i64> {
    let (line, col) = stream.peek_pos().unwrap_or((1, 1));
    match stream.consume() {
        Some((Token::Int(n), _, _)) => Ok(n),
        _ => Err(Error::Parse {
            line,
            col,
            message: "Expected integer literal for array size".to_string(),
        }),
    }
}

fn expect_str_or_ident(stream: &mut TokenStream) -> Result<String> {
    let (line, col) = stream.peek_pos().unwrap_or((1, 1));
    match stream.consume() {
        Some((Token::Ident(s), _, _)) => Ok(s),
        Some((Token::Str(s), _, _)) => Ok(s),
        _ => Err(Error::Parse {
            line,
            col,
            message: "Expected string or identifier".to_string(),
        }),
    }
}
