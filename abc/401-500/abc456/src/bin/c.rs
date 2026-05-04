use ac_library::*;
use proconio::{input, marker::*};

fn main() {
    input! {
        s: Chars,
    }

    let n = s.len();
    let mut ans = ModInt998244353::new(0);

    let mut r = 1;
    for l in 0..n {
        r = r.max(l + 1);
        while r < n && s[r - 1] != s[r] {
            r += 1;
        }
        ans += r - l;
    }
    println!("{}", ans);
}
