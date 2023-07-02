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
        c: [String; n],
        d: [String; m],
        p: [usize; m+1],
    }
    let mut map = HashMap::new();
    for i in 0..m {
        map.insert(d[i].as_str(), p[i+1]);
    }
    let mut ans = 0;
    for i in 0..n {
        if let Some(v) = map.get(&c[i].as_str()) {
            ans += *v;
        } else {
            ans += p[0];
        }
    }
    println!("{}", ans);
}

