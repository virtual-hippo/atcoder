use proconio::input;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn main() {
    input! {
        n: usize,
    }
    let mut vec = Vec::with_capacity(n);
    for _ in 0..n {
        input! {
            t: usize,
            d: usize,
        }
        vec.push((t, t + d));
    }
    vec.sort();

    let mut b_heap = BinaryHeap::with_capacity(n);

    let mut it = 0;
    let mut ans = 0;
    let mut current = 0;
    loop {
        if b_heap.is_empty() {
            if it == n {
                break;
            }
            current = vec[it].0;
        }
        while it < n && vec[it].0 == current {
            b_heap.push(Reverse(vec[it].1));
            it += 1;
        }
        while let Some(Reverse(val)) = b_heap.peek() {
            if *val < current {
                b_heap.pop();
            } else {
                break;
            }
        }

        if !b_heap.is_empty() {
            b_heap.pop();
            ans += 1;
        }
        current += 1;
    }
    println!("{}", ans);
}
