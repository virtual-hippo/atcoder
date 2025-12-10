use proconio::{fastout, input};

fn solve(_: usize) {
    input! {
        a: usize,
        b: usize,
        c: usize,
    }

    let ans = a.min(c).min((a + b + c) / 3);
    println!("{}", ans);
}

#[fastout]
fn main() {
    input! {
        t: usize,
    }
    (0..t).for_each(solve);
}
