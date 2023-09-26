// use std::collections::HashSet;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use std::collections::BinaryHeap;
// use proconio::marker::Chars;
use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        m: usize,
        t: usize,
        a: [usize; n-1],
    }
    let mut bonus = HashMap::with_capacity(m);
    for _ in 0..m {
        input! {
            x: usize,
            y: usize,
        }
        bonus.insert(x, y);
    }
    let mut current = t;
    for i in 0..n - 1 {
        if let Some(y) = bonus.get(&(i + 1)) {
            current += y;
        }
        if current <= a[i] {
            println!("{}", "No");
            return;
        }
        current -= a[i];
    }
    println!("{}", "Yes");
}
