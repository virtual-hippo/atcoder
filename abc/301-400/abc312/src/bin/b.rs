use proconio::marker::Chars;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        s: [Chars; n],
    }
    for i in 0..n - 8 {
        for j in 0..m - 8 {
            let mut flag1 = true;
            for ii in i..i + 3 {
                for jj in j..j + 3 {
                    if s[ii][jj] == '.' {
                        flag1 = false;
                    }
                    if s[ii + 6][jj + 6] == '.' {
                        flag1 = false;
                    }
                }
            }
            if flag1 == false {
                continue;
            }
            let mut flag2 = true;
            for d in 0..4 {
                if s[i + d][j + 3] == '#' {
                    flag2 = false;
                }
                if s[i + d + 5][j + 5] == '#' {
                    flag2 = false;
                }
                if s[i + 3][j + d] == '#' {
                    flag2 = false;
                }
                if s[i + 5][j + d + 5] == '#' {
                    flag2 = false;
                }
            }
            if flag2 == false {
                continue;
            }
            println!("{} {}", i + 1, j + 1);
        }
    }
}
