use proconio::input;

fn dfs(colors: &mut Vec<usize>, graph: &Vec<Vec<usize>>, pos: usize) -> (usize, usize){
    let mut ret = (0, 0);

    for point in graph[pos].iter() {
        if colors[*point] == std::usize::MAX {
            colors[*point] = 1 - colors[pos];
            let pp = dfs(colors, graph, *point);
            if pp.0 == 0 && pp.1 == 0 {
                return (0, 0);
            }
            ret.0 += pp.0;
            ret.1 += pp.1;
        } else if colors[*point] == colors[pos] {
            return (0, 0);
        }
    }

    if colors[pos] == 0 {
        ret.0 += 1;
    } else {
        ret.1 += 1;
    }

    ret
}

fn main() {
    input! {
        n: usize,
        m: usize,
    }
    let mut graph = vec![vec![]; n+1];
    for _ in 0..m {
        input! {
            u: usize,
            v: usize,
        }
        graph[u].push(v);
        graph[v].push(u);
    }

    let mut ans = ((n * (n-1))/ 2) - m;

    let mut colors = vec![std::usize::MAX; n+1];
    for i in 1..n+1 {
        if colors[i] == std::usize::MAX {
            colors[i] = 0;
            let ret = dfs(&mut colors, &graph, i);
            if ret.0 == 0 {
                println!("{}", 0);
                return;
            }
            ans -= ret.0 * (ret.0 - 1) / 2;
            ans -= ret.1 * (ret.1 - 1) / 2;
        }
    }
    println!("{}", ans);
}

