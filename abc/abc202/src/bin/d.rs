// use std::collections::HashSet;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use std::collections::BinaryHeap;
// use proconio::marker::Chars;
use proconio::input;

fn main() {
    input! {
        a: u64,
        b: u64,
        k: u64,
    }
    for i in 0..(1_u64 << (a+b)) {
        let mut candidate = "".to_string();
        for j in (0_u64..(a+b)).rev() {
            if i & (1 << j) == 0{
                candidate.push('a');
            } else {
                candidate.push('b');
            }
        }
        println!("{}", &candidate);
        // if i == 3 {
        //     println!("{}", &candidate);
        //     return;
        // }
    }
}

