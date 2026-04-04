use qpp::{analyze, codegen, parse};

fn codegen_ir(src: &str) -> String {
    let program = parse(src).unwrap_or_else(|e| panic!("Parse failed: {}", e));
    let annotated = analyze(&program).unwrap_or_else(|e| panic!("Analyze failed: {}", e));
    codegen::llvm::compile_to_ir_string(&annotated)
        .unwrap_or_else(|e| panic!("Codegen failed: {}", e))
}

fn codegen_ok(src: &str) {
    let _ = codegen_ir(src);
}

// ── Basic codegen ──

#[test]
fn cg_empty_main() {
    let ir = codegen_ir("craft main() -> void { send; }");
    assert!(ir.contains("_ql_main"));
    assert!(ir.contains("define"));
}

#[test]
fn cg_integer_literal() {
    let ir = codegen_ir(
        r#"
craft main() -> void {
    make x: i32 = 42;
    send;
}
"#,
    );
    assert!(ir.contains("42"));
}

#[test]
fn cg_float_literal() {
    let ir = codegen_ir(
        r#"
craft main() -> void {
    make x: f64 = 3.14;
    send;
}
"#,
    );
    assert!(ir.contains("3.14") || ir.contains("double"));
}

#[test]
fn cg_string_literal() {
    let ir = codegen_ir(
        r#"
craft main() -> void {
    make s: str = "hello";
    send;
}
"#,
    );
    assert!(ir.contains("hello"));
}

#[test]
fn cg_bool_literal() {
    let ir = codegen_ir(
        r#"
craft main() -> void {
    make b: bool = true;
    send;
}
"#,
    );
    assert!(ir.contains("i1") || ir.contains("true") || ir.contains("1"));
}

// ── Arithmetic ──

#[test]
fn cg_int_add() {
    let ir = codegen_ir(
        r#"
craft do_add(a: i32, b: i32) -> i32 { send a + b; }
craft main() -> void {
    make x: i32 = do_add(10, 20);
    send;
}
"#,
    );
    assert!(ir.contains("add"));
}

#[test]
fn cg_int_sub() {
    let ir = codegen_ir(
        r#"
craft do_sub(a: i32, b: i32) -> i32 { send a - b; }
craft main() -> void {
    make x: i32 = do_sub(20, 10);
    send;
}
"#,
    );
    assert!(ir.contains("sub"));
}

#[test]
fn cg_int_mul() {
    let ir = codegen_ir(
        r#"
craft do_mul(a: i32, b: i32) -> i32 { send a * b; }
craft main() -> void {
    make x: i32 = do_mul(3, 4);
    send;
}
"#,
    );
    assert!(ir.contains("mul"));
}

#[test]
fn cg_int_div() {
    let ir = codegen_ir(
        r#"
craft do_div(a: i32, b: i32) -> i32 { send a / b; }
craft main() -> void {
    make x: i32 = do_div(10, 2);
    send;
}
"#,
    );
    assert!(ir.contains("sdiv"));
}

#[test]
fn cg_int_mod() {
    let ir = codegen_ir(
        r#"
craft do_mod(a: i32, b: i32) -> i32 { send a % b; }
craft main() -> void {
    make x: i32 = do_mod(10, 3);
    send;
}
"#,
    );
    assert!(ir.contains("srem"));
}

#[test]
fn cg_float_arithmetic() {
    let ir = codegen_ir(
        r#"
craft fadd(a: f64, b: f64) -> f64 { send a + b; }
craft fsub(a: f64, b: f64) -> f64 { send a - b; }
craft fmul(a: f64, b: f64) -> f64 { send a * b; }
craft fdiv(a: f64, b: f64) -> f64 { send a / b; }
craft main() -> void {
    make a: f64 = fadd(1.5, 2.5);
    make b: f64 = fsub(5.0, 1.0);
    make c: f64 = fmul(2.0, 3.0);
    make d: f64 = fdiv(10.0, 3.0);
    send;
}
"#,
    );
    assert!(ir.contains("fadd"));
    assert!(ir.contains("fsub"));
    assert!(ir.contains("fmul"));
    assert!(ir.contains("fdiv"));
}

// ── Comparisons ──

#[test]
fn cg_comparisons() {
    let ir = codegen_ir(
        r#"
craft cmp(a: i32, b: i32) -> void {
    make v1: bool = a == b;
    make v2: bool = a != b;
    make v3: bool = a < b;
    make v4: bool = a > b;
    make v5: bool = a <= b;
    make v6: bool = a >= b;
    send;
}
craft main() -> void {
    cmp(1, 2);
    send;
}
"#,
    );
    assert!(ir.contains("icmp eq"));
    assert!(ir.contains("icmp ne"));
    assert!(ir.contains("icmp slt"));
    assert!(ir.contains("icmp sgt"));
    assert!(ir.contains("icmp sle"));
    assert!(ir.contains("icmp sge"));
}

// ── Unary ops ──

#[test]
fn cg_unary_neg() {
    let ir = codegen_ir(
        r#"
craft negate(x: i32) -> i32 { send -x; }
craft main() -> void {
    make r: i32 = negate(5);
    send;
}
"#,
    );
    assert!(ir.contains("sub") || ir.contains("neg"));
}

// ── Functions ──

#[test]
fn cg_function_call() {
    let ir = codegen_ir(
        r#"
craft add(a: i32, b: i32) -> i32 {
    send a + b;
}
craft main() -> void {
    make r: i32 = add(1, 2);
    send;
}
"#,
    );
    assert!(ir.contains("define"));
    assert!(ir.contains("call"));
    assert!(ir.contains("add"));
}

#[test]
fn cg_recursive_function() {
    codegen_ok(
        r#"
craft fib(n: i32) -> i32 {
    check (n <= 1) { send n; }
    send fib(n - 1) + fib(n - 2);
}
craft main() -> void {
    make r: i32 = fib(10);
    send;
}
"#,
    );
}

// ── Control flow ──

#[test]
fn cg_if_else() {
    let ir = codegen_ir(
        r#"
craft main() -> void {
    make x: i32 = 10;
    check (x > 5) {
        print(1);
    } otherwise {
        print(0);
    }
    send;
}
"#,
    );
    assert!(ir.contains("br"));
    assert!(ir.contains("then"));
    assert!(ir.contains("else"));
}

#[test]
fn cg_for_loop() {
    let ir = codegen_ir(
        r#"
craft main() -> void {
    for (make shift i: i32 = 0; i < 10; i = i + 1) {
        print(i);
    }
    send;
}
"#,
    );
    assert!(ir.contains("for_loop") || ir.contains("br"));
}

#[test]
fn cg_while_loop() {
    let ir = codegen_ir(
        r#"
craft main() -> void {
    make shift n: i32 = 10;
    loopwhile (n > 0) {
        n = n - 1;
    }
    send;
}
"#,
    );
    assert!(ir.contains("while_loop") || ir.contains("br"));
}

// ── Print builtins ──

#[test]
fn cg_print_int() {
    let ir = codegen_ir(
        r#"
craft main() -> void {
    print(42);
    send;
}
"#,
    );
    assert!(ir.contains("printf"));
    assert!(ir.contains("%ld"));
}

#[test]
fn cg_print_string() {
    let ir = codegen_ir(
        r#"
craft main() -> void {
    print("hello");
    send;
}
"#,
    );
    assert!(ir.contains("printf"));
    assert!(ir.contains("%s"));
}

#[test]
fn cg_print_float() {
    let ir = codegen_ir(
        r#"
craft main() -> void {
    print(3.14);
    send;
}
"#,
    );
    assert!(ir.contains("printf"));
    assert!(ir.contains("%f"));
}

#[test]
fn cg_writeln() {
    let ir = codegen_ir(
        r#"
craft main() -> void {
    writeln(42);
    send;
}
"#,
    );
    assert!(ir.contains("printf"));
}

// ── Variable operations ──

#[test]
fn cg_mutable_reassign() {
    codegen_ok(
        r#"
craft main() -> void {
    make shift x: i32 = 0;
    x = 42;
    print(x);
    send;
}
"#,
    );
}

#[test]
fn cg_multiple_variables() {
    codegen_ok(
        r#"
craft main() -> void {
    make a: i32 = 1;
    make b: i32 = 2;
    make c: i32 = a + b;
    print(c);
    send;
}
"#,
    );
}

// ── Digit separators in codegen ──

#[test]
fn cg_digit_separators() {
    let ir = codegen_ir(
        r#"
craft main() -> void {
    make x: i32 = 1_000_000;
    print(x);
    send;
}
"#,
    );
    assert!(ir.contains("1000000"));
}

// ── Complex programs ──

#[test]
fn cg_fibonacci_program() {
    codegen_ok(
        r#"
craft fib(n: i32) -> i32 {
    check (n <= 0) { send 0; }
    check (n == 1) { send 1; }
    send fib(n - 1) + fib(n - 2);
}
craft main() -> void {
    make r: i32 = fib(10);
    print(r);
    send;
}
"#,
    );
}

#[test]
fn cg_factorial_program() {
    codegen_ok(
        r#"
craft factorial(n: i32) -> i32 {
    check (n <= 1) { send 1; }
    send n * factorial(n - 1);
}
craft main() -> void {
    make r: i32 = factorial(5);
    print(r);
    send;
}
"#,
    );
}

#[test]
fn cg_nested_if() {
    codegen_ok(
        r#"
craft main() -> void {
    make x: i32 = 10;
    check (x > 0) {
        check (x > 5) {
            print(2);
        } otherwise {
            print(1);
        }
    } otherwise {
        print(0);
    }
    send;
}
"#,
    );
}

#[test]
fn cg_loop_with_conditional() {
    codegen_ok(
        r#"
craft main() -> void {
    for (make shift i: i32 = 0; i < 100; i = i + 1) {
        check (i % 2 == 0) {
            print(i);
        }
    }
    send;
}
"#,
    );
}

// ── Main wrapper ──

#[test]
fn cg_has_main_wrapper() {
    let ir = codegen_ir("craft main() -> void { send; }");
    assert!(ir.contains("define i32 @main()"));
    assert!(ir.contains("@_ql_main"));
}

// ── Builtin declare_builtins ──

#[test]
fn cg_declares_builtins() {
    let ir = codegen_ir("craft main() -> void { send; }");
    assert!(ir.contains("printf"));
    assert!(ir.contains("malloc"));
    assert!(ir.contains("strlen"));
}
