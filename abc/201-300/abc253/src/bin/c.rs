// use std::collections::HashSet;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use std::collections::BinaryHeap;
// use proconio::marker::Chars;
use proconio::input;
use std::collections::BTreeMap;

fn main() {
    input! {
        q: usize,
    }
    let mut map = BTreeMap::new();
    for _ in 0..q {
        input! {
            query: usize,
        }
        match query {
            1 => {
                input! {
                    x: usize,
                }
                *map.entry(x).or_insert(0) += 1;
            }
            2 => {
                input! {
                    x: usize,
                    c: usize,
                }
                if let Some(val) = map.get_mut(&x) {
                    if *val > c {
                        *val -= c;
                    } else {
                        map.remove(&x);
                    }
                }
            }
            3 => {
                let max = map.iter().last().unwrap().0;
                let min = map.iter().next().unwrap().0;
                println!("{}", max - min);
            }
            _ => unreachable!()
        }
    }
}

