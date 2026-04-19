use proconio::input;
use std::cmp::Reverse;
use std::collections::*;

fn main() {
    input! {
        q: usize,
    }

    let mut heap = BinaryHeap::new();
    let mut now = 0;

    for _ in 0..q {
        input! {
            q: usize,
            h: usize,
        }
        if q == 1 {
            heap.push(Reverse(h));
            now += 1;
            println!("{}", now);
        } else {
            while let Some(Reverse(ph)) = heap.pop() {
                if ph > h {
                    heap.push(Reverse(ph));
                    break;
                }
                now -= 1;
            }
            println!("{}", now);
        }
    }
}
