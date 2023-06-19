// use std::collections::HashSet;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use std::collections::BinaryHeap;
// use proconio::marker::Chars;
use proconio::input;

fn calc_d(a: usize,b: usize,c: usize, x: usize) -> usize {
    let mut current = 0;
    let mut ret = 0;
    while current < x {
        let mut run = 0;
        while current < x && run < a {
            ret += b;
            current += 1;
            run += 1;
        }
        let mut rest = 0;
        while current < x && rest < c {
            current += 1;
            rest += 1;
        }
    }
    ret
}

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
        d: usize,
        e: usize,
        f: usize,
        x: usize,
    }
    let takahashi = calc_d(a,b,c,x);
    let aoki = calc_d(d,e,f,x);
    if takahashi == aoki {
        println!("Draw");
    } else if takahashi < aoki {
        println!("Aoki");
    } else {
        println!("Takahashi");
    }
}

