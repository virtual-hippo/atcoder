use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        n: usize,
        s: [Chars; n],
    }
    let m = {
        let mut ret = 0;
        for i in 0..n {
            ret = ret.max(s[i].len());
        }
        ret
    };
    let mut t = vec![vec!['*'; n]; m];
    for i in 0..n {
        for (j, &ch) in s[i].iter().enumerate() {
            t[j][n - i - 1] = ch;
        }
    }

    for i in 0..m {
        for j in (0..n).rev() {
            if t[i][j] == '*' {
                t[i][j] = '_';
            } else {
                break;
            }
        }
    }

    for i in 0..m {
        for j in 0..n {
            if t[i][j] == '_' {
                continue;
            }
            print!("{}", t[i][j]);
        }
        println!("");
    }
}
