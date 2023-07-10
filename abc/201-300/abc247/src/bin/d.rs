// use std::collections::HashSet;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use std::collections::BinaryHeap;
// use proconio::marker::Chars;
use proconio::input;
use std::collections::VecDeque;

fn main() {
    input! {
        q: usize,
    }
    let mut deq = VecDeque::with_capacity(1_000_000);

    for _ in 0..q {
        input! {
            query: usize,
        }
        match query {
            1 => {
                input! {
                    x: usize,
                    c: usize,
                }
                deq.push_back((x, c));
            }
            2 => {
                input! {
                    c: usize,
                }
                let mut cnt = c;
                let mut ans = 0;
                while deq[0].1 < cnt {
                    ans += deq[0].0 * deq[0].1;
                    cnt -= deq[0].1;
                    deq.pop_front();
                }
                ans += deq[0].0 * cnt;
                deq[0].1 -= cnt;
                println!("{}", ans);
            }
            _ => unreachable!()
        }
    }
}

