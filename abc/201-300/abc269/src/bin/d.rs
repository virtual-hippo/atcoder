use proconio::{fastout, input};

fn dfs(visited: &mut Vec<Vec<bool>>, boared: &Vec<Vec<bool>>, pos: (usize, usize)) {
    visited[pos.0][pos.1] = true;
    let vec = vec![(-1, -1), (-1, 0), (0, -1), (0, 1), (1, 0), (1, 1)];
    for (i, j) in vec.into_iter() {
        let (ii, jj) = ((pos.0 as i32 + i) as usize, (pos.1 as i32 + j) as usize);
        if boared[ii][jj] && visited[ii][jj] == false {
            dfs(visited, boared, (ii, jj));
        }
    }
}

#[fastout]
fn main() {
    input! {
        n: usize,
    }
    let mut boared = vec![vec![false; 2004]; 2004];
    for _ in 0..n {
        input! {
            x: i32,
            y: i32,
        }
        boared[(x + 1002) as usize][(y + 1002) as usize] = true;
    }
    let mut visited = vec![vec![false; 2004]; 2004];
    let mut ans = 0;
    for i in 0..2004 {
        for j in 0..2004 {
            if boared[i][j] && visited[i][j] == false {
                dfs(&mut visited, &boared, (i, j));
                ans += 1;
            }
        }
    }
    println!("{}", ans);
}
