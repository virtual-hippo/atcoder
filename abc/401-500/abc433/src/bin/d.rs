use itertools::*;
use proconio::{fastout, input};
use rustc_hash::FxHashMap;

fn count_digits(n: i64) -> u32 {
    if n < 10 {
        return 1;
    }
    1 + count_digits(n / 10)
}

#[fastout]
fn main() {
    input! {
        n: usize,
        m: i64,
        a: [i64; n],
    }
    let b = a.iter().copied().map(count_digits).collect_vec();

    let mut counter = vec![FxHashMap::default(); 11];
    iproduct!(1..11, 0..n).for_each(|(bi, i)| {
        let ten = 10_i64.pow(bi) % m;
        let k = ((-ten * a[i]) % m + m) % m;
        *counter[bi as usize].entry(k).or_insert(0) += 1_u64;
    });

    let mut ans = 0;
    for j in 0..n {
        let d = a[j] % m;

        if let Some(v) = counter[b[j] as usize].get(&d) {
            ans += *v;
        }
    }

    println!("{}", ans);
}
