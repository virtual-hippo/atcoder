// use std::collections::HashSet;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use std::collections::BinaryHeap;
// use proconio::marker::Chars;
use proconio::input;
use proconio::marker::Chars;
use std::collections::HashMap;

fn main() {
    input! {
        s: Chars,
    }
    let mut map = HashMap::new();
    for i in 0..s.len() {
        *map.entry(s[i]).or_insert(0) += 1;
    }
    let mut soui = true;
    let mut has_large = false;
    let mut has_small = false;
    for (k, v) in map.iter() {
        if *v > 1 {
            soui = false;
        }
        let large_a = 65_u8;
        for i in 0..26 {
            if (large_a + i) as char == *k {
                has_large = true;
            }
        }
        let small_a = 97_u8;
        for i in 0..26 {
            if (small_a + i) as char == *k {
                has_small = true;
            }
        }
    }
    if soui && has_large && has_small {
        println!("Yes");
    } else {
        println!("No");
    }
}

