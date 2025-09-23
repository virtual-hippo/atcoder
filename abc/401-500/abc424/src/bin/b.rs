use proconio::{fastout, input};
use std::collections::HashSet;

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
    }

    let mut counter = vec![HashSet::new(); n];
    let mut has_output = HashSet::new();

    for _ in 0..k {
        input! {
            a: usize,
            b: usize,
        }

        let a = a - 1;
        let b = b - 1;
        counter[a].insert(b);

        if counter[a].len() == m && !has_output.contains(&a) {
            print!("{} ", a + 1);
            has_output.insert(a);
        }
    }
}
