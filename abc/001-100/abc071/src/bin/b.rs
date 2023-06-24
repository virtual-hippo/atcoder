use proconio::input;
use proconio::marker::Chars;
use std::collections::HashSet;

fn main() {
    input! {
        s: Chars,
    }
    let mut set = HashSet::new();
    for &ch in s.iter() {
        set.insert(ch);
    }
    let small_a = 97_u8;
    let b = (0..26).map(|i| (small_a + i) as char).filter(|ch| set.contains(ch) == false).collect::<Vec<char>>();
    if b.len() == 0 {
        println!("None");
    } else {
        println!("{}", b[0]);
    }
}

