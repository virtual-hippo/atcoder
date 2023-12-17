use proconio::{fastout, input};
use superslice::Ext;

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        a: [u64; n],
    }
    let mut c = vec![0; n];
    for i in 0..n {
        c[i] = a[i] - (i + 1) as u64;
    }
    for _ in 0..q {
        input! {
            k: u64,
        }
        if c[n - 1] < k {
            println!("{}", a[n - 1] + (k - c[n - 1]));
        } else {
            let i = c.lower_bound(&k);
            println!("{}", (a[i] - 1) - (c[i] - k));
        }
    }
}
