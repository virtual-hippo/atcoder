// use std::collections::HashSet;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use std::collections::BinaryHeap;
// use proconio::marker::Chars;
use proconio::input;

type Shohin = (usize,usize, Vec<usize>);

fn is_ok(x: &Shohin, y: &Shohin) -> bool {
    if x.0 < y.0 {
        return false;
    }
    if x.1 > y.1 {
        return false;
    }
    for i in 0..x.1 {
        let mut is_contain = false;
        for j in 0..y.1 {
            if x.2[i] == y.2[j] {
                is_contain = true;
            }
        }
        if is_contain == false {
            return false;
        }
    }
    for i in 0..y.1 {
        let mut is_contain = false;
        for j in 0..x.1 {
            if y.2[i] == x.2[j] {
                is_contain = true;
            }
        }
        if is_contain == false {
            return true;
        }
    }
    if x.0 > y.0 {
        true
    } else {
        false
    }
}

fn main() {
    input! {
        n: usize,
        _m: usize,
    }
    let mut shohin = Vec::with_capacity(n);
    for _ in 0..n {
        input! {
            p: usize,
            c: usize,
            f: [usize; c],
        }
        shohin.push((p,c,f));
    }
    for i in 0..n {
        for j in 0..n {
            if is_ok(&shohin[i], &shohin[j]) {
                println!("Yes");
                return;
            }
        }
    }
    println!("No");
}

