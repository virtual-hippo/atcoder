// use std::collections::HashSet;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use std::collections::BinaryHeap;
// use proconio::marker::Chars;
use proconio::input;
use std::collections::HashMap;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
    }
    let mut map = HashMap::new();
    for _ in 0..n {
        input! {
            mut s: Chars,
        }
        s.sort();
        let string = s.iter().collect::<String>();
        *map.entry(string).or_insert(0_u64) += 1;
    }
    let ans = map.iter().fold(0, |sum, (_, val)| sum + (1..*val).fold(0, |sum2, x| sum2+x));
    println!("{}", ans);
}

