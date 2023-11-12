use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        q: usize,
        s: Chars,
    }
    let mut cur = 0;
    let mut vec = vec![0; n];
    for i in 1..n {
        if s[i] == s[i - 1] {
            cur += 1;
        }
        vec[i] = cur;
    }

    for _ in 0..q {
        input! {
            l: usize,
            r: usize,
        }

        println!("{}", vec[r - 1] - vec[l - 1]);
    }
}
