use proconio::marker::Chars;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        p: [Chars; n],
    }

    let mut s = vec![vec![0; n + 1]; n + 1];

    for i in 0..n {
        for j in 0..n {
            let val = if p[i][j] == 'B' { 1_i64 } else { 0 };
            s[i + 1][j + 1] = val + s[i + 1][j] + s[i][j + 1] - s[i][j];
        }
    }

    let g = |i: usize, j: usize| -> i64 {
        let mut ret = 0;
        let (dh, dw) = ((i / n) as i64, (j / n) as i64);
        let (mh, mw) = ((i % n), (j % n));
        ret += dh * dw * s[n][n];
        ret += s[mh][n] * dw;
        ret += s[n][mw] * dh;
        ret += s[mh][mw];
        ret
    };

    for _ in 0..q {
        input! {
            (a, b): (usize,usize),
            (c, d): (usize,usize),
        }
        let ans = g(c + 1, d + 1) - g(a, d + 1) - g(c + 1, b) + g(a, b);
        println!("{}", ans);
    }
}
