// use std::collections::HashSet;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use std::collections::BinaryHeap;
// use proconio::marker::Chars;
use proconio::input;
use std::collections::VecDeque;

fn main() {
    input! {
        n: usize,
        p: [i64; n],
    }
    let n_i64 = n as i64;
    let mut deq = VecDeque::from(p);
    let mut ans = 0;
    for ii in &[n_i64-1, 0, 1] {
        let mut current = 0;
        while deq[0] != *ii {
            let val = deq.pop_back().unwrap();
            deq.push_front(val);
        }
        for i in 0..n {
            let idx = i as i64;
            if idx -1 <= deq[i] && deq[i] <= idx + 1 {
                current += 1;
            }
        }
        ans = std::cmp::max(ans, current);
    }
    
    println!("{}", ans);
}

