use proconio::input;
use proconio::marker::Chars;
use std::collections::HashMap;

fn main() {
    input! {
        s: Chars,
    }
    let mut table = vec![0; 10];
    let mut map = HashMap::new();
    *map.entry(table.clone()).or_insert(0) += 1;

    for i in 0..s.len() {
        let num = s[i].to_digit(10).unwrap() as usize;
        table[num] += 1;
        table[num] %= 2;
        *map.entry(table.clone()).or_insert(0) += 1;
    }

    let mut ans: u64 = 0;
    for &val in map.values() {
        ans += (val * (val - 1)) / 2;
    }

    println!("{}", ans);
}
