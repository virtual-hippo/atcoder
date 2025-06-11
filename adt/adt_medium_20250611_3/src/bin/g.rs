use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
    }

    let mut bucket = vec![0; n + 1];
    for _ in 0..m {
        input! {
            a: usize,
            b: usize,
        }
        bucket[(a + b) % n] += 1;
    }

    let all = (m * (m - 1)) / 2;
    let eq = bucket.iter().filter(|&&x| x > 1).map(|&x| (x * (x - 1)) / 2).sum::<usize>();

    let ans = all - eq;
    println!("{}", ans);
}
