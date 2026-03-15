// Fibonacci benchmark: iterative fib(40)
craft fib(n: i32) -> i64 {
    check (n <= 1) {
        send n as i64;
    }
    make shift a: i64 = 0;
    make shift b: i64 = 1;
    make shift i: i32 = 2;
    loopwhile (i <= n) {
        make tmp: i64 = a + b;
        a = b;
        b = tmp;
        i = i + 1;
    }
    send b;
}

craft main() -> void {
    make result: i64 = fib(40);
    print(`fib(40): ${result}`);
    send;
}
