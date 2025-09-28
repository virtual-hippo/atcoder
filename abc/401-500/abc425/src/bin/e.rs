use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        t: usize,
        m: u64,
    }
    // パスカルの三角形を用いて nCk を前計算しておく
    // nCk = n-1Ck + n-1Ck-1
    let binom = {
        let s = 5000;
        let mut binom = vec![vec![0; s + 1]; s + 1];
        binom[0][0] = 1;

        for i in 0..s {
            for j in 0..i + 1 {
                binom[i + 1][j] += binom[i][j] % m;
                binom[i + 1][j + 1] += binom[i][j] % m;
            }
        }
        binom
    };
    for _ in 0..t {
        solve(m, &binom);
    }
}

fn solve(m: u64, binom: &Vec<Vec<u64>>) {
    input! {
        n: usize,
        c: [usize; n],
    }

    let mut ans = 1;
    let mut length = 0;

    for i in 0..n {
        length += c[i];
        ans *= binom[length][c[i]];
        ans %= m;
    }

    println!("{}", ans);
}
