use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        s: Chars,
    }

    let w = s[0] as u8 - b'0' as u8;
    let s = s[2] as u8 - b'0' as u8;

    if s == 8 {
        println!("{}-{}", w + 1, 1);
    } else {
        println!("{}-{}", w, s + 1);
    }
}
