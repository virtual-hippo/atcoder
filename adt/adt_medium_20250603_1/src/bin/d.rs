use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        n: usize,
        d: usize,
        mut s: Chars,
    }

    let mut d = d;
    for i in (0..n).rev() {
        if d > 0 && s[i] == '@' {
            d -= 1;
            s[i] = '.';
        }
    }

    let ans = s.iter().collect::<String>();
    println!("{}", ans);
}
