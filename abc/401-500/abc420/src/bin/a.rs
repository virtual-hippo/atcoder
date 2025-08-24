use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        x: usize,
        y: usize,
    }

    let ans = (x + y) % 12;
    if ans == 0 {
        println!("{}", 12);
    } else {
        println!("{}", ans);
    }
}
