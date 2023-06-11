// use std::collections::HashSet;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use std::collections::BinaryHeap;
// use proconio::marker::Chars;
use proconio::input;

fn main() {
    input! {
        (h,w): (usize, usize),
        a: [[u8; w]; h],
    }
    for i in 0..h{
        for j in 0..w{
            if a[i][j] == 0 {
                print!(".");
            } else {
                print!("{}", (64_u8 + a[i][j]) as char);
            }
        }
        println!("");
    }
}

