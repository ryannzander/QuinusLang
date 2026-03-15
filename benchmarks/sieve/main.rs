// Sieve of Eratosthenes: find primes up to N
fn main() {
    let n: usize = 1_000_000;
    let mut buf = vec![1u8; n + 1];
    let limit = (n as f64).sqrt() as usize;
    for i in 2..=limit {
        if buf[i] != 0 {
            let mut j = i * i;
            while j <= n {
                buf[j] = 0;
                j += i;
            }
        }
    }
    let count = buf[2..=n].iter().filter(|&&x| x != 0).count();
    println!("primes up to {}: {}", n, count);
}
