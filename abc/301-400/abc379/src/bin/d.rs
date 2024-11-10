use proconio::{fastout, input};
use std::collections::HashMap;

#[fastout]
fn main() {
    input! {
        q: usize,
    }

    let mut map = HashMap::new();
    let mut d = 0;

    for _ in 0..q {
        input! {
            n: usize,
        }
        match n {
            1 => *map.entry(d).or_insert(0) += 1,
            2 => {
                input! {
                    t: usize,
                }
                d += t;
            }
            3 => {
                input! {
                    h: usize,
                }
                let mut p = 0;
                let mut b = vec![];
                for (k, v) in map.iter() {
                    if d - *k >= h {
                        p += *v;
                        b.push(*k);
                    }
                }
                for k in b.iter() {
                    map.remove(k);
                }
                println!("{}", p);
            }
            _ => unreachable!(),
        }
    }
}
