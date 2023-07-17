use proconio::input;
use std::collections::HashMap;
use std::collections::BTreeSet;

fn main() {
    input! {
        (_h,_w): (usize, usize),
        n: usize,
    }
    let mut a = Vec::with_capacity(n);
    let mut b = Vec::with_capacity(n);
    let mut a_set = BTreeSet::new();
    let mut b_set = BTreeSet::new();
    for _ in 0..n {
        input! {
            ab: (usize, usize),
        }
        a.push(ab.0);
        b.push(ab.1);
        a_set.insert(ab.0);
        b_set.insert(ab.1);
    }

    let mut a_map = HashMap::with_capacity(n);
    let mut b_map = HashMap::with_capacity(n);

    for (i, &v) in a_set.iter().enumerate() {
        a_map.insert(v, i);
    }
    for (i, &v) in b_set.iter().enumerate() {
        b_map.insert(v, i);
    }
    for i in 0..n {
        let a = 1 + a_map.get(&a[i]).unwrap();
        let b = 1 + b_map.get(&b[i]).unwrap(); 
        println!("{} {}", a, b);
    }
}

