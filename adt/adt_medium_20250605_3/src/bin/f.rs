use proconio::{fastout, input, marker::Chars};
use std::collections::HashMap;

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
    }

    let mut s_cols = vec![vec![]; w];
    for _ in 0..h {
        input! {
            row: Chars,
        }

        for j in 0..w {
            s_cols[j].push(row[j]);
        }
    }

    let mut t_cols = vec![vec![]; w];
    for _ in 0..h {
        input! {
            row: Chars,
        }

        for j in 0..w {
            t_cols[j].push(row[j]);
        }
    }

    let mut s_map = HashMap::new();
    for col in s_cols.iter() {
        *s_map.entry(col.clone()).or_insert(0) += 1;
    }

    let mut t_map = HashMap::new();
    for col in t_cols.iter() {
        *t_map.entry(col.clone()).or_insert(0) += 1;
    }

    if s_map == t_map {
        println!("Yes");
    } else {
        println!("No");
    }
}
