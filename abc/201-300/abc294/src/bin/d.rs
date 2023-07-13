use proconio::input;
use std::collections::BTreeSet;

fn main() {
    input! {
        n: usize,
        q: usize,
    }
    let mut b_set = BTreeSet::new();
    let mut hito: Vec<usize> = (1..n + 1).rev().collect();

    for _ in 0..q {
        input! {
            event: usize,
        }
        match event {
            1 => {
                b_set.insert(hito.pop().unwrap());
            }
            2 => {
                input! {x: usize,}
                b_set.remove(&x);
            }
            3 => println!("{}", b_set.iter().next().unwrap()),
            _ => unreachable!(),
        }
    }
}
