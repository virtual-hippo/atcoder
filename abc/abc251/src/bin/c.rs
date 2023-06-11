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
    }
    let mut map = HashMap::new();
    for i in 0..n {
        input! {
            (s, t): (String, usize),
        }
        if map.contains_key(&s) == false {
            map.insert(s, (i+1, t));
        }
    }
    let ans: (usize, usize) = map.iter().fold((0,0), |ans, (_, (i, t))| if ans.1 < *t {(*i,*t)} else {ans});
    println!("{}", ans.0);
}

