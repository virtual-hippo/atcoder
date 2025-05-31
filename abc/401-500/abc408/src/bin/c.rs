use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
    }
    let mut s = vec![0_i64; n + 1];

    for _ in 0..m {
        input! {
            l: usize,
            r: usize,
        }
        let a = l - 1;
        let b = r - 1;
        s[a] += 1;
        s[b + 1] -= 1;
    }
    let mut ss = vec![0; n + 1];
    for i in 0..n {
        ss[i + 1] = ss[i] + s[i];
    }
    let v = (1..n + 1).map(|i| ss[i]);

    let ans = v.min().unwrap_or(0);
    println!("{}", ans);
}
