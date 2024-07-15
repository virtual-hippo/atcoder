use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        q: usize,
    }
    let mut a = vec![];
    for _ in 0..q {
        input! {
            qq: usize,
            x: usize,
        }
        if qq == 1 {
            a.push(x);
        } else {
            println!("{}", a[a.len() - x]);
        }
    }
}
