use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        s: [Chars; 8],
    }
    let mut row = vec![true; 8];
    let mut col = vec![true; 8];
    for i in 0..8 {
        for j in 0..8 {
            if s[i][j] == '#' {
                row[i] = false;
                col[j] = false;
            }
        }
    }

    let mut ans = 0;
    for i in 0..8 {
        for j in 0..8 {
            if row[i] && col[j] {
                ans += 1;
            }
        }
    }
    println!("{}", ans);
}
