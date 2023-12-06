use proconio::marker::Chars;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        s: [Chars; 10],
    }
    let (mut a, mut b, mut c, mut d) = (usize::MAX, usize::MAX, usize::MAX, usize::MAX);
    for i in 0..10 {
        for j in 0..10 {
            if a == usize::MAX && s[i][j] == '#' {
                a = i + 1;
                c = j + 1;
                for ii in i..9 {
                    if s[ii + 1][j] == '.' {
                        b = ii + 1;
                        break;
                    }
                }
                if b == usize::MAX {
                    b = 10;
                }
                for jj in j..9 {
                    if s[i][jj + 1] == '.' {
                        d = jj + 1;
                        break;
                    }
                }
                if d == usize::MAX {
                    d = 10;
                }
            }
        }
    }
    println!("{} {}", a, b);
    println!("{} {}", c, d);
}
