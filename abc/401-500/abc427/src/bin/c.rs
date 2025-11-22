use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        edges: [(Usize1, Usize1); m],
    }

    let mut ans = m;

    for bit in 0..(1 << n) {
        let mut now = 0;
        for i in 0..m {
            let (u, v) = edges[i];
            if ((bit >> u) & 1) == ((bit >> v) & 1) {
                now += 1;
            }
        }
        ans = ans.min(now);
    }

    println!("{}", ans);
}
