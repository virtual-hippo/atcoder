use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        s: Chars,
        t: Chars,
    }
    let mut ii = 0;
    for (i, ch) in t.iter().enumerate() {
        if s[ii] == *ch {
            print!("{} ", i + 1);
            ii += 1;
        }
    }
}
