use proconio::{fastout, input};
use rustc_hash::FxHashMap;

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        s: [String; n],
    }

    let mut ans = 0;
    for bit in 0..1 << n {
        let mut map = FxHashMap::default();
        for i in 0..n {
            if bit & (1 << i) == 0 {
                for ch in s[i].chars() {
                    *map.entry(ch).or_insert(0) += 1;
                }
            }
        }
        let cnt = map.values().filter(|&&x| x == k).count();
        ans = ans.max(cnt);
    }

    println!("{}", ans);
}
