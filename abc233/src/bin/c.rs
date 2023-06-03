// use std::collections::HashSet;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use std::collections::BinaryHeap;
// use proconio::marker::Chars;
use proconio::input;



fn f(x: usize,y: u64,n: usize, candidate: &mut Vec<u64>, table: &Vec<Vec<u64>>) {
    if x == n-1{
        for i in 0..table[x].len() {
            if table[x][i] > 1_000_000_000_000_000_000 / y {
                continue;
            }
            candidate.push(y * table[x][i]);
        }
    } else {
        for i in 0..table[x].len() {
            if table[x][i] > 1_000_000_000_000_000_000 / y {
                continue;
            }
            f(x+1,y * table[x][i],n,candidate, table);
        }
    }
}

fn main() {
    input! {
        n: usize,
        x: u64,
    }
    let mut table = vec![vec![]; n];
    for i in 0..n {
        input! {
            l: usize,
            a: [u64; l],
        }
        table[i] = a;
    }
    let mut candidate = vec![];
    f(0,1,n, &mut candidate, &table);
    let mut ans = 0;
    for i in 0..candidate.len() {
        if candidate[i] == x {
            ans += 1;
        }
    }
    println!("{}", ans);
}

