use qpp::preprocess;
use qpp::{analyze, codegen, parse, parse_with_imports};
use std::path::Path;

fn full_pipeline(src: &str) -> String {
    let program = parse(src).unwrap_or_else(|e| panic!("Parse failed: {}", e));
    let annotated = analyze(&program).unwrap_or_else(|e| panic!("Analyze failed: {}", e));
    codegen::llvm::compile_to_ir_string(&annotated)
        .unwrap_or_else(|e| panic!("Codegen failed: {}", e))
}

fn full_pipeline_with_defines(src: &str, defines: &[&str]) -> String {
    let defs: Vec<String> = defines.iter().map(|s| s.to_string()).collect();
    let flattened = preprocess::preprocess_with_defines(src, Path::new("."), &defs)
        .unwrap_or_else(|e| panic!("Preprocess failed: {}", e));
    full_pipeline(&flattened)
}

fn full_pipeline_with_imports(src: &str) -> String {
    let program = parse_with_imports(src, Path::new("."), &[])
        .unwrap_or_else(|e| panic!("Parse with imports failed: {}", e));
    let annotated = analyze(&program).unwrap_or_else(|e| panic!("Analyze failed: {}", e));
    codegen::llvm::compile_to_ir_string(&annotated)
        .unwrap_or_else(|e| panic!("Codegen failed: {}", e))
}

// ── Full pipeline: parse -> analyze -> codegen ──

#[test]
fn e2e_hello_world() {
    let ir = full_pipeline(
        r#"
craft main() -> void {
    print("Hello, World!");
    send;
}
"#,
    );
    assert!(ir.contains("Hello, World!"));
    assert!(ir.contains("printf"));
}

#[test]
fn e2e_arithmetic() {
    let ir = full_pipeline(
        r#"
craft main() -> void {
    make a: i32 = 10 + 20;
    make b: i32 = a * 3;
    make c: i32 = b - 5;
    make d: i32 = c / 2;
    make e: i32 = d % 7;
    print(e);
    send;
}
"#,
    );
    assert!(ir.contains("_ql_main"));
    assert!(ir.contains("printf"));
}

#[test]
fn e2e_function_composition() {
    let ir = full_pipeline(
        r#"
craft double(x: i32) -> i32 { send x * 2; }
craft inc(x: i32) -> i32 { send x + 1; }
craft main() -> void {
    make r: i32 = double(inc(double(5)));
    print(r);
    send;
}
"#,
    );
    assert!(ir.contains("call"));
    assert!(ir.contains("double"));
    assert!(ir.contains("inc"));
}

#[test]
fn e2e_fibonacci() {
    full_pipeline(
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
fn e2e_countdown() {
    full_pipeline(
        r#"
craft main() -> void {
    make shift n: i32 = 10;
    loopwhile (n > 0) {
        print(n);
        n = n - 1;
    }
    print(0);
    send;
}
"#,
    );
}

#[test]
fn e2e_for_loop_sum() {
    full_pipeline(
        r#"
craft main() -> void {
    make shift sum: i32 = 0;
    for (make shift i: i32 = 1; i <= 100; i = i + 1) {
        sum = sum + i;
    }
    print(sum);
    send;
}
"#,
    );
}

#[test]
fn e2e_nested_conditionals() {
    full_pipeline(
        r#"
craft classify(n: i32) -> i32 {
    check (n > 0) {
        check (n > 100) {
            send 3;
        } otherwise {
            check (n > 10) {
                send 2;
            } otherwise {
                send 1;
            }
        }
    } otherwise {
        send 0;
    }
}
craft main() -> void {
    print(classify(150));
    print(classify(50));
    print(classify(5));
    print(classify(-1));
    send;
}
"#,
    );
}

#[test]
fn e2e_multiple_functions() {
    full_pipeline(
        r#"
craft max(a: i32, b: i32) -> i32 {
    check (a > b) { send a; }
    send b;
}
craft min(a: i32, b: i32) -> i32 {
    check (a < b) { send a; }
    send b;
}
craft clamp(v: i32, lo: i32, hi: i32) -> i32 {
    send min(max(v, lo), hi);
}
craft main() -> void {
    print(clamp(150, 0, 100));
    print(clamp(-5, 0, 100));
    print(clamp(42, 0, 100));
    send;
}
"#,
    );
}

#[test]
fn e2e_gcd() {
    full_pipeline(
        r#"
craft gcd(a: i32, b: i32) -> i32 {
    check (b == 0) { send a; }
    send gcd(b, a % b);
}
craft main() -> void {
    make r: i32 = gcd(48, 18);
    print(r);
    send;
}
"#,
    );
}

#[test]
fn e2e_power() {
    full_pipeline(
        r#"
craft power(base: i32, exp: i32) -> i32 {
    check (exp == 0) { send 1; }
    send base * power(base, exp - 1);
}
craft main() -> void {
    make r: i32 = power(2, 10);
    print(r);
    send;
}
"#,
    );
}

// ── E2E with preprocessor defines ──

#[test]
fn e2e_with_debug_define() {
    let ir = full_pipeline_with_defines(
        r#"
#if DEBUG
craft main() -> void {
    print("debug mode");
    send;
}
#else
craft main() -> void {
    print("release mode");
    send;
}
#endif
"#,
        &["DEBUG"],
    );
    assert!(ir.contains("debug mode"));
    assert!(!ir.contains("release mode"));
}

#[test]
fn e2e_without_debug_define() {
    let ir = full_pipeline_with_defines(
        r#"
#if DEBUG
craft main() -> void {
    print("debug mode");
    send;
}
#else
craft main() -> void {
    print("release mode");
    send;
}
#endif
"#,
        &[],
    );
    assert!(ir.contains("release mode"));
}

// ── E2E float programs ──

#[test]
fn e2e_float_arithmetic() {
    full_pipeline(
        r#"
craft main() -> void {
    make a: f64 = 1.5;
    make b: f64 = 2.5;
    make c: f64 = a + b;
    make d: f64 = a * b;
    print(c);
    print(d);
    send;
}
"#,
    );
}

// ── E2E bool programs ──

#[test]
fn e2e_boolean_logic() {
    full_pipeline(
        r#"
craft main() -> void {
    make a: bool = true;
    make b: bool = false;
    make c: bool = a && b;
    make d: bool = a || b;
    check (d) {
        print(1);
    }
    send;
}
"#,
    );
}

// ── E2E mixed types ──

#[test]
fn e2e_mixed_type_program() {
    full_pipeline(
        r#"
craft abs_val(x: i32) -> i32 {
    check (x < 0) { send 0 - x; }
    send x;
}
craft main() -> void {
    make x: i32 = -42;
    make y: i32 = abs_val(x);
    make name: str = "result";
    print(name);
    print(y);
    send;
}
"#,
    );
}

// ── E2E with stdlib import (parse-only, codegen may not support all modules) ──

#[test]
fn e2e_simd_import_pipeline() {
    let ir = full_pipeline_with_imports(
        r#"
bring "simd";
craft main() -> void {
    send;
}
"#,
    );
    assert!(ir.contains("_ql_main") || ir.contains("main"));
}

// ── Stress tests ──

#[test]
fn e2e_many_variables() {
    let mut src = String::from("craft main() -> void {\n");
    for i in 0..50 {
        src.push_str(&format!("    make v{}: i32 = {};\n", i, i));
    }
    src.push_str("    send;\n}\n");
    full_pipeline(&src);
}

#[test]
fn e2e_deeply_nested_ifs() {
    let mut src = String::from("craft main() -> void {\n    make x: i32 = 42;\n");
    for _ in 0..10 {
        src.push_str("    check (x > 0) {\n");
    }
    src.push_str("        print(x);\n");
    for _ in 0..10 {
        src.push_str("    }\n");
    }
    src.push_str("    send;\n}\n");
    full_pipeline(&src);
}

#[test]
fn e2e_many_functions() {
    let mut src = String::new();
    for i in 0..20 {
        src.push_str(&format!(
            "craft func{}(x: i32) -> i32 {{ send x + {}; }}\n",
            i, i
        ));
    }
    src.push_str("craft main() -> void {\n");
    src.push_str("    make shift r: i32 = 0;\n");
    for i in 0..20 {
        src.push_str(&format!("    r = func{}(r);\n", i));
    }
    src.push_str("    print(r);\n    send;\n}\n");
    full_pipeline(&src);
}
