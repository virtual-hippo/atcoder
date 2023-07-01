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
        s: [String; n],
        t: [String; m],
    }
    let mut map = HashMap::new();
    for i in 0..n {
        map.insert(s[i].as_str(), (i, false));
    }
    for j in 0..m {
        if let Some(v) = map.get_mut(&t[j].as_str()) {
            v.1 = true;
        }
    }
    let mut ans = map.iter().map(|(_, &v)| (v.0, v.1)).collect::<Vec<(usize, bool)>>();
    ans.sort_by(|(a,_),(b,_)| a.cmp(b));
    for i in 0..n {
        if ans[i].1 {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}

