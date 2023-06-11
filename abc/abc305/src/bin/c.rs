// use std::collections::HashSet;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use std::collections::BinaryHeap;
// use proconio::marker::Chars;
use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        (h,w): (usize, usize),
        s: [Chars; h],
    }
    let mut x_vec = vec![];
    let mut y_vec = vec![];
    for i in 0..h {
        for j in 0..w {
            if s[i][j] == '#' {
                x_vec.push(j);
                y_vec.push(i);
            }
        }
    }
    let left = x_vec.iter().fold(w-1, |min, x| std::cmp::min(min, *x));
    let right = x_vec.iter().fold(0, |max, x| std::cmp::max(max, *x));
    let up = y_vec.iter().fold(h-1, |min, x| std::cmp::min(min, *x));
    let bottom = y_vec.iter().fold(0, |max, x| std::cmp::max(max, *x));
    for i in up..=bottom {
        for j in left..=right {
            if s[i][j] == '.' {
                println!("{} {}", i+1, j+1);
                return;
            }
        }
    }
}

