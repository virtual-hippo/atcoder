use proconio::input;
use proconio::marker::Chars;

fn mex(x: usize, y: usize, z: usize) -> usize {
    for i in 0..3 {
        if i != x && i != y && i != z {
            return i;
        }
    }
    return 3;
}

fn main() {
    input! {
        n: usize,
        a: [usize; n],
        s: Chars,
    }
    let mut cnt_m = vec![vec![0, 0, 0]; n + 1];
    let mut cnt_x = vec![vec![0, 0, 0]; n + 1];
    for i in 0..n {
        for j in 0..3 {
            cnt_m[i + 1][j] = cnt_m[i][j];
            cnt_x[i + 1][j] = cnt_x[i][j];
        }
        if s[i] == 'M' {
            cnt_m[i + 1][a[i]] += 1;
        }
        if s[i] == 'X' {
            cnt_x[i + 1][a[i]] += 1;
        }
    }

    let mut ans = 0;
    for i in 0..n {
        if s[i] != 'E' {
            continue;
        }
        let current_m = vec![cnt_m[i + 1][0], cnt_m[i + 1][1], cnt_m[i + 1][2]];
        let current_x = vec![
            cnt_x[n][0] - cnt_x[i + 1][0],
            cnt_x[n][1] - cnt_x[i + 1][1],
            cnt_x[n][2] - cnt_x[i + 1][2],
        ];
        for j in 0..3 {
            for k in 0..3 {
                ans += mex(j, a[i], k) * current_m[j] * current_x[k];
            }
        }
    }
    println!("{}", ans);
}

pub fn solve2() {
    input! {
        n: usize,
        a: [usize; n],
        s: Chars,
    }

    const MEX: [char; 3] = ['M', 'E', 'X'];

    let mut init = vec![vec![0; 8]; 4];
    init[0][0] = 1;

    let dp = (0..n).fold(init, |mut dp, i| {
        let j = MEX.iter().position(|&ch| ch == s[i]).unwrap() + 1;

        for k in 0..8 {
            dp[j][k | 1 << a[i]] += dp[j - 1][k];
        }

        dp
    });

    fn mex(k: usize) -> usize {
        (0..).find(|&i| (k >> i) & 1 != 1).unwrap()
    }

    let ans = (0..8).map(|k| dp[3][k] * mex(k)).sum::<usize>();
    println!("{}", ans);
}
