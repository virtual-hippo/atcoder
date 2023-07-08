use proconio::input;
use std::collections::HashMap;
use std::collections::BTreeSet;

fn main() {
    input! {
        n: usize,
        q: usize,
    }
    let mut box_map = HashMap::with_capacity(n/2);
    let mut card_map = HashMap::with_capacity(n/2);
    for _ in 0..q {
        input! {
            query: usize,
        }
        match query {
            1 => {
                input! {
                    i: usize,
                    j: usize,
                }
                box_map.entry(j).or_insert_with(|| vec![]).push(i);
                card_map.entry(i).or_insert_with(|| BTreeSet::new()).insert(j);
            },
            2 => {
                input! {
                    i: usize,
                }
                let vec = box_map.get_mut(&i).unwrap();
                vec.sort();
                vec.iter().for_each(|&v| print!("{} ", v));
                println!("");
            },
            3 => {
                input! {
                    i: usize,
                }
                let set = card_map.get_mut(&i).unwrap();
                set.iter().for_each(|&v| print!("{} ", v));
                println!("");
            },
            _ => unreachable!(),
        }
    }
}

