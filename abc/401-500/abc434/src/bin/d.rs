use itertools::*;
use proconio::{fastout, input, marker::Usize1};

const GRID_SIZE: usize = 2001;

#[fastout]
fn main() {
    input! {
        n: usize,
        rectangles: [(Usize1, Usize1, Usize1, Usize1); n],
    }

    // 2次元いもす法：矩形領域の範囲加算を O(1) で行う
    let imos_grid = {
        let mut grid = vec![vec![0_i64; GRID_SIZE]; GRID_SIZE];
        for &(up, down, left, right) in &rectangles {
            let (start_row, start_col) = (up, left);
            let (end_row, end_col) = (down + 1, right + 1);

            grid[start_row][start_col] += 1;
            grid[end_row][start_col] -= 1;
            grid[start_row][end_col] -= 1;
            grid[end_row][end_col] += 1;
        }
        grid
    };

    // いもす法の累積和を計算して、各マスの重なり数を求める
    let overlap_count = build_2d_cumsum(&imos_grid, GRID_SIZE);

    // 重なりが0のマスの数を計算
    let zero_overlap_count = 2000 * 2000
        - iproduct!(0..GRID_SIZE, 0..GRID_SIZE)
            .filter(|&(i, j)| overlap_count[i + 1][j + 1] != 0)
            .count() as i64;

    // 重なりが1のマスを1、それ以外を0とした配列
    let single_overlap_grid = (0..GRID_SIZE)
        .map(|i| {
            (0..GRID_SIZE)
                .map(|j| if overlap_count[i + 1][j + 1] == 1 { 1 } else { 0 })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    // 重なりが1のマスの累積和
    let single_overlap_cumsum = build_2d_cumsum(&single_overlap_grid, GRID_SIZE);

    // 各矩形について、その範囲内にある「重なりが1のマス」の数を計算
    for &(up, down, left, right) in &rectangles {
        let (start_row, start_col) = (up, left);
        let (end_row, end_col) = (down + 1, right + 1);

        let single_count = query_2d_cumsum(&single_overlap_cumsum, start_row, start_col, end_row, end_col);

        println!("{}", zero_overlap_count + single_count);
    }
}

/// 2次元累積和を構築する
fn build_2d_cumsum(grid: &[Vec<i64>], size: usize) -> Vec<Vec<i64>> {
    let mut cumsum = vec![vec![0_i64; size + 1]; size + 1];
    for i in 0..size {
        for j in 0..size {
            cumsum[i + 1][j + 1] = cumsum[i + 1][j] + cumsum[i][j + 1] - cumsum[i][j] + grid[i][j];
        }
    }
    cumsum
}

/// 2次元累積和を使って矩形領域の合計を求める
fn query_2d_cumsum(cumsum: &[Vec<i64>], start_row: usize, start_col: usize, end_row: usize, end_col: usize) -> i64 {
    cumsum[end_row][end_col] - cumsum[start_row][end_col] - cumsum[end_row][start_col] + cumsum[start_row][start_col]
}
