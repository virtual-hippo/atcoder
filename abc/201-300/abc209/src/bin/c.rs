use proconio::input;

fn main() {
    input! {
        n: usize,
        mut c: [usize; n],
    }
    const MOD: usize = 1_000_000_007;

    c.sort();
    let seki = (0..n).fold(1, |seki, i| (seki * (c[i] - i)) % MOD);
    println!("{}", seki);
}
