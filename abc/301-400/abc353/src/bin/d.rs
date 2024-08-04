use proconio::{fastout, input};

const M: u64 = 998244353;

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [u64;n],
    }
    let mut ans = 0;
    let mut l = vec![0; n];
    for i in 0..n {
        l[i] = a[i].to_string().len() as u32;
    }
    let mut s = vec![0; n + 1];
    for i in 0..n {
        s[i + 1] = a[i] + s[i];
        s[i + 1] %= M;
    }

    for i in 0..n - 1 {
        ans += 10_u64.pow(l[i + 1]) * s[i + 1];
        ans %= M;
    }

    for i in 1..n {
        ans += a[i] * (i as u64);
        ans %= M;
    }
    println!("{}", ans);
}
