use proconio::input;
use std::collections::HashMap;
use std::collections::HashSet;

fn solve(
    n: &usize,
    t: &usize,
    hate: &HashMap<usize, Vec<usize>>,
    teams: &mut Vec<HashSet<usize>>,
    now: usize,
) -> usize {
    if now == *n {
        return if teams.len() == *t { 1 } else { 0 };
    }

    let mut ans = 0;

    for team_index in 0..teams.len() {
        let is_contain_hated = hate
            .get(&now)
            .unwrap()
            .iter()
            .filter(|&hito| teams[team_index].contains(hito))
            .count()
            > 0;
        if is_contain_hated {
            continue;
        }
        teams[team_index].insert(now);
        ans += solve(&n, &t, &hate, teams, now + 1);
        teams[team_index].remove(&now);
    }

    if teams.len() < *t {
        let mut set = HashSet::with_capacity(*n);
        set.insert(now);
        teams.push(set);
        ans += solve(&n, &t, &hate, teams, now + 1);
        teams.pop();
    }

    ans
}

fn main() {
    input! {
        n: usize,
        t: usize,
        m: usize,
    }
    let mut hate = HashMap::with_capacity(n);
    for i in 0..n {
        hate.insert(i, vec![]);
    }
    for _ in 0..m {
        input! {
            a: usize,
            b: usize,
        }
        hate.entry(a - 1).or_insert_with(|| vec![]).push(b - 1);
        hate.entry(b - 1).or_insert_with(|| vec![]).push(a - 1);
    }
    let mut teams = Vec::with_capacity(t);
    println!("{}", solve(&n, &t, &hate, &mut teams, 0));
}
