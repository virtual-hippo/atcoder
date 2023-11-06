// use std::collections::HashSet;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use std::collections::BinaryHeap;
// use proconio::marker::Chars;
use proconio::input;
use std::collections::HashSet;

fn check1(a: &Vec<Vec<usize>>) -> bool {
    for i in 0..9 {
        let mut set = HashSet::with_capacity(9);
        for j in 0..9 {
            set.insert(a[i][j]);
        }
        if set.len() != 9 {
            return false;
        }
    }
    true
}

fn check2(a: &Vec<Vec<usize>>) -> bool {
    for i in 0..9 {
        let mut set = HashSet::with_capacity(9);
        for j in 0..9 {
            set.insert(a[j][i]);
        }
        if set.len() != 9 {
            return false;
        }
    }
    true
}

fn check3(a: &Vec<Vec<usize>>) -> bool {
    let array = [0, 3, 6];
    for &r in array.iter() {
        for &c in array.iter() {
            let mut set = HashSet::with_capacity(9);
            for i in r..(r + 3) {
                for j in c..(c + 3) {
                    set.insert(a[j][i]);
                }
            }
            if set.len() != 9 {
                return false;
            }
        }
    }
    true
}

fn main() {
    input! {
        a: [[usize; 9]; 9],
    }
    if check1(&a) && check2(&a) && check3(&a) {
        println!("Yes");
    } else {
        println!("No");
    }
}
