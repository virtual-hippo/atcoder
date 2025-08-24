use ac_library::*;
use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
    }

    let mut dsu = Dsu::new(n);
    let mut colors = vec![0; n];
    let mut nodes = vec![0; n];

    for _ in 0..q {
        input! {
            q: usize,
        }

        match q {
            1 => {
                input! {
                    u: Usize1,
                    v: Usize1,
                }
                let leader_u = dsu.leader(u);
                let leader_v = dsu.leader(v);

                if leader_u == leader_v {
                    continue;
                }
                dsu.merge(u, v);

                let leader = dsu.leader(u);

                nodes[leader] = nodes[leader_u] + nodes[leader_v];
                nodes[leader_u ^ leader_v ^ leader] = 0;
            },
            2 => {
                input! {
                    u: Usize1,
                }
                colors[u] ^= 1;
                let leader = dsu.leader(u);
                if colors[u] == 0 {
                    nodes[leader] -= 1;
                } else {
                    nodes[leader] += 1;
                }
            },
            3 => {
                input! {
                    u: Usize1,
                }
                let leader = dsu.leader(u);
                if nodes[leader] == 0 {
                    println!("No");
                } else {
                    println!("Yes");
                }
            },
            _ => unreachable!(),
        }
    }
}
