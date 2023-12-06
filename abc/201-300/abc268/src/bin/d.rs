use itertools::Itertools;
use std::collections::HashSet;

use proconio::{fastout, input};

fn check_underscore(cnt: usize, src: &mut Vec<String>, t: &HashSet<String>) -> Option<String> {
    if cnt == 0 {
        let current_s = src.join("_");
        if t.contains(&current_s) == false {
            return Some(current_s);
        } else {
            return None;
        }
    }
    for i in 0..src.len() - 1 {
        src[i] += "_";
        if let Some(ans) = check_underscore(cnt - 1, src, t) {
            return Some(ans);
        }
        src[i].pop();
    }
    None
}

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        s: [String; n],
    }
    let mut t = HashSet::new();
    for _ in 0..m {
        input! {
            tt: String,
        }
        t.insert(tt);
    }
    for vec in (0..n).permutations(n) {
        let mut current = vec
            .into_iter()
            .map(|i| s[i].clone())
            .collect::<Vec<String>>();
        let current_s = current.join("_");
        if current_s.len() < 3 || 16 < current_s.len() {
            println!("{}", -1);
            return;
        }
        if t.contains(&current_s) == false {
            println!("{}", current_s);
            return;
        }
        for i in 1..(16 - current_s.len() + 1) {
            if let Some(ans) = check_underscore(i, &mut current, &t) {
                println!("{}", ans);
                return;
            }
        }
    }
    println!("{}", -1);
}
