use proconio::{fastout, input};
use std::collections::HashMap;

#[fastout]
fn main() {
    input! {
        s: String,
    }
    let mut map = HashMap::new();
    for ch in s.chars() {
        *map.entry(ch).or_insert(0) += 1;
    }

    let mut map2 = HashMap::new();
    for (&ch, &i) in map.iter() {
        map2.entry(i).or_insert_with(|| vec![]).push(ch);
    }
    for (_, vec) in map2.iter() {
        if vec.len() != 2 {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
