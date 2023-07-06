// use std::collections::HashSet;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use std::collections::BinaryHeap;
// use proconio::marker::Chars;
use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        (h,w): (usize, usize),
        a: [[usize; w]; h],
    }
    let kaisuu = h-1 + w-1;
    let mut ans = 0;

    for bit in 0..(1<<kaisuu) {
        let mut current = (0,0);
        let mut set = HashSet::with_capacity(20);
        set.insert(a[current.0][current.1]);
        for i in 0..kaisuu {
            if bit & (1 << i) == 0 {
                // 下
                if current.0 + 1 < h {
                    current.0 += 1;
                } else {
                    break;
                }
            } else {
                // 右
                if current.1 + 1 < w {
                    current.1 += 1;
                } else {
                    break;
                }
            }
            set.insert(a[current.0][current.1]);
        }
        if set.len() == kaisuu+1 {
            ans += 1;
        }
    }
    println!("{}", ans);
}

