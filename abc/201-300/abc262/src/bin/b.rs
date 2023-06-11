// use std::collections::HashSet;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use std::collections::BinaryHeap;
// use proconio::marker::Chars;
use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        m: usize,
    }
    let mut graph = vec![HashSet::new(); n];
    for _ in 0..m {
        input! {
            u: usize,
            v: usize,
        }
        graph[u-1].insert(v-1);
        graph[v-1].insert(u-1);
    }
    let mut ans = 0;
    for i in 0..n-2 {
        for j in i..n-1 {
            for k in j..n {
                if graph[i].contains(&j) && graph[j].contains(&k) && graph[k].contains(&i) {
                    ans += 1;
                } 
            }
        }
    }
    println!("{}", ans);
}

