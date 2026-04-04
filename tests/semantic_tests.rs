use qpp::{analyze, parse};

fn analyze_ok(src: &str) {
    let program = parse(src).unwrap_or_else(|e| panic!("Parse failed: {}", e));
    analyze(&program).unwrap_or_else(|e| panic!("Analyze failed: {}", e));
}

fn analyze_err(src: &str) -> String {
    let program = parse(src).unwrap_or_else(|e| panic!("Parse failed: {}", e));
    let err = analyze(&program).unwrap_err();
    format!("{}", err)
}

// ── Basic programs that should pass ──

#[test]
fn sem_minimal_main() {
    analyze_ok("craft main() -> void { send; }");
}

#[test]
fn sem_variable_declaration_and_use() {
    analyze_ok(
        r#"
craft main() -> void {
    make x: i32 = 42;
    print(x);
    send;
}
"#,
    );
}

#[test]
fn sem_mutable_variable_reassign() {
    analyze_ok(
        r#"
craft main() -> void {
    make shift x: i32 = 0;
    x = 10;
    print(x);
    send;
}
"#,
    );
}

#[test]
fn sem_function_call() {
    analyze_ok(
        r#"
craft add(a: i32, b: i32) -> i32 { send a + b; }
craft main() -> void {
    make r: i32 = add(1, 2);
    print(r);
    send;
}
"#,
    );
}

#[test]
fn sem_nested_function_calls() {
    analyze_ok(
        r#"
craft double(x: i32) -> i32 { send x * 2; }
craft inc(x: i32) -> i32 { send x + 1; }
craft main() -> void {
    make r: i32 = double(inc(5));
    print(r);
    send;
}
"#,
    );
}

#[test]
fn sem_if_else() {
    analyze_ok(
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
}

#[test]
fn sem_for_loop() {
    analyze_ok(
        r#"
craft main() -> void {
    for (make shift i: i32 = 0; i < 10; i = i + 1) {
        print(i);
    }
    send;
}
"#,
    );
}

#[test]
fn sem_while_loop() {
    analyze_ok(
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
}

#[test]
fn sem_constant() {
    analyze_ok(
        r#"
eternal MAX: i32 = 100;
craft main() -> void {
    print(MAX);
    send;
}
"#,
    );
}

#[test]
fn sem_string_variable() {
    analyze_ok(
        r#"
craft main() -> void {
    make s: str = "hello world";
    print(s);
    send;
}
"#,
    );
}

#[test]
fn sem_bool_variable() {
    analyze_ok(
        r#"
craft main() -> void {
    make b: bool = true;
    check (b) { print(1); }
    send;
}
"#,
    );
}

#[test]
fn sem_float_variable() {
    analyze_ok(
        r#"
craft main() -> void {
    make f: f64 = 3.14;
    print(f);
    send;
}
"#,
    );
}

#[test]
fn sem_multiple_types() {
    analyze_ok(
        r#"
craft main() -> void {
    make a: i32 = 1;
    make b: i64 = 2;
    make c: f32 = 1.0;
    make d: f64 = 2.0;
    make e: bool = true;
    make f: str = "hi";
    send;
}
"#,
    );
}

#[test]
fn sem_struct_definition() {
    analyze_ok(
        r#"
form Point { x: i32, y: i32, }
craft main() -> void { send; }
"#,
    );
}

#[test]
fn sem_enum_definition() {
    analyze_ok(
        r#"
state Color { Red, Green, Blue, }
craft main() -> void { send; }
"#,
    );
}

#[test]
fn sem_extern_declaration() {
    analyze_ok(
        r#"
extern craft puts(s: str) -> i32;
craft main() -> void { send; }
"#,
    );
}

#[test]
fn sem_type_alias() {
    analyze_ok(
        r#"
alias Byte = u8;
craft main() -> void { send; }
"#,
    );
}

#[test]
fn sem_write_writeln_builtins() {
    analyze_ok(
        r#"
craft main() -> void {
    write(1);
    writeln(2);
    send;
}
"#,
    );
}

#[test]
fn sem_defer() {
    analyze_ok(
        r#"
craft main() -> void {
    defer { print(1); }
    print(2);
    send;
}
"#,
    );
}

#[test]
fn sem_digit_separators() {
    analyze_ok(
        r#"
craft main() -> void {
    make big: i32 = 1_000_000;
    print(big);
    send;
}
"#,
    );
}

#[test]
fn sem_arithmetic_expressions() {
    analyze_ok(
        r#"
craft main() -> void {
    make a: i32 = 10 + 5;
    make b: i32 = 10 - 5;
    make c: i32 = 10 * 5;
    make d: i32 = 10 / 5;
    make e: i32 = 10 % 3;
    send;
}
"#,
    );
}

#[test]
fn sem_comparison_expressions() {
    analyze_ok(
        r#"
craft main() -> void {
    make a: bool = 1 == 1;
    make b: bool = 1 != 2;
    make c: bool = 1 < 2;
    make d: bool = 2 > 1;
    make e: bool = 1 <= 1;
    make f: bool = 2 >= 1;
    send;
}
"#,
    );
}

#[test]
fn sem_logical_operators() {
    analyze_ok(
        r#"
craft main() -> void {
    make a: bool = true && false;
    make b: bool = true || false;
    send;
}
"#,
    );
}

#[test]
fn sem_function_returning_value() {
    analyze_ok(
        r#"
craft square(x: i32) -> i32 {
    send x * x;
}
craft main() -> void {
    make r: i32 = square(5);
    print(r);
    send;
}
"#,
    );
}

#[test]
fn sem_recursive_function() {
    analyze_ok(
        r#"
craft fib(n: i32) -> i32 {
    check (n <= 1) { send n; }
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

// ── Error cases ──

#[test]
fn sem_error_undefined_variable() {
    let err = analyze_err(
        r#"
craft main() -> void {
    make x: i32 = y;
    send;
}
"#,
    );
    assert!(
        err.contains("Undefined") || err.contains("undefined") || err.contains("not defined"),
        "Expected undefined var error, got: {}",
        err
    );
}

#[test]
fn sem_error_type_mismatch() {
    let err = analyze_err(
        r#"
craft main() -> void {
    make x: i32 = "hello";
    send;
}
"#,
    );
    assert!(
        err.contains("assign") || err.contains("type") || err.contains("mismatch"),
        "Expected type mismatch error, got: {}",
        err
    );
}

#[test]
fn sem_error_wrong_arg_count() {
    let err = analyze_err(
        r#"
craft main() -> void {
    make n: usize = len(1, 2);
    send;
}
"#,
    );
    assert!(
        err.contains("len") || err.contains("argument") || err.contains("param"),
        "Expected arg count error, got: {}",
        err
    );
}

#[test]
fn sem_error_const_type_mismatch() {
    let err = analyze_err(
        r#"
eternal X: i32 = "bad";
craft main() -> void { send; }
"#,
    );
    assert!(
        err.contains("constant") || err.contains("assign") || err.contains("type"),
        "Expected const type error, got: {}",
        err
    );
}

#[test]
fn sem_error_duplicate_function() {
    let result = parse(
        r#"
craft foo() -> void { send; }
craft foo() -> void { send; }
craft main() -> void { send; }
"#,
    );
    if let Ok(program) = result {
        let res = analyze(&program);
        if let Err(e) = res {
            let msg = format!("{}", e);
            assert!(
                msg.contains("duplicate") || msg.contains("already") || msg.contains("redefin"),
                "Expected duplicate error, got: {}",
                msg
            );
        }
    }
}
