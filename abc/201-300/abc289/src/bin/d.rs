use proconio::input;

const UNKNOWN: usize = 0;
const OK: usize = 1;
const MOCHI: usize = 2;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
        m: usize,
        b: [usize; m],
        x: usize,
    }
    let mut dp = vec![UNKNOWN; x+1];
    dp[0] = OK;
    for j in 0..m {
        dp[b[j]] = MOCHI;
    }

    for i in 0..x+1 {
        if dp[i] == OK {
            for j in 0..n {
                let next = i + a[j];
                if next < x+1 && dp[next] == UNKNOWN {
                    dp[next] = OK;
                }
            }
        }
    }

    if dp[x] == OK {
        println!("Yes");
    } else {
        println!("No");
    }
}

