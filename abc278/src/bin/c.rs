use proconio::input;
use std::collections::HashSet;
use std::collections::HashMap;

fn main() {
    input! {
        _: usize,
        q: usize,
    }
    let mut map = HashMap::new();
    for _ in 0..q {
        input! {
            t: usize,
            a: usize,
            b: usize,
        }
        if t == 1 {
            map.entry(a).or_insert_with(|| HashSet::new()).insert(b);
        } else if t == 2 {
            if let Some(x) = map.get_mut(&a) {
                x.remove(&b);
            }
        } else {
            let ret1 = if let Some(x) = map.get(&a) {
                x.contains(&b)
            } else {
                false
            };
            let ret2 = if let Some(x) = map.get(&b) {
                x.contains(&a)
            } else {
                false
            };
            if ret1 && ret2 {
                println!("Yes");
            } else {
                println!("No");
            }
        }
    }
}
