use itertools::Itertools;
use proconio::input;
use std::collections::HashSet;
use std::iter::FromIterator;

fn is_same(
    a: &Vec<Vec<usize>>,
    b: &Vec<Vec<usize>>,
    removed_rows: &HashSet<usize>,
    removed_cols: &HashSet<usize>,
) -> bool {
    let mut d_row = 0;
    for i in 0..b.len() {
        while removed_rows.contains(&(i + d_row)) {
            d_row += 1;
        }
        let mut d_col = 0;
        for j in 0..b[0].len() {
            while removed_cols.contains(&(j + d_col)) {
                d_col += 1;
            }
            if a[i + d_row][j + d_col] != b[i][j] {
                return false;
            }
        }
    }
    true
}

fn main() {
    input! {
        (h1, w1): (usize, usize),
        a: [[usize; w1]; h1],
        (h2, w2): (usize, usize),
        b: [[usize; w2]; h2],
    }
    let diff_h = h1 - h2;
    let diff_w = w1 - w2;

    for removed_rows in (0..h1).combinations(diff_h) {
        let removed_rows_set = HashSet::from_iter(removed_rows.iter().cloned());
        for removed_cols in (0..w1).combinations(diff_w) {
            let removed_cols_set = HashSet::from_iter(removed_cols.iter().cloned());
            if is_same(&a, &b, &removed_rows_set, &removed_cols_set) {
                println!("Yes");
                return;
            }
        }
    }
    println!("No");
}
