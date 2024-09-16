use proconio::{fastout, input};
use superslice::Ext;

#[fastout]
fn main() {
    input! {
        n: usize,
        x: [i32; n],
        p: [u64; n],
    }
    let mut s = vec![0; n + 1];
    for i in 0..n {
        s[i + 1] = p[i] + s[i];
    }

    input! {
        q: usize,
    }

    for _ in 0..q {
        input! {
            l: i32,
            r: i32,
        }
        let ll = x.upper_bound(&(l - 1));
        let rr = x.upper_bound(&(r)).min(n);
        println!("{}", s[rr] - s[ll]);
    }
}
