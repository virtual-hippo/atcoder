use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [i64; n],
        b: [i64; n],
    }
    let mut aa: Vec<(i64, usize)> = a.iter().enumerate().map(|(i, &v)| (v, i)).collect();
    let mut bb: Vec<(i64, usize)> = b.iter().enumerate().map(|(i, &v)| (v, i)).collect();
    aa.sort_by(|x, y| y.0.cmp(&x.0));
    bb.sort_by(|x, y| y.0.cmp(&x.0));
    let ans = aa[0].0 + bb[0].0;
    println!("{}", ans);
}
