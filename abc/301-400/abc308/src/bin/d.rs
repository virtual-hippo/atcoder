use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        (h,w): (usize, usize),
        s: [Chars; h],
    }
    let snuke = vec!['s', 'n', 'u', 'k', 'e'];
    let len_snuke = snuke.len() as i64;
    let mut dp = vec![vec![-1_i64; w + 1]; h + 1];

    if s[0][0] == 's' {
        dp[0][0] = 0;
    } else {
        println!("No");
        return;
    }

    let mut i = 0;
    while i < h {
        let mut j = 0;
        let mut is_modoru_i = false;
        while j < w {
            let mut is_modoru_j = false;
            if dp[i][j] == -1 {
                j += 1;
                continue;
            }
            if i < h - 1 {
                let next = s[i + 1][j];
                if next == snuke[((dp[i][j] + 1) % len_snuke) as usize] {
                    dp[i + 1][j] = dp[i][j] + 1;
                }
            }
            if j < w - 1 {
                let next = s[i][j + 1];
                if next == snuke[((dp[i][j] + 1) % len_snuke) as usize] {
                    dp[i][j + 1] = dp[i][j] + 1;
                }
            }

            if 0 < i {
                let next = s[i - 1][j];
                if next == snuke[((dp[i][j] + 1) % len_snuke) as usize] {
                    if dp[i-1][j] == -1 {
                        dp[i - 1][j] = dp[i][j] + 1;
                        is_modoru_i = true;
                    }
                }
            }
            if 0 < j {
                let next = s[i][j - 1];
                if next == snuke[((dp[i][j] + 1) % len_snuke) as usize] {
                    if dp[i][j-1] == -1 {
                        dp[i][j - 1] = dp[i][j] + 1;
                        is_modoru_j = true;
                    }
                }
            }
            if is_modoru_j {
                j -= 1;
            } else {
                j += 1;
            }
        }
        if is_modoru_i {
            i -= 1;
        } else {
            i += 1;
        }
    }

    if dp[h - 1][w - 1] != -1 {
        println!("Yes");
    } else {
        println!("No");
    }
}
