use ac_library::*;
use itertools::*;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        a: [u64; n],
        b: [u64; m],
    }

    let mmm = 998244353;

    let a = a.iter().copied().sorted().collect_vec();
    let b = b.iter().copied().sorted().collect_vec();
    // 累積和
    let bs = std::iter::once(0_u64)
        .chain(b.iter().scan(0_u64, |acc, &x| {
            *acc += x;
            Some(*acc)
        }))
        .collect_vec();

    let mut ans = ModInt998244353::new(0);

    let mut j = 0;

    for i in 0..n {
        while j < m && b[j] < a[i] {
            j += 1;
        }

        let (l_cnt, r_cnt) = (j, m - j);
        let (bl, br) = (bs[j], bs[m] - bs[j]);

        let v = (a[i] * (l_cnt as u64) - bl) % mmm + (br - a[i] * (r_cnt as u64)) % mmm;

        ans += v;
    }

    println!("{}", ans);
}
