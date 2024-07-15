use itertools::{iproduct, Itertools};
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        (h,w): (usize, usize),
        a: [[usize; w]; h],
        b: [[usize; w]; h],
    }
    if a == b {
        println!("{}", 0);
        return;
    }

    let mut ans = usize::MAX;

    for vec_row in (0..h).permutations(h) {
        for vec_col in (0..w).permutations(w) {
            let is_ok = iproduct!(0..h, 0..w)
                .find(|&(i, j)| a[vec_row[i]][vec_col[j]] != b[i][j])
                .is_none();
            if !is_ok {
                continue;
            }
            let row_cnt = (0..h)
                .tuple_combinations()
                .filter(|&(i, j)| vec_row[i] > vec_row[j])
                .count();
            let col_cnt = (0..w)
                .tuple_combinations()
                .filter(|&(i, j)| vec_col[i] > vec_col[j])
                .count();
            ans = ans.min(row_cnt + col_cnt);
        }
    }
    if ans != usize::MAX {
        println!("{}", ans);
    } else {
        println!("{}", -1);
    }
}
