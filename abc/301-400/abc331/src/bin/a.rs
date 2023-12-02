use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        m: usize,
        d: usize,
        y: usize,
        mm: usize,
        dd: usize,
    }
    if dd < d {
        println!("{} {} {}", y, mm, dd + 1);
    } else {
        if mm < m {
            println!("{} {} {}", y, mm + 1, 1);
        } else {
            println!("{} {} {}", y + 1, 1, 1);
        }
    }
}
