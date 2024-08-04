use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let mut v: Vec<(usize, usize)> = a.iter().enumerate().map(|(i, &v)| (i, v)).collect();
    v.sort_by(|a, b| b.1.cmp(&a.1));
    let ans = v[1].0 + 1;
    println!("{}", ans);
}
