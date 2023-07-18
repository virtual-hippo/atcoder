use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        (h,w): (usize, usize),
        s: [Chars; h],
    }
    let mut cnt = 0;
    for i in 0..h {
        for j in 0..w {
            if s[i][j] == '#' {
                cnt += 1;
            }
        }
    }
    println!("{}", cnt);
}

