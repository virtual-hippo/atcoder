use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        s: Chars,
    }
    let mut ans = 0;
    let mut cnt = 0;
    for i in 0..n {
        if s[i] == 'O' {
            cnt += 1;
        } else {
            cnt = 0;
        }
        if cnt == k {
            ans += 1;
            cnt = 0;
        }
    }
    println!("{}", ans);
}
