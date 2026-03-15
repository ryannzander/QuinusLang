// Sum benchmark: simple loop summing 1..N
craft main() -> void {
    make N: i64 = 100_000_000;
    make shift s: i64 = 0;
    make shift i: i64 = 0;
    loopwhile (i < N) {
        s = s + i;
        i = i + 1;
    }
    print(`sum: ${s}`);
    send;
}
