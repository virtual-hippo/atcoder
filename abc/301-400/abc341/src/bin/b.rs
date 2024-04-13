use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let mut a = a.clone();
    for i in 0..n - 1 {
        input! {
            s: usize,
            t: usize,
        }
        a[i + 1] += (a[i] / s) * t;
    }
    let ans = a[n - 1];
    println!("{}", ans);
}
