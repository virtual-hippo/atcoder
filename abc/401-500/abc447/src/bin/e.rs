use ac_library::*;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize,
        m: usize,
        uv: [(Usize1,Usize1); m],
    }

    let mut dsu = Dsu::new(n);
    let mut cnt = n;
    let mut ans = ModInt998244353::new(0);

    for i in (0..m).rev() {
        let (u, v) = uv[i];
        if cnt == 2 && !dsu.same(u, v) {
            ans += ModInt998244353::new(2).pow((i + 1) as u64);
            continue;
        }

        if !dsu.same(u, v) {
            cnt -= 1;
        }

        dsu.merge(u, v);
    }

    println!("{}", ans);
}
