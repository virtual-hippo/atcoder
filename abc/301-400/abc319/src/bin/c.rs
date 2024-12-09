use itertools::Itertools;
use proconio::{fastout, input};
use std::collections::HashSet;

#[fastout]
fn main() {
    input! {
        c: [[usize; 3]; 3],
    }
    let all = (1..10).fold(1, |sum, v| sum * v);
    let mut rows = vec![vec![]; 3];
    let mut cols = vec![vec![]; 3];
    let mut diag_down = vec![];
    let mut diag_up = vec![];

    for row in (0..3).permutations(3) {
        for col in (0..3).permutations(3) {
            for i in row {
                for j in col {
                    //
                }
            }
        }
    }

    println!("{}", ans);
}
