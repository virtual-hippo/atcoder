use proconio::{fastout, input};
use superslice::Ext;

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        r: [usize; n],
    }
    let sorted_r = {
        let mut r = r.clone();
        r.sort();
        r
    };
    let mut s = vec![0; n + 1];
    for i in 0..n {
        s[i + 1] = s[i] + sorted_r[i];
    }
    for _ in 0..q {
        input! {
            x: usize,
        }
        let ind = s.lower_bound(&x);
        let ans = if ind < n + 1 && s[ind] == x {
            ind
        } else {
            ind - 1
        };

        println!("{}", ans);
    }
}
