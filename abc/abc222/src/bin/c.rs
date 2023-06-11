// use std::collections::HashSet;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use std::collections::BinaryHeap;
// use proconio::marker::Chars;
use proconio::input;
use proconio::marker::Chars;

fn janken(a: char, b: char) -> u8 {
    if a == b {
        0
    } else if a == 'G'{
        if b == 'C' {
            1
        } else {
            2
        }
    } else if a == 'C'{
        if b == 'P' {
            1
        } else {
            2
        }
    } else {
        if b == 'G' {
            1
        } else {
            2
        }
    }
}

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [Chars; 2 * n],
    }
    let mut cnt = (0..2*n).map(|x| (x, 0)).collect::<Vec<(usize, usize)>>();
    let mut current = (0..2*n).collect::<Vec<usize>>();
    for j in 0..m {
        for i in 0..n {
            let (left, right) = (current[i*2], current[i*2 + 1]); 
            let ret = janken(a[left][j], a[right][j]);
            if ret == 1 {
                cnt[left].1 += 1;
            } else if ret == 2 {
                cnt[right].1 += 1;
            }
        }
        let mut clone = cnt.clone();
        clone.sort_by(|a, b| b.1.cmp(&a.1));
        current = clone.iter().map(|(i, _)| *i).collect();
    }
    for i in current {
        println!("{}", i+1);
    }
}

