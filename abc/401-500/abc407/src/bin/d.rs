use itertools::*;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        a: [[usize; w]; h],
    }

    let indexs = iproduct!(0..h, 0..w).collect::<Vec<(usize, usize)>>();
    let mut ans = 0;
    let mut used = vec![vec![0; w]; h];
    dfs(&indexs, &mut used, &a, (h, w), 0, &mut ans);

    println!("{}", ans);
}

fn dfs(indexs: &Vec<(usize, usize)>, used: &mut Vec<Vec<usize>>, a: &Vec<Vec<usize>>, (h, w): (usize, usize), now: usize, ans: &mut usize) {
    let calc_score = |used: &Vec<Vec<usize>>| -> usize {
        iproduct!(0..h, 0..w)
            .filter(|&(i, j)| used[i][j] == 0)
            .map(|(i, j)| a[i][j])
            .fold(0, |acc, v| acc ^ v)
    };

    if now == indexs.len() {
        let score = calc_score(used);

        if score > *ans {
            *ans = score;
        }

        return;
    }

    let (i, j) = indexs[now];

    // 置かない
    dfs(indexs, used, a, (h, w), now + 1, ans);

    // 縦に置く
    if i < h - 1 && used[i][j] == 0 && used[i + 1][j] == 0 {
        used[i][j] = 1;
        used[i + 1][j] = 1;
        dfs(indexs, used, a, (h, w), now + 1, ans);
        used[i][j] = 0;
        used[i + 1][j] = 0;
    }

    // 横に置く
    if j < w - 1 && used[i][j] == 0 && used[i][j + 1] == 0 {
        used[i][j] = 1;
        used[i][j + 1] = 1;
        dfs(indexs, used, a, (h, w), now + 1, ans);
        used[i][j] = 0;
        used[i][j + 1] = 0;
    }
}
