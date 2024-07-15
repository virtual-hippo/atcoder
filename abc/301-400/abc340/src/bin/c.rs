use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
    }
    let mut cnt = 1;
    while 2_usize.pow(cnt) < n {
        cnt += 1;
    }
    cnt -= 1;

    let v = n * (cnt as usize);
    let ans = v + 2 * (n - 2_usize.pow(cnt));
    println!("{}", ans);
}
