// Fibonacci benchmark: iterative fib(40)
fn fib(n: i32) -> i64 {
    if n <= 1 {
        return n as i64;
    }
    let mut a: i64 = 0;
    let mut b: i64 = 1;
    let mut i = 2;
    while i <= n {
        let tmp = a + b;
        a = b;
        b = tmp;
        i += 1;
    }
    b
}

fn main() {
    let result = fib(40);
    println!("fib(40): {}", result);
}
