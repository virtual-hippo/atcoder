// use std::collections::HashSet;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use std::collections::BinaryHeap;
// use proconio::marker::Chars;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        a: [[usize; n]; n],
    }
    let center_i = (k * k) / 2 + 1;
    let mut ans = usize::MAX;
    for i in 0..n - k + 1 {
        for j in 0..n - k + 1 {
            let mut koho = vec![];
            for ii in i..i + k {
                for jj in j..j + k {
                    // print!("{} ", a[ii][jj]);
                    koho.push(a[ii][jj]);
                }
                // println!();
            }
            koho.sort_by(|a, b| b.cmp(a));
            ans = ans.min(koho[center_i - 1]);
            // println!();
        }
    }
    println!("{}", ans);
}
