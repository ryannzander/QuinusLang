// Sum benchmark: simple loop summing 1..N
fn main() {
    let n: i64 = 100_000_000;
    let mut s: i64 = 0;
    let mut i: i64 = 0;
    while i < n {
        s += i;
        i += 1;
    }
    println!("sum: {}", s);
}
