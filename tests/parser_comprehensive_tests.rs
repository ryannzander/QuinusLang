use qpp::ast::*;
use qpp::parse;

fn parse_ok(src: &str) -> Program {
    parse(src).unwrap_or_else(|e| panic!("Parse failed: {}", e))
}

fn parse_fn_body(body_src: &str) -> Program {
    parse_ok(&format!("craft main() -> void {{ {} }}", body_src))
}

// ── Variable declarations ──

#[test]
fn parse_immutable_var() {
    let p = parse_fn_body("make x: i32 = 42; send;");
    let f = match &p.items[0] {
        TopLevelItem::Fn(f) => f,
        _ => panic!("Expected function"),
    };
    match &f.body[0] {
        Stmt::VarDecl {
            name,
            mutable,
            ty,
            init,
        } => {
            assert_eq!(name, "x");
            assert!(!mutable);
            assert!(matches!(ty, Some(Type::I32)));
            assert!(matches!(init, Expr::Literal(Literal::Int(42))));
        }
        _ => panic!("Expected VarDecl"),
    }
}

#[test]
fn parse_mutable_var() {
    let p = parse_fn_body("make shift x: i32 = 0; send;");
    let f = match &p.items[0] {
        TopLevelItem::Fn(f) => f,
        _ => panic!("Expected function"),
    };
    match &f.body[0] {
        Stmt::VarDecl { mutable, .. } => assert!(mutable),
        _ => panic!("Expected VarDecl"),
    }
}

#[test]
fn parse_var_with_expression() {
    let p = parse_fn_body("make x: i32 = 1 + 2; send;");
    let f = match &p.items[0] {
        TopLevelItem::Fn(f) => f,
        _ => panic!("Expected function"),
    };
    match &f.body[0] {
        Stmt::VarDecl { init, .. } => {
            assert!(matches!(init, Expr::Binary { op: BinOp::Add, .. }));
        }
        _ => panic!("Expected VarDecl"),
    }
}

// ── Types ──

#[test]
fn parse_all_primitive_types() {
    let types = [
        ("i8", Type::I8),
        ("i16", Type::I16),
        ("i32", Type::I32),
        ("i64", Type::I64),
        ("u8", Type::U8),
        ("u16", Type::U16),
        ("u32", Type::U32),
        ("u64", Type::U64),
        ("usize", Type::Usize),
        ("f32", Type::F32),
        ("f64", Type::F64),
        ("int", Type::Int),
        ("float", Type::Float),
        ("bool", Type::Bool),
        ("str", Type::Str),
    ];
    for (name, expected) in types {
        let p = parse_fn_body(&format!("make x: {} = 0; send;", name));
        let f = match &p.items[0] {
            TopLevelItem::Fn(f) => f,
            _ => panic!("Expected function for type {}", name),
        };
        match &f.body[0] {
            Stmt::VarDecl { ty: Some(t), .. } => {
                assert_eq!(t, &expected, "Type mismatch for {}", name);
            }
            _ => panic!("Expected VarDecl for type {}", name),
        }
    }
}

// ── Functions ──

#[test]
fn parse_function_with_return() {
    let p = parse_ok("craft add(a: i32, b: i32) -> i32 { send a + b; }");
    assert_eq!(p.items.len(), 1);
    if let TopLevelItem::Fn(f) = &p.items[0] {
        assert_eq!(f.name, "add");
        assert_eq!(f.params.len(), 2);
        assert!(matches!(f.return_type, Some(Type::I32)));
    } else {
        panic!("Expected function");
    }
}

#[test]
fn parse_void_function() {
    let p = parse_ok("craft noop() -> void { send; }");
    if let TopLevelItem::Fn(f) = &p.items[0] {
        assert!(matches!(f.return_type, Some(Type::Void)));
    } else {
        panic!("Expected function");
    }
}

#[test]
fn parse_multiple_functions() {
    let p = parse_ok(
        r#"
craft foo() -> void { send; }
craft bar() -> void { send; }
craft main() -> void { send; }
"#,
    );
    assert_eq!(p.items.len(), 3);
}

// ── Control flow ──

#[test]
fn parse_check_otherwise() {
    let p = parse_fn_body("check (1 > 0) { print(1); } otherwise { print(0); } send;");
    let f = match &p.items[0] {
        TopLevelItem::Fn(f) => f,
        _ => panic!("Expected function"),
    };
    match &f.body[0] {
        Stmt::If {
            then_body,
            else_body,
            ..
        } => {
            assert!(!then_body.is_empty());
            assert!(else_body.is_some());
        }
        _ => panic!("Expected If"),
    }
}

#[test]
fn parse_check_without_otherwise() {
    let p = parse_fn_body("check (x > 0) { print(x); } send;");
    let f = match &p.items[0] {
        TopLevelItem::Fn(f) => f,
        _ => panic!("Expected function"),
    };
    match &f.body[0] {
        Stmt::If { else_body, .. } => assert!(else_body.is_none()),
        _ => panic!("Expected If"),
    }
}

#[test]
fn parse_for_loop() {
    let p = parse_fn_body("for (make shift i: i32 = 0; i < 10; i = i + 1) { print(i); } send;");
    let f = match &p.items[0] {
        TopLevelItem::Fn(f) => f,
        _ => panic!("Expected function"),
    };
    assert!(matches!(f.body[0], Stmt::For { .. }));
}

#[test]
fn parse_while_loop() {
    let p = parse_fn_body("loopwhile (x > 0) { x = x - 1; } send;");
    let f = match &p.items[0] {
        TopLevelItem::Fn(f) => f,
        _ => panic!("Expected function"),
    };
    assert!(matches!(f.body[0], Stmt::While { .. }));
}

#[test]
fn parse_foreach() {
    let p = parse_fn_body("foreach x in arr { print(x); } send;");
    let f = match &p.items[0] {
        TopLevelItem::Fn(f) => f,
        _ => panic!("Expected function"),
    };
    assert!(matches!(f.body[0], Stmt::Foreach { .. }));
}

#[test]
fn parse_break_continue() {
    let p = parse_fn_body("loopwhile (true) { stop; } send;");
    let f = match &p.items[0] {
        TopLevelItem::Fn(f) => f,
        _ => panic!("Expected function"),
    };
    if let Stmt::While { body, .. } = &f.body[0] {
        assert!(matches!(body[0], Stmt::Break));
    }

    let p2 = parse_fn_body("loopwhile (true) { skip; } send;");
    let f2 = match &p2.items[0] {
        TopLevelItem::Fn(f) => f,
        _ => panic!("Expected function"),
    };
    if let Stmt::While { body, .. } = &f2.body[0] {
        assert!(matches!(body[0], Stmt::Continue));
    }
}

// ── Expressions ──

#[test]
fn parse_binary_ops() {
    let ops = [
        ("+", BinOp::Add),
        ("-", BinOp::Sub),
        ("*", BinOp::Mul),
        ("/", BinOp::Div),
        ("%", BinOp::Mod),
        ("==", BinOp::Eq),
        ("!=", BinOp::Ne),
        ("<", BinOp::Lt),
        ("<=", BinOp::Le),
        (">", BinOp::Gt),
        (">=", BinOp::Ge),
    ];
    for (sym, expected_op) in ops {
        let p = parse_fn_body(&format!("make x: i32 = 1 {} 2; send;", sym));
        let f = match &p.items[0] {
            TopLevelItem::Fn(f) => f,
            _ => panic!("Expected function for op {}", sym),
        };
        match &f.body[0] {
            Stmt::VarDecl { init, .. } => match init {
                Expr::Binary { op, .. } => assert_eq!(op, &expected_op, "Op mismatch for {}", sym),
                _ => panic!("Expected Binary for op {}", sym),
            },
            _ => panic!("Expected VarDecl for op {}", sym),
        }
    }
}

#[test]
fn parse_unary_neg() {
    let p = parse_fn_body("make x: i32 = -5; send;");
    let f = match &p.items[0] {
        TopLevelItem::Fn(f) => f,
        _ => panic!("Expected function"),
    };
    match &f.body[0] {
        Stmt::VarDecl { init, .. } => {
            assert!(matches!(init, Expr::Literal(Literal::Int(-5))));
        }
        _ => panic!("Expected VarDecl"),
    }
}

#[test]
fn parse_function_call() {
    let p = parse_fn_body("print(42); send;");
    let f = match &p.items[0] {
        TopLevelItem::Fn(f) => f,
        _ => panic!("Expected function"),
    };
    match &f.body[0] {
        Stmt::ExprStmt(Expr::Call { callee, args }) => {
            assert!(matches!(callee.as_ref(), Expr::Ident(n) if n == "print"));
            assert_eq!(args.len(), 1);
        }
        _ => panic!("Expected ExprStmt(Call)"),
    }
}

#[test]
fn parse_nested_function_call() {
    let p = parse_fn_body("print(add(1, 2)); send;");
    let f = match &p.items[0] {
        TopLevelItem::Fn(f) => f,
        _ => panic!("Expected function"),
    };
    match &f.body[0] {
        Stmt::ExprStmt(Expr::Call { args, .. }) => {
            assert!(matches!(&args[0], Expr::Call { .. }));
        }
        _ => panic!("Expected ExprStmt(Call)"),
    }
}

// ── Structs ──

#[test]
fn parse_struct_def() {
    let p = parse_ok("form Point { x: i32, y: i32, }");
    if let TopLevelItem::Struct(s) = &p.items[0] {
        assert_eq!(s.name, "Point");
        assert_eq!(s.fields.len(), 2);
        assert_eq!(s.fields[0].name, "x");
        assert_eq!(s.fields[1].name, "y");
    } else {
        panic!("Expected Struct");
    }
}

// ── Enums ──

#[test]
fn parse_enum_def() {
    let p = parse_ok("state Color { Red, Green, Blue, }");
    if let TopLevelItem::Enum(e) = &p.items[0] {
        assert_eq!(e.name, "Color");
        assert_eq!(e.variants.len(), 3);
    } else {
        panic!("Expected Enum");
    }
}

#[test]
fn parse_enum_with_tuple_variants() {
    let p = parse_ok("state Shape { Circle(f64), Rect(f64, f64), }");
    if let TopLevelItem::Enum(e) = &p.items[0] {
        assert_eq!(e.name, "Shape");
        assert!(matches!(&e.variants[0], EnumVariant::Tuple(n, _) if n == "Circle"));
        assert!(matches!(&e.variants[1], EnumVariant::Tuple(n, _) if n == "Rect"));
    } else {
        panic!("Expected Enum");
    }
}

// ── Constants ──

#[test]
fn parse_constant() {
    let p = parse_ok("eternal PI: f64 = 3.14; craft main() -> void { send; }");
    if let TopLevelItem::Const(c) = &p.items[0] {
        assert_eq!(c.name, "PI");
        assert!(matches!(c.ty, Type::F64));
    } else {
        panic!("Expected Const");
    }
}

// ── Extern ──

#[test]
fn parse_extern() {
    let p = parse_ok("extern craft puts(s: str) -> i32; craft main() -> void { send; }");
    if let TopLevelItem::Extern(e) = &p.items[0] {
        assert_eq!(e.name, "puts");
        assert_eq!(e.params.len(), 1);
        assert!(matches!(e.return_type, Some(Type::I32)));
    } else {
        panic!("Expected Extern");
    }
}

// ── Alias ──

#[test]
fn parse_alias() {
    let p = parse_ok("alias Int = i32; craft main() -> void { send; }");
    if let TopLevelItem::Alias(a) = &p.items[0] {
        assert_eq!(a.name, "Int");
        assert!(matches!(a.ty, Type::I32));
    } else {
        panic!("Expected Alias");
    }
}

// ── Import/Bring ──

#[test]
fn parse_bring() {
    let p = parse_ok(r#"bring "io"; craft main() -> void { send; }"#);
    assert!(matches!(&p.items[0], TopLevelItem::Import(i) if i.path == vec!["io"]));
}

// ── Defer ──

#[test]
fn parse_defer() {
    let p = parse_fn_body("defer { print(1); } send;");
    let f = match &p.items[0] {
        TopLevelItem::Fn(f) => f,
        _ => panic!("Expected function"),
    };
    assert!(matches!(&f.body[0], Stmt::Defer { .. }));
}

// ── Hazard ──

#[test]
fn parse_hazard() {
    let p = parse_fn_body("hazard { print(1); } send;");
    let f = match &p.items[0] {
        TopLevelItem::Fn(f) => f,
        _ => panic!("Expected function"),
    };
    assert!(matches!(&f.body[0], Stmt::Hazard { .. }));
}

// ── Array init ──

#[test]
fn parse_array_init() {
    let p = parse_fn_body("make shift arr: [i32; 3] = { 1, 2, 3 }; send;");
    let f = match &p.items[0] {
        TopLevelItem::Fn(f) => f,
        _ => panic!("Expected function"),
    };
    match &f.body[0] {
        Stmt::VarDecl { ty, init, .. } => {
            assert!(matches!(ty, Some(Type::ArraySized(_, 3))));
            assert!(matches!(init, Expr::ArrayInit(v) if v.len() == 3));
        }
        _ => panic!("Expected VarDecl"),
    }
}

// ── Assignment ──

#[test]
fn parse_assignment() {
    let p = parse_fn_body("make shift x: i32 = 0; x = 42; send;");
    let f = match &p.items[0] {
        TopLevelItem::Fn(f) => f,
        _ => panic!("Expected function"),
    };
    match &f.body[1] {
        Stmt::Assign {
            target: AssignTarget::Ident(name),
            ..
        } => assert_eq!(name, "x"),
        _ => panic!("Expected Assign"),
    }
}

// ── Cast ──

#[test]
fn parse_cast() {
    let p = parse_fn_body("make x: f64 = 42 as f64; send;");
    let f = match &p.items[0] {
        TopLevelItem::Fn(f) => f,
        _ => panic!("Expected function"),
    };
    match &f.body[0] {
        Stmt::VarDecl { init, .. } => {
            assert!(matches!(
                init,
                Expr::Cast {
                    target_ty: Type::F64,
                    ..
                }
            ));
        }
        _ => panic!("Expected VarDecl"),
    }
}

// ── Error cases ──

#[test]
fn parse_error_unclosed_brace() {
    assert!(parse("craft main() -> void {").is_err());
}

#[test]
fn parse_error_missing_semicolon() {
    assert!(parse("craft main() -> void { make x: i32 = 1 }").is_err());
}

#[test]
fn parse_error_invalid_type() {
    assert!(parse("craft main() -> void { make x: badtype = 1; send; }").is_ok());
}

// ── Class ──

#[test]
fn parse_class_def() {
    let p = parse_ok(
        r#"
class Animal {
    name: str;
    init(name: str) {
        this.name = name;
    }
    craft speak() -> void {
        print(this.name);
    }
}
"#,
    );
    if let TopLevelItem::Class(c) = &p.items[0] {
        assert_eq!(c.name, "Animal");
        assert_eq!(c.fields.len(), 1);
        assert!(c.init.is_some());
        assert_eq!(c.methods.len(), 1);
    } else {
        panic!("Expected Class");
    }
}

// ── String interpolation ──

#[test]
fn parse_string_interpolation() {
    let p = parse_fn_body(r#"make name: str = "world"; print(`hello ${name}`); send;"#);
    assert_eq!(p.items.len(), 1);
}

// ── Tuple ──

#[test]
fn parse_tuple_expression() {
    let p = parse_fn_body("make x: (i32, i32) = (1, 2); send;");
    let f = match &p.items[0] {
        TopLevelItem::Fn(f) => f,
        _ => panic!("Expected function"),
    };
    match &f.body[0] {
        Stmt::VarDecl { ty, .. } => {
            assert!(matches!(ty, Some(Type::Tuple(_))));
        }
        _ => panic!("Expected VarDecl"),
    }
}

// ── Choose / pattern matching ──

#[test]
fn parse_choose() {
    let p = parse_ok(
        r#"
state Dir { Up, Down, }
craft main() -> void {
    make d: Dir = Up;
    choose (d) {
        Up => { print(1); }
        Down => { print(2); }
    }
    send;
}
"#,
    );
    assert_eq!(p.items.len(), 2);
}

// ── With statement ──

#[test]
fn parse_with_statement() {
    let p = parse_ok(
        r#"
craft acquire() -> i32 { send 1; }
craft main() -> void {
    with f = acquire() { print(f); }
    send;
}
"#,
    );
    let f = match &p.items[1] {
        TopLevelItem::Fn(f) => f,
        _ => panic!("Expected function"),
    };
    assert!(matches!(&f.body[0], Stmt::With { .. }));
}

// ── Try/catch ──

#[test]
fn parse_try_catch() {
    let p = parse_fn_body("try { print(1); } catch (e) { print(e); } send;");
    let f = match &p.items[0] {
        TopLevelItem::Fn(f) => f,
        _ => panic!("Expected function"),
    };
    assert!(matches!(&f.body[0], Stmt::TryCatch { .. }));
}

// ── Pointer type ──

#[test]
fn parse_pointer_type() {
    let p = parse_fn_body("make p: link i32 = 0; send;");
    let f = match &p.items[0] {
        TopLevelItem::Fn(f) => f,
        _ => panic!("Expected function"),
    };
    match &f.body[0] {
        Stmt::VarDecl {
            ty: Some(Type::Ptr(inner)),
            ..
        } => {
            assert!(matches!(inner.as_ref(), Type::I32));
        }
        _ => panic!("Expected VarDecl with Ptr type"),
    }
}

// ── Result type ──

#[test]
fn parse_result_type() {
    let p = parse_ok("craft maybe() -> Result(i32, str) { send Ok(1); }");
    if let TopLevelItem::Fn(f) = &p.items[0] {
        assert!(matches!(f.return_type, Some(Type::Result(_, _))));
    } else {
        panic!("Expected function");
    }
}

// ── Impl block ──

#[test]
fn parse_impl_block() {
    let p = parse_ok(
        r#"
form Vec2 { x: f64, y: f64, }
impl Vec2 {
    craft length() -> f64 {
        send 0.0;
    }
}
"#,
    );
    assert_eq!(p.items.len(), 2);
    assert!(matches!(&p.items[1], TopLevelItem::Impl(_)));
}
