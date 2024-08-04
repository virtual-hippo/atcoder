use proconio::{fastout, input};

fn solve1() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let mut s = vec![0; n + 1];
    for i in 0..n {
        s[i + 1] = a[i] ^ s[i];
    }

    let n = n + 1;
    let mut ans = 0;

    for k in 0..30 {
        let mut cnt_one = 0;
        for i in 0..n {
            if s[i] >> k & 1 == 1 {
                cnt_one += 1;
            }
        }
        ans += cnt_one * (n - cnt_one) * (1 << k);
    }
    for &v in a.iter() {
        ans -= v;
    }
    println!("{}", ans);
}

fn solve2() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut ans = 0;

    for k in 0..30 {
        let mut dp = vec![vec![0; 2]; 3];
        dp[0][0] = 1;
        for i in 0..n {
            let x = a[i] >> k & 1;
            let mut old = vec![vec![0; 2]; 3];
            (dp, old) = (old, dp);
            for j in 0..3 {
                for kk in 0..2 {
                    for nj in 0..3 {
                        if nj >= j {
                            let mut nk = kk;
                            if nj == 1 && x == 1 {
                                nk ^= 1;
                            }
                            dp[nj][nk] += old[j][kk];
                        }
                    }
                }
            }
        }
        let now = dp[1][1] + dp[2][1];
        ans += now * (1 << k);
    }
    for &v in a.iter() {
        ans -= v;
    }
    println!("{}", ans);
}

#[fastout]
fn main() {
    solve2();
}
