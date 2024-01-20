use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
    }
    print!("L");
    for i in 0..n {
        print!("o");
    }
    print!("n");
    print!("g");
}
