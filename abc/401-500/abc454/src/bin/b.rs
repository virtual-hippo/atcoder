use itertools::*;
use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        f: [usize; n],
    }

    let count = f.iter().copied().unique().count();
    if count == n {
        println!("Yes");
    } else {
        println!("No");
    }
    if count >= m {
        println!("Yes");
    } else {
        println!("No");
    }
}
