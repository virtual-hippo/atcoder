use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [i64; n],
    }

    let mut ans = n;
    let mut l = 0;
    while l < n - 1 {
        let mut r = l + 1;
        let d = a[r] - a[l];

        while r < n && a[r] - a[r - 1] == d {
            r += 1;
        }

        let len = r - l;
        ans += (1 + len) * len / 2 - len;

        l = r - 1;
    }

    println!("{}", ans);
}
