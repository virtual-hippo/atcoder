#![allow(unused_imports)]
use ac_library::FenwickTree;
use proconio::{fastout, input, marker::Chars};
use std::collections::{HashMap, HashSet};

#[fastout]
fn main() {
    input! {
        mut a: [usize; 3],
    }
    a.sort();
    if a[0] + a[1] == a[2] {
        println!("Yes");
    } else if a[0] == a[1] && a[1] == a[2] {
        println!("Yes");
    } else {
        println!("No");
    }
}
