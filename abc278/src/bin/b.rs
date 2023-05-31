// use std::collections::HashSet;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use std::collections::BinaryHeap;
// use proconio::marker::Chars;
use proconio::input;

fn judge(h1: usize, h2: usize, m1: usize, m2: usize) -> bool {
    let x = h1 * 10 + m1;
    let y = h2 * 10 + m2;
    x <= 23 && y <= 59 
}

fn main() {
    input! {
        h: usize,
        m: usize,
    }
    let mut h1 = h / 10;
    let mut h2 = h % 10;
    let mut m1 = m / 10;
    let mut m2 = m % 10;
    while judge(h1, h2, m1, m2) == false {
        // let x = h1 * 10 + h2;
        // let y = m1 * 10 + m2;
        if m2 == 9 {
            m2 = 0;
            m1 += 1;
            if m1 == 6 {
                m1 = 0;
                h2 += 1;
                if h2 == 10 {
                    h2 = 0;
                    h1 += 1;
                } else if h2 == 4 && h1 == 2 {
                    h1 = 0;
                    h2 = 0;
                }
            }
        } else {
            m2 += 1;
        }
    }
    let x = h1 * 10 + h2;
    let y = m1 * 10 + m2;
    println!("{} {}", x, y);
}

