// use std::collections::HashSet;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use std::collections::BinaryHeap;
// use proconio::marker::Chars;
use proconio::input;
use proconio::marker::Chars;
use std::collections::HashSet;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
    }
    // let mut min = 10_000_000;
    // let char_vec = vec!['0','1','2','3','4','5','6','7','8','9'];
    // for ch in char_vec {
    //     //
    // }
    let mut map = HashMap::new();
    for _ in 0..n {
        input! {
            s: Chars,
        }
        for j in 0..10 {
            map.entry(s[j]).or_insert_with(|| vec![]).push(j);
        }
    }

    let mut map2 = HashMap::new();
    for (key, vec) in map.iter_mut() {
        vec.sort();
        let mut set = HashSet::new();
        for i in 0..vec.len() {
            if set.contains(&vec[i]) {
                let mut val = vec[i];
                while set.contains(&val) {
                    val += 10;
                }
                set.insert(val);
            } else {
                set.insert(vec[i]);
            }
        }
        let mut max = 0;
        for val in set.iter() {
            max = std::cmp::max(max, *val);
        }
        map2.insert(key, max);
    }
    let mut ans = 10000;
    for (_, val) in map2.iter_mut() {
        ans = std::cmp::min(ans, *val);
    }
    println!("{}", ans);
}

