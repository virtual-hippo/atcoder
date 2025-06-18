use proconio::{fastout, input};

use superslice::Ext;

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
        q: usize,
    }
    let mut idx = vec![vec![]; n];

    for i in 0..n {
        let v = a[i] - 1;
        idx[v].push(i);
    }

    for _ in 0..q {
        input! {
            l: usize,
            r: usize,
            x: usize,
        }
        let l = l - 1;
        let x = x - 1;

        let l_idx = idx[x].lower_bound(&l);
        let r_idx = idx[x].lower_bound(&r);

        println!("{}", r_idx - l_idx);
    }
}
