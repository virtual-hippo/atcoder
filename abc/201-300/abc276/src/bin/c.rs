// use std::collections::HashSet;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use std::collections::BinaryHeap;
// use proconio::marker::Chars;
use proconio::input;

fn main() {
    input! {
        n: usize,
        p: [usize; n],
    }
    let pos = {
        let mut ret = 0;
        for i in (0..n - 1).rev() {
            if p[i] > p[i + 1] {
                ret = i;
                break;
            }
        }
        ret
    };
    let new = (pos + 1..n)
        .map(|i| p[i])
        .filter(|&x| p[pos] > x)
        .max()
        .unwrap();
    let mut new_tail = (pos..n)
        .map(|i| p[i])
        .filter(|&x| x != new)
        .collect::<Vec<usize>>();

    new_tail.sort_by(|a, b| b.cmp(a));

    for i in 0..pos {
        print!("{} ", p[i]);
    }
    print!("{} ", new);
    for i in 0..new_tail.len() {
        print!("{} ", new_tail[i]);
    }
}
