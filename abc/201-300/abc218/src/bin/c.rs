use proconio::marker::Chars;
use proconio::{fastout, input};

fn rotate(grid: &mut Vec<Vec<char>>) {
    if grid.len() != grid[0].len() {
        panic!()
    }
    let mut new = vec![vec!['.'; grid.len()]; grid.len()];
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            new[j][grid[i].len() - 1 - i] = grid[i][j];
        }
    }

    *grid = new;
}

fn find_left_top(grid: &Vec<Vec<char>>) -> (i32, i32) {
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == '#' {
                return (i as i32, j as i32);
            }
        }
    }
    panic!()
}

fn count(grid: &Vec<Vec<char>>) -> usize {
    let (h, w) = (grid.len(), grid[0].len());
    (0..h)
        .flat_map(|i| (0..w).map(move |j| (i, j)))
        .filter(|&(i, j)| grid[i][j] == '#')
        .count()
}

fn is_ok(s: &Vec<Vec<char>>, t: &Vec<Vec<char>>, n: usize) -> bool {
    let (si, sj) = find_left_top(&s);
    let (ti, tj) = find_left_top(&t);
    let offset = (ti - si, tj - sj);
    for i in 0..n {
        for j in 0..n {
            let (ii, jj) = (i as i32 + offset.0, j as i32 + offset.1);
            if 0 <= ii && ii < n as i32 && 0 <= jj && jj < n as i32 {
                if s[i][j] != t[ii as usize][jj as usize] {
                    return false;
                }
            } else {
                if s[i][j] == '#' {
                    return false;
                }
            }
        }
    }
    true
}

#[fastout]
fn main() {
    input! {
        n: usize,
        s: [Chars; n],
        t: [Chars; n],
    }
    if count(&s) != count(&t) {
        println!("No");
        return;
    }

    let mut cloned_t = t.clone();
    for _ in 0..4 {
        if is_ok(&s, &cloned_t, n) {
            println!("Yes");
            return;
        }
        rotate(&mut cloned_t);
    }
    println!("No");
}
