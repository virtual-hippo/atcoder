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
        c: [Chars; h],
    }
    let n = std::cmp::min(h,w);
    let mut s = vec![0; n];
    let mut centers = vec![];
    for i in 1..h-1 {
        for j in 1..w-1 {
            if
                c[i][j] == '#' &&
                c[i+1][j+1] == '#' &&
                c[i+1][j-1] == '#' &&
                c[i-1][j+1] == '#' &&
                c[i-1][j-1] == '#' {
                centers.push((i,j));
            }
        }
    }
    for i in 0..centers.len() {
        let mut size = 1;
        while
            centers[i].0 + size < h &&
            centers[i].1 + size < w &&
            c[centers[i].0 + size][centers[i].1 + size] == '#' {

            size += 1
        }
        s[size-2] += 1;
    }

    for i in 0..n {
        if i == n-1 {
            print!("{}\n", s[i]);
        } else {
            print!("{} ", s[i]);
        }
    }
}

// fn main() {
//     input! {
//         (h,w): (usize, usize),
//         c: [Chars; h],
//     }
//     let mut state = vec![vec![false; w]; h];
//     let n = std::cmp::min(h,w);
//     let mut s = vec![0; n];

//     for i in 0..h {
//         for j in 0..w {
//             if c[i][j] == '#' && state[i][j] == false {
//                 let mut len = 0;
//                 {
//                     let (mut k, mut l) = (i,j);
//                     while k < h && l < w && c[k][l] == '#'{
//                         k += 1;
//                         l += 1;
//                         len += 1;
//                     }
//                 }
//                 let size = len/2;
//                 s[size-1] += 1;

//                 let center = (i+size, j + size);
//                 state[center.0][center.1] = true;
//                 for k in 1..=size {
//                     state[center.0+k][center.1+k] = true;
//                     state[center.0-k][center.1+k] = true;
//                     state[center.0+k][center.1-k] = true;
//                     state[center.0-k][center.1-k] = true;
//                 }
//             }
//         }
//     }
//     for i in 0..n-1 {
//         print!("{} ", s[i]);
//     }
//     print!("{}", s[n-1]);
// }