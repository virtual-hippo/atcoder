// use std::collections::HashSet;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use std::collections::BinaryHeap;
// use proconio::marker::Chars;
use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let mut vec: Vec<(usize, usize)> = Vec::with_capacity(n);
    let mut prev = 0;
    let mut cnt = 0;
    for i in 0..n {
        cnt += 1;
        let k = a[i];
        let tail = if vec.len() == 0 {
            0
        } else {
            vec.len()-1
        };
        if k != prev {
            vec.push((k, 1));
            prev = k;
        } else {
            vec[tail].1 += 1;
            if vec[tail].1 >= k {
                vec[tail].1 -= k;
                cnt -= k;
                if vec[tail].1 == 0 {
                    vec.pop();
                    if vec.len() == 0 {
                        prev = 0;
                    } else {
                        let new_tail = vec.len()-1;
                        prev = vec[new_tail].0;
                    }
                }
            }
        }
        println!("{}", cnt);
    }
}

