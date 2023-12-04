use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: u128,
    }
    const MOD: u128 = 998_244_353;
    let mut ans = 0;
    let mut it = 10u128;
    while it - 1 < n {
        let len = it - 1 - it / 10 + 1;
        let last = (it * 9) / 10;
        let added = ((1 + last) * len / 2) % MOD;
        ans += added;
        ans %= MOD;
        it *= 10;
    }

    let len = n - it / 10 + 1;
    let last = n - (it / 10) + 1;
    let added = ((1 + last) * len / 2) % MOD;
    ans += added;
    ans %= MOD;

    println!("{}", ans);
}
