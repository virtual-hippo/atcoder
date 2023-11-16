use itertools::Itertools;
use proconio::input;
use proconio::marker::Chars;
use std::collections::HashSet;

fn check(n: usize, grid: &mut Vec<Vec<char>>, c: &Vec<char>) -> bool {
    for i in 0..n {
        let mut set = HashSet::new();
        let mut is_init = true;
        for j in 0..n {
            if grid[j].len() == 0 {
                break;
            }
            if grid[j][i] != '.' && is_init {
                is_init = false;
                if grid[j][i] != c[i] {
                    return false;
                }
            }
            if grid[j][i] != '.' && set.contains(&grid[j][i]) {
                return false;
            }
            set.insert(grid[j][i]);
        }
    }
    true
}

fn dfs(
    n: usize,
    grid: &mut Vec<Vec<char>>,
    used: &mut Vec<Vec<bool>>,
    koho: &Vec<Vec<Vec<char>>>,
    c: &Vec<char>,
    tail: usize,
) -> bool {
    if tail == n {
        println!("Yes");
        for i in 0..n {
            for j in 0..n {
                print!("{}", grid[i][j]);
            }
            println!();
        }
        return true;
    }

    for i in 0..koho[tail].len() {
        if used[tail][i] == true {
            continue;
        }
        used[tail][i] = true;
        for j in 0..n {
            grid[tail].push(koho[tail][i][j]);
        }
        if check(n, grid, c) {
            if dfs(n, grid, used, koho, c, tail + 1) {
                return true;
            }
        }
        grid[tail].clear();
        used[tail][i] = false;
    }
    false
}

fn main() {
    input! {
        n: usize,
        r: Chars,
        c: Chars,
    }
    let abc = {
        let mut ret = vec!['A', 'B', 'C'];
        for _ in 0..(n - 3) {
            ret.push('.');
        }
        ret
    };
    let mut koho = vec![vec![]; n];
    let mut used = vec![vec![]; n];
    {
        let mut it = 0;
        while it < n {
            for perm in (0..n).permutations(n) {
                let current = perm.iter().map(|&i| abc[i]).collect::<Vec<char>>();
                let pos = current.iter().position(|&ch| ch != '.').unwrap();
                if current[pos] == r[it] {
                    koho[it].push(current);
                    used[it].push(false);
                }
            }
            it += 1;
        }
    }
    let mut grid = vec![vec![]; n];
    if dfs(n, &mut grid, &mut used, &koho, &mut &c, 0) {
        return;
    }
    println!("No");
}
