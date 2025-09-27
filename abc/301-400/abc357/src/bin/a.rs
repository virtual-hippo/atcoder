use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        h: [usize; n],
    }

    let mut m = m;
    for i in 0..n {
        if m >= h[i] {
            m -= h[i];
        } else {
            println!("{}", i);
            return;
        }
    }

    let ans = n;
    println!("{}", ans);
}
