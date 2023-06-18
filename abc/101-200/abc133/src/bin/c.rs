// use std::collections::HashSet;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use std::collections::BinaryHeap;
// use proconio::marker::Chars;
use proconio::input;

fn main() {
    input! {
        l: usize,
        r: usize,
    }
    let ini_val = l % 2019;
    let mut koho = vec![];
    koho.push(ini_val);

    let mut current = l+1;
    while current <= r && current % 2019 != ini_val {
        koho.push(current % 2019);
        current += 1;
    }
    let mut ans = 2019;
    for i in 0..koho.len()-1 {
        for j in i+1..koho.len() {
            ans = std::cmp::min(ans, (koho[i] * koho[j]) % 2019);
        }
    }
    println!("{}", ans);
}

