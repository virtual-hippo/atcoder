// https://atcoder.jp/contests/awc0002/tasks/awc0002_a

use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    }

    let ans = a.iter().position(|&x| x == k).map(|x| x as i64 + 1).unwrap_or(-1);
    println!("{}", ans);
}
