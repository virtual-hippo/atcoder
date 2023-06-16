// use std::collections::HashSet;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use std::collections::BinaryHeap;
// use proconio::marker::Chars;
use proconio::input;
use proconio::marker::Chars;
use std::collections::VecDeque;

fn main() {
    input! {
        n: usize,
        a: u64,
        b: u64,
        s: Chars,
    }
    let mut deq = VecDeque::from(s);
    let mut ans = std::u64::MAX;
    for i in 0..n-1 {
        if i != 0 {
            let head = deq.pop_front().unwrap();
            deq.push_back(head);
        }

        let mut j = 0;
        let mut cnt = 0_u64;
        while j <= n-1-j {
            if deq[j] != deq[n-1-j] {
                cnt += 1;
            }
            j += 1;
        }
        let current = a * (i as u64) + b * cnt;
        ans = std::cmp::min(current, ans);
    }
    println!("{}", ans);
}

