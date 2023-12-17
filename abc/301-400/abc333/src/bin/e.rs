use std::collections::HashMap;

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        tx: [(usize, usize); n],
    }
    let mut map = HashMap::new();
    let mut kodo = vec![];

    for i in (0..n).rev() {
        if tx[i].0 == 2 {
            *map.entry(tx[i].1).or_insert(0) += 1;
        } else {
            if let Some(val) = map.get_mut(&tx[i].1) {
                *val -= 1;
                if *val == 0 {
                    map.remove(&tx[i].1);
                }
                kodo.push(1);
            } else {
                kodo.push(0);
            }
        }
    }
    if map.len() != 0 {
        println!("-1");
        return;
    }
    let kodo = kodo.into_iter().rev().collect::<Vec<usize>>();
    let mut current = 0;
    let mut k = 0;
    let mut it = 0;
    for i in 0..n {
        if tx[i].0 == 1 {
            if kodo[it] == 1 {
                current += 1;
            }
            it += 1;
        } else {
            current -= 1;
        }
        k = k.max(current);
    }
    println!("{}", k);
    for i in 0..kodo.len() {
        print!("{} ", kodo[i]);
    }
}
