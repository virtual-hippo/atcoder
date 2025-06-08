use itertools::*;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        h: usize,
        w: usize,
        ab: [(usize, usize); n],
    }

    let mut grid = vec![0; h];
    if dfs(&ab, h, w, n, &mut grid, &mut 0, (0, 0)) {
        println!("Yes");
    } else {
        println!("No");
    }
}

fn dfs(ab: &Vec<(usize, usize)>, h: usize, w: usize, n: usize, grid: &mut Vec<usize>, used: &mut usize, pos: (usize, usize)) -> bool {
    let nw_bit = (1 << w) - 1;
    if iproduct!(0..h).all(|r| grid[r] == nw_bit) {
        return true;
    }

    for i in 0..n {
        if (*used >> i) & 1 == 1 {
            continue; // すでに使われているタイルはスキップ
        }

        for &(na, nb) in [ab[i], (ab[i].1, ab[i].0)].iter() {
            if pos.0 + na > h || pos.1 + nb > w {
                continue; // タイルがグリッドに収まらない場合はスキップ
            }

            let nb_bit = (1 << nb) - 1;

            // 置けるか確認
            if (pos.0..pos.0 + na).any(|r| (grid[r] & (nb_bit << (w - (pos.1 + nb)))) != 0) {
                continue; // タイルが重なる場合はスキップ
            }
            // タイルを置く
            (pos.0..pos.0 + na).for_each(|r| grid[r] |= nb_bit << (w - (pos.1 + nb)));
            *used |= 1 << i;
            let next_pos_opt = iproduct!(0..h, 0..w).find(|&(r, c)| (grid[r] & (1 << (w - 1 - c))) == 0);
            if let Some(next_pos) = next_pos_opt {
                if dfs(ab, h, w, n, grid, used, next_pos) {
                    return true;
                }
            } else {
                return true;
            }
            // タイルを戻す
            *used &= !(1 << i);
            (pos.0..pos.0 + na).for_each(|r| grid[r] ^= nb_bit << (w - (pos.1 + nb)));
        }
    }

    false
}
