use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        a: [i64; n+1],
        mut c: [i64; n+m+1],
    }
    let mut b = vec![0; m + 1];
    for i in (0..m + 1).rev() {
        b[i] = c[i + n] / a[n];
        for j in 0..n + 1 {
            c[i + j] -= b[i] * a[j];
        }
    }
    for i in 0..m + 1 {
        print!("{} ", b[i]);
    }
}
