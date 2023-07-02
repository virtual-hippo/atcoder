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
        st: [(String, String); n]
    }
    let mut map = HashMap::with_capacity(n);
    let mut is_checked = HashMap::with_capacity(n);
    for i in 0..n {
        map.insert(st[i].0.as_str(), st[i].1.as_str());
        is_checked.insert(st[i].0.as_str(), false);
    }


    for (&k, &v) in map.iter() {
        let mut find = v;
        if let Some(val) = is_checked.get_mut(&k) {
            if *val {
                continue;
            } else {
                *val = true;
            }
        }
        while let Some(&v) = map.get(&find) {
            if let Some(val) = is_checked.get_mut(&find) {
                *val = true;
            }
            if v == k {
                println!("No");
                return;
            }
            find = v;
        }
    }

    println!("Yes");
}

// use std::collections::HashSet;
// fn main() {
//     input! {
//         n: usize,
//         st: [(String, String); n],
//     }
//     let mut s_set = HashSet::with_capacity(n);
//     let mut t_set = HashSet::with_capacity(n);
//     for i in 0..n {
//         s_set.insert(st[i].0.as_str());
//         t_set.insert(st[i].1.as_str());
//     }

//     for i in 0..n {
//         let s = st[i].0.as_str();
//         let t = st[i].1.as_str();
//         if s_set.contains(&t) && t_set.contains(&s) {
//             s_set.remove(&t);
//             t_set.remove(&s);
//         }
//     }

//     if s_set.len() == t_set.len() && s_cnt == n{
//         println!("No");
//     } else {
//         println!("Yes");
//     }
// }


