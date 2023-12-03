use proconio::{fastout, input};
use superslice::Ext;

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        mut a: [usize; n],
    }
    a.sort();
    for _ in 0..q {
        input! {x: usize}
        let ans = n - a.lower_bound(&x);
        println!("{}", ans);
    }
}
