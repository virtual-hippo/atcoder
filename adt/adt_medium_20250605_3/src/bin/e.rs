use proconio::{fastout, input};
use std::collections::{BTreeMap, BTreeSet};

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
    }

    let mut boxes = vec![BTreeMap::<usize, usize>::new(); n + 1];
    let mut cards = vec![BTreeSet::<usize>::new(); 200_001];

    for _ in 0..q {
        input! {
            t: usize,
        }

        match t {
            1 => {
                input! {
                    i: usize,
                    j: usize,
                }
                *boxes[j].entry(i).or_insert(0) += 1;
                cards[i].insert(j);
            },
            2 => {
                input! {
                    i: usize,
                }
                for (&k, &v) in boxes[i].iter() {
                    for _ in 0..v {
                        print!("{} ", k);
                    }
                }
                println!();
            },
            3 => {
                input! {
                    i: usize,
                }
                for v in cards[i].iter() {
                    print!("{} ", v);
                }
                println!();
            },
            _ => unreachable!(),
        }
    }
}
