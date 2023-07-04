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
    let mut vec = vec![];
    for i in 0..h {
        for j in 0..w {
            if s[i][j] == 'o' {
                vec.push((i,j));
            }
        }
    }
    let yoko = if vec[0].1 < vec[1].1 {
        vec[1].1 - vec[0].1
    } else {
        vec[0].1 - vec[1].1
    };

    let tate = if vec[0].0 < vec[1].0 {
        vec[1].0 - vec[0].0
    } else {
        vec[0].0 - vec[1].0
    };
    
    println!("{}", yoko + tate);
}

