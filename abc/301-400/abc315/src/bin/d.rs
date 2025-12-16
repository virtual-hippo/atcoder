use itertools::*;
use proconio::{fastout, input, marker::Chars};

fn should_be_deleted(m: usize, x: &[usize]) -> bool {
    let tot = x.iter().copied().sum::<usize>();
    let k = (0..m).filter(|&j| x[j] > 0).count();

    tot >= 2 && k == 1
}

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        c: [Chars; h],
    }

    let mut a = vec![Vec::with_capacity(w); h];
    for i in 0..h {
        for j in 0..w {
            let v = (c[i][j] as u8 - b'a') as usize;
            a[i].push(v);
        }
    }

    let m = 26;
    let mut row = vec![vec![0; m]; h];
    let mut col = vec![vec![0; m]; w];

    for i in 0..h {
        for j in 0..w {
            row[i][a[i][j]] += 1;
            col[j][a[i][j]] += 1;
        }
    }

    let mut r_deleted = vec![false; h];
    let mut c_deleted = vec![false; w];

    loop {
        let row_to_be_detleted = (0..h)
            .filter(|&i| !r_deleted[i])
            .filter(|&i| should_be_deleted(m, &row[i]))
            .collect_vec();
        let col_to_be_detleted = (0..w)
            .filter(|&i| !c_deleted[i])
            .filter(|&i| should_be_deleted(m, &col[i]))
            .collect_vec();

        for &i in row_to_be_detleted.iter() {
            for j in 0..w {
                if r_deleted[i] || c_deleted[j] {
                    continue;
                }
                row[i][a[i][j]] -= 1;
                col[j][a[i][j]] -= 1;
            }

            r_deleted[i] = true;
        }

        for &j in col_to_be_detleted.iter() {
            for i in 0..h {
                if r_deleted[i] || c_deleted[j] {
                    continue;
                }
                row[i][a[i][j]] -= 1;
                col[j][a[i][j]] -= 1;
            }

            c_deleted[j] = true;
        }

        if row_to_be_detleted.len() == 0 && col_to_be_detleted.len() == 0 {
            break;
        }
    }

    let ans = iproduct!(0..h, 0..w).filter(|&(i, j)| !(r_deleted[i] || c_deleted[j])).count();
    println!("{}", ans);
}
