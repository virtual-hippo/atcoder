use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; n],
    }

    let s = a.iter().sum::<usize>();
    if s <= m {
        println!("Yes");
    } else {
        println!("No");
    }
}
