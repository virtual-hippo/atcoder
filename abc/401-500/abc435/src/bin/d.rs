use proconio::{fastout, input, marker::Usize1};

fn dfs(can_reach: &mut Vec<bool>, in_deg: &Vec<Vec<usize>>, v: usize) {
    can_reach[v] = true;
    for &u in in_deg[v].iter() {
        if can_reach[u] == false {
            dfs(can_reach, in_deg, u);
        }
    }
}

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
    }

    let in_deg = {
        let mut in_deg: Vec<Vec<usize>> = vec![vec![]; n];
        for _ in 0..m {
            input! {
                x: Usize1,
                y: Usize1,
            }
            in_deg[y].push(x);
        }
        in_deg
    };

    input! {
        q: usize,
    }
    let mut can_reach = vec![false; n];

    for _ in 0..q {
        input! {
            x: usize,
            v: Usize1,
        }

        match x {
            1 => {
                if !can_reach[v] {
                    dfs(&mut can_reach, &in_deg, v);
                }
            },
            2 => {
                if can_reach[v] {
                    println!("Yes");
                } else {
                    println!("No");
                }
            },
            _ => unreachable!(),
        }
    }
}
