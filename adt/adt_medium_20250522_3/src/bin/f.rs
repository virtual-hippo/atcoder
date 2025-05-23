use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        s: Chars,
    }
    let mut a = vec![0; n];
    for i in 1..n {
        if s[i] == s[i - 1] {
            a[i] = 1;
        }
    }
    let mut ss = vec![0; n + 1];
    for i in 0..n {
        ss[i + 1] = ss[i] + a[i];
    }

    for _ in 0..q {
        input! {
            l: usize,
            r: usize,
        }
        let ans = ss[r] - ss[l];
        println!("{}", ans);
    }
}
