use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        p: usize,
        q: usize,
        x: usize,
        y: usize,
    }

    if p <= x && x < p + 100 && q <= y && y < q + 100 {
        println!("Yes");
    } else {
        println!("No");
    }
}
