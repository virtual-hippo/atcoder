use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        s: Chars,
    }

    let mut ans = 0.0;
    for i in 0..s.len() - 1 {
        if s[i] != 't' {
            continue;
        }
        for j in (i + 1..s.len()).rev() {
            if s[j] != 't' {
                continue;
            }
            let len = j - i + 1;
            if len < 3 {
                continue;
            }
            let count = s[i + 1..j].iter().filter(|&&c| c == 't').count();

            let v = count as f64 / (len - 2) as f64;
            if ans < v {
                ans = v;
            }
        }
    }

    println!("{}", ans);
}
