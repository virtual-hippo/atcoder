use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        n: usize,
        s: [Chars; n],
    }

    let w_cnt = s.iter().map(|row| row.iter().filter(|&&c| c == '.').count()).sum::<usize>();

    for i in 0..n {
        for j in 0..n - 5 {
            let cnt = (0..6).map(|k| s[i][j + k]).filter(|&c| c == '.').count();
            if w_cnt > 1 && cnt <= 2 {
                println!("Yes");
                return;
            }
            let cnt = (0..6).map(|k| s[i][j + k]).filter(|&c| c == '#').count();
            if cnt == 6 {
                println!("Yes");
                return;
            }
        }
    }

    for j in 0..n {
        for i in 0..n - 5 {
            let cnt = (0..6).map(|k| s[i + k][j]).filter(|&c| c == '.').count();
            if w_cnt > 1 && cnt <= 2 {
                println!("Yes");
                return;
            }

            let cnt = (0..6).map(|k| s[i + k][j]).filter(|&c| c == '#').count();
            if cnt == 6 {
                println!("Yes");
                return;
            }
        }
    }

    for i in 0..n {
        for j in 0..n {
            if i + 5 < n && j + 5 < n {
                let cnt = (0..6).map(|k| s[i + k][j + k]).filter(|&c| c == '.').count();
                if w_cnt > 1 && cnt <= 2 {
                    println!("Yes");
                    return;
                }

                let cnt = (0..6).map(|k| s[i + k][j + k]).filter(|&c| c == '#').count();
                if cnt == 6 {
                    println!("Yes");
                    return;
                }

                let cnt = (0..6).map(|k| s[i + k][n - (j + k) - 1]).filter(|&c| c == '.').count();
                if w_cnt > 1 && cnt <= 2 {
                    println!("Yes");
                    return;
                }

                let cnt = (0..6).map(|k| s[i + k][n - (j + k) - 1]).filter(|&c| c == '#').count();
                if cnt == 6 {
                    println!("Yes");
                    return;
                }
            }
        }
    }

    println!("No");
}
