use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [i64; n],
    }
    let mut s = vec![0; n + 1];
    for i in 0..n {
        s[i + 1] = s[i] + a[i];
    }
    let minimum = s.iter().min().unwrap();
    let start = 0.max(minimum * -1);

    let ans = a.iter().fold(start, |acc, &x| acc + x);

    println!("{}", ans);
}
