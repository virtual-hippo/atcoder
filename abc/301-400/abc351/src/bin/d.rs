use proconio::{fastout, input, marker::Chars};
use std::collections::{HashSet, VecDeque};

fn bfs(
    visited: &mut Vec<Vec<bool>>,
    grid: &Vec<Vec<char>>,
    start: (usize, usize),
    (h, w): (usize, usize),
) -> usize {
    let mut cnt = 1;

    let mut queue = VecDeque::new();
    let mut set = HashSet::new();

    queue.push_back(start);
    set.insert(start);
    while queue.is_empty() == false {
        let pos = queue.pop_front().unwrap();
        visited[pos.0][pos.1] = true;

        // can't move
        if 0 < pos.0 && grid[pos.0 - 1][pos.1] == '#' {
            continue;
        }
        if pos.0 < h - 1 && grid[pos.0 + 1][pos.1] == '#' {
            continue;
        }
        if 0 < pos.1 && grid[pos.0][pos.1 - 1] == '#' {
            continue;
        }
        if pos.1 < w - 1 && grid[pos.0][pos.1 + 1] == '#' {
            continue;
        }

        // move
        let nexts = {
            let mut ret = vec![];
            if 0 < pos.0 {
                ret.push((pos.0 - 1, pos.1));
            }
            if pos.0 < h - 1 {
                ret.push((pos.0 + 1, pos.1));
            }
            if 0 < pos.1 {
                ret.push((pos.0, pos.1 - 1));
            }
            if pos.1 < w - 1 {
                ret.push((pos.0, pos.1 + 1));
            }
            ret
        };

        for to in nexts.iter() {
            if set.contains(to) == false {
                set.insert(*to);
                queue.push_back(*to);
                cnt += 1;
            }
        }
    }
    cnt
}

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars; h],
    }
    let mut visited = vec![vec![false; w]; h];

    let mut ans = 0;
    for i in 0..h {
        for j in 0..w {
            if s[i][j] == '#' || visited[i][j] {
                continue;
            }
            ans = ans.max(bfs(&mut visited, &s, (i, j), (h, w)));
        }
    }
    println!("{}", ans);
}
