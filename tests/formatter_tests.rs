use qpp::{fmt, parse};

fn roundtrip(src: &str) {
    let program = parse(src).unwrap_or_else(|e| panic!("Parse failed: {}", e));
    let formatted = fmt::format_program(&program);
    let reparsed = parse(&formatted)
        .unwrap_or_else(|e| panic!("Reparse failed after fmt: {}\n{}", e, formatted));
    assert_eq!(
        program.items.len(),
        reparsed.items.len(),
        "Item count mismatch after roundtrip.\nOriginal: {}\nFormatted: {}",
        src,
        formatted
    );
}

#[test]
fn fmt_simple_function() {
    roundtrip(
        r#"craft main() -> void {
    make x: i32 = 42;
    send;
}
"#,
    );
}

#[test]
fn fmt_function_with_params() {
    roundtrip("craft add(a: i32, b: i32) -> i32 { send a + b; }");
}

#[test]
fn fmt_if_else() {
    roundtrip(
        r#"craft main() -> void {
    check (x > 0) {
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
fn fmt_for_loop() {
    roundtrip(
        r#"craft main() -> void {
    for (make shift i: i32 = 0; i < 10; i = i + 1) {
        print(i);
    }
    send;
}
"#,
    );
}

#[test]
fn fmt_while_loop() {
    roundtrip(
        r#"craft main() -> void {
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
fn fmt_foreach() {
    roundtrip(
        r#"craft main() -> void {
    make shift arr: [i32; 3] = { 1, 2, 3 };
    foreach x in arr { print(x); }
    send;
}
"#,
    );
}

#[test]
fn fmt_defer() {
    roundtrip(
        r#"craft main() -> void {
    defer { print(1); }
    send;
}
"#,
    );
}

#[test]
fn fmt_struct() {
    roundtrip("form Point { x: i32, y: i32, }");
}

#[test]
fn fmt_enum() {
    roundtrip("state Color { Red, Green, Blue, }");
}

#[test]
fn fmt_constant() {
    roundtrip(
        r#"eternal MAX: i32 = 100;
craft main() -> void { send; }
"#,
    );
}

#[test]
fn fmt_extern() {
    roundtrip(
        r#"extern craft puts(s: str) -> i32;
craft main() -> void { send; }
"#,
    );
}

#[test]
fn fmt_multiple_functions() {
    roundtrip(
        r#"craft foo() -> void { send; }
craft bar(x: i32) -> i32 { send x; }
craft main() -> void {
    make r: i32 = bar(42);
    send;
}
"#,
    );
}

#[test]
fn fmt_nested_if() {
    roundtrip(
        r#"craft main() -> void {
    check (x > 0) {
        check (x > 10) {
            print(2);
        }
    }
    send;
}
"#,
    );
}

#[test]
fn fmt_complex_expressions() {
    roundtrip(
        r#"craft main() -> void {
    make x: i32 = 1 + 2 * 3;
    make y: bool = x > 5 && x < 20;
    send;
}
"#,
    );
}

#[test]
fn fmt_hazard_block() {
    roundtrip(
        r#"craft main() -> void {
    hazard {
        print(1);
    }
    send;
}
"#,
    );
}

#[test]
fn fmt_try_catch() {
    roundtrip(
        r#"craft main() -> void {
    try {
        print(1);
    } catch (e) {
        print(e);
    }
    send;
}
"#,
    );
}

#[test]
fn fmt_alias() {
    roundtrip(
        r#"alias MyInt = i32;
craft main() -> void { send; }
"#,
    );
}

#[test]
fn fmt_impl_block() {
    roundtrip(
        r#"form Vec2 { x: f64, y: f64, }
impl Vec2 {
    craft length() -> f64 { send 0.0; }
}
"#,
    );
}

#[test]
fn fmt_idempotent() {
    let src = r#"craft main() -> void {
    make x: i32 = 42;
    print(x);
    send;
}
"#;
    let program = parse(src).unwrap();
    let formatted1 = fmt::format_program(&program);
    let program2 = parse(&formatted1).unwrap();
    let formatted2 = fmt::format_program(&program2);
    assert_eq!(formatted1, formatted2, "Formatter is not idempotent");
}
