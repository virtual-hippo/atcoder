use proconio::input;
use std::collections::BTreeSet;

// https://namacha411.hatenablog.com/entry/2022/02/28/194540
fn main() {
    input! {
        q: usize,
    }
    let mut a = BTreeSet::new();
    for i in 0..q {
        input! {
            query: usize,
        }
        if query == 1 {
            input! {
                x: usize,
            }
            a.insert((x, i));
        }
        if query == 2 {
            input! {
                x: usize,
                k: usize,
            }
            if let Some((x, _)) = a.range(..=(x, q+1)).rev().nth(k - 1) {
                println!("{}", x);
            } else {
                println!("-1");
            }
        }
        if query == 3 {
            input! {
                x: usize,
                k: usize,
            }
            if let Some((x, _)) = a.range((x, 0)..).nth(k - 1) {
                println!("{}", x);
            } else {
                println!("-1");
            }
        }
    }
}

