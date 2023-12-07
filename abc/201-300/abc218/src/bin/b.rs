use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        p: [u8; 26],
    }
    for i in 0..26 {
        print!("{}", (b'a' + p[i] - 1) as char)
    }
}
