use proconio::{fastout, input, marker::Chars};

fn dfs(visited: &mut Vec<Vec<bool>>, s: &Vec<Vec<char>>, pos: (usize, usize), now: usize) {
    visited[pos.0][pos.1] = true;
    if pos.0 == s.len() - 1 && pos.1 == s[0].len() - 1 {
        return;
    }
    let snuke = ['s', 'n', 'u', 'k', 'e'];
    let next = (now + 1) % 5;

    if pos.0 + 1 < s.len() && s[pos.0 + 1][pos.1] == snuke[next] && !visited[pos.0 + 1][pos.1] {
        dfs(visited, s, (pos.0 + 1, pos.1), next);
    }
    if pos.1 + 1 < s[0].len() && s[pos.0][pos.1 + 1] == snuke[next] && !visited[pos.0][pos.1 + 1] {
        dfs(visited, s, (pos.0, pos.1 + 1), next);
    }
    if pos.0 > 0 && s[pos.0 - 1][pos.1] == snuke[next] && !visited[pos.0 - 1][pos.1] {
        dfs(visited, s, (pos.0 - 1, pos.1), next);
    }
    if pos.1 > 0 && s[pos.0][pos.1 - 1] == snuke[next] && !visited[pos.0][pos.1 - 1] {
        dfs(visited, s, (pos.0, pos.1 - 1), next);
    }
}

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars; h],
    }

    let mut visited = vec![vec![false; w]; h];
    let snuke = ['s', 'n', 'u', 'k', 'e'];
    if let Some(i) = (0..5).find(|&i| snuke[i] == s[0][0]) {
        dfs(&mut visited, &s, (0, 0), i);
    } else {
        println!("No");
        return;
    }
    if visited[h - 1][w - 1] {
        println!("Yes");
    } else {
        println!("No");
    }
}
