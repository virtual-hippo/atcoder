use proconio::input;

fn main() {
    input! {
        n: i64,
        m: i64,
    }
    const MOD: i64 = 1_000_000_007;
    if 1 < (n - m).abs() {
        println!("{}", 0);
        return;
    }
    if n == m {
        let kaijo1 = (1..n+1).fold(1, |seki, v| (seki * v) % MOD);
        println!("{}", (kaijo1 * kaijo1 * 2) % MOD);
    } else {
        let kaijo1 = (1..n+1).fold(1, |seki, v| (seki * v) % MOD);
        let kaijo2 = (1..m+1).fold(1, |seki, v| (seki * v) % MOD);
        println!("{}", (kaijo1 * kaijo2) % MOD);
    }
}

