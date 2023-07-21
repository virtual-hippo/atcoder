use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
    }
    let mut graph = vec![vec![0, 0]; n+1];

    for _ in 0..m {
        input! {
            a: usize,
            b: char,
            c: usize,
            d: char,
        }
        if b == 'R' && d == 'R' {
            graph[a][0] = c;
            graph[c][0] = a;
        } else if b == 'R' && d == 'B' {
            graph[a][0] = c;
            graph[c][1] = a;
        } else if b == 'B' && d == 'R' {
            graph[a][1] = c;
            graph[c][0] = a;
        } else if b == 'B' && d == 'B' {
            graph[a][1] = c;
            graph[c][1] = a;
        }
        
    }

    let mut visited = vec![false; n+1];
    let (mut x, mut y) = (0, 0);
    for i in 1..n+1 {
        if !visited[i] {
            let mut flag = true;
            dfs(&mut visited, &graph, i, &mut flag);
            if flag {
                x += 1;
            } else {
                y += 1;
            }
        }
    }
    println!("{} {}", x, y);
}

fn dfs(visited: &mut Vec<bool>, graph: &Vec<Vec<usize>>, pos: usize, flag: &mut bool) {
    if graph[pos][0] == 0 || graph[pos][1] == 0 {
        *flag = false;
    }
    visited[pos] = true;
    for point in graph[pos].iter() {
        if visited[*point] == false {
            dfs(visited, graph, *point, flag);
        }
    }
}

