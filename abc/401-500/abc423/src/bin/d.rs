use proconio::{fastout, input};
use std::collections::BinaryHeap;

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
    }

    let mut restaurant = BinaryHeap::new();
    let mut p = 0;
    let mut t = 0;

    for _ in 0..n {
        input! {
            a: usize,
            b: usize,
            c: usize,
        }

        t = t.max(a);

        while p + c > k {
            if let Some((std::cmp::Reverse(pb), pc)) = restaurant.pop() {
                p -= pc;
                t = t.max(pb);
            }
        }

        println!("{}", t);
        restaurant.push((std::cmp::Reverse(t + b), c));
        p += c;
    }
}
