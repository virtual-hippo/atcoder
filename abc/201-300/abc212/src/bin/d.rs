use proconio::input;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn main() {
    input! {
        q: usize,
    }
    let mut heap = BinaryHeap::new();
    let mut diff = 0;

    for _ in 0..q {
        input! {
            query: usize,
        }
        match query {
            1 => {
                input! {
                    x: i64,
                }
                heap.push(Reverse(x - diff));
            }
            2 => {
                input! {
                    x: i64,
                }
                diff += x;
            }
            3 => {
                if let Some(Reverse(val)) = heap.pop() {
                    println!("{}", val + diff);
                }
            }
            _ => unreachable!(),
        }
    }
}
