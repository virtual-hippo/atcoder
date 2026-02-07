use itertools::*;
use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        s: Chars,
    }

    if s.iter().unique().count() == 1 {
        println!("Yes");
    } else {
        println!("No");
    }
}
