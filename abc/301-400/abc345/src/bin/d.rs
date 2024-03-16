use proconio::{fastout, input};

fn put_tile(
    (top, left): (usize, usize),
    grid: &mut Vec<Vec<bool>>,
    tile: (usize, usize),
) -> Option<(usize, usize)> {
    for i in top..top + tile.0 {
        for j in left..left + tile.1 {
            if i < grid.len() && j < grid[0].len() && grid[i][j] == false {
            } else {
                return None;
            }
        }
    }

    for i in top..top + tile.0 {
        for j in left..left + tile.1 {
            grid[i][j] = true;
        }
    }

    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == false {
                return Some((i, j));
            }
        }
    }

    Some((0, 0))
}

fn remove_tile((top, left): (usize, usize), grid: &mut Vec<Vec<bool>>, tile: (usize, usize)) {
    for i in top..top + tile.0 {
        for j in left..left + tile.1 {
            grid[i][j] = false;
        }
    }
}

fn solve(
    top_left: (usize, usize),
    grid: &mut Vec<Vec<bool>>,
    ab: &Vec<(usize, usize)>,
    used: &mut Vec<bool>,
) -> bool {
    for i in 0..used.len() {
        if used[i] {
            continue;
        }
        used[i] = true;

        for tile in [ab[i], (ab[i].1, ab[i].0)] {
            if let Some(next_corner) = put_tile(top_left, grid, tile) {
                if next_corner == (0, 0) {
                    return true;
                }
                let ret = solve(next_corner, grid, ab, used);
                if ret == false {
                    remove_tile(top_left, grid, tile);
                } else {
                    return true;
                }
            }
        }
        used[i] = false;
    }
    false
}

#[fastout]
fn main() {
    input! {
        n: usize,
        h: usize,
        w: usize,
        ab: [(usize,usize); n],
    }
    let mut grid = vec![vec![false; w]; h];
    let mut used = vec![false; n];
    let ans = solve((0, 0), &mut grid, &ab, &mut used);
    if ans {
        println!("Yes");
    } else {
        println!("No");
    }
}
