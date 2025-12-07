use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut r = a[0];
    let mut cnt = 1;

    for i in 1..n {
        if i < r {
            r = r.max(i + a[i]);
            cnt += 1;
        } else {
            break;
        }
    }

    let ans = cnt;
    println!("{}", ans);
}
