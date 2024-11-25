use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        k: usize,
        mut s: [Chars; h],
    }
    let mut ans = 0;
    for i in 0..h {
        for j in 0..w {
            if s[i][j] == '.' {
                dfs(0, k, (i, j), &mut s, &mut ans);
            }
        }
    }
    println!("{}", ans);
}

fn dfs(cnt: usize, k: usize, pos: (usize, usize), grid: &mut Vec<Vec<char>>, ans: &mut usize) {
    if cnt == k {
        *ans += 1;
        return;
    }

    grid[pos.0][pos.1] = '#';

    if can_move(grid, pos, Dir::UP) {
        dfs(cnt + 1, k, (pos.0 - 1, pos.1), grid, ans);
    }
    if can_move(grid, pos, Dir::DOWN) {
        dfs(cnt + 1, k, (pos.0 + 1, pos.1), grid, ans);
    }
    if can_move(grid, pos, Dir::LEFT) {
        dfs(cnt + 1, k, (pos.0, pos.1 - 1), grid, ans);
    }
    if can_move(grid, pos, Dir::RIGHT) {
        dfs(cnt + 1, k, (pos.0, pos.1 + 1), grid, ans);
    }
    grid[pos.0][pos.1] = '.';
}

enum Dir {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

fn can_move(s: &Vec<Vec<char>>, (i, j): (usize, usize), dir: Dir) -> bool {
    match dir {
        Dir::UP => i > 0 && s[i - 1][j] == '.',
        Dir::DOWN => i < s.len() - 1 && s[i + 1][j] == '.',
        Dir::LEFT => j > 0 && s[i][j - 1] == '.',
        Dir::RIGHT => j < s[0].len() - 1 && s[i][j + 1] == '.',
    }
}
