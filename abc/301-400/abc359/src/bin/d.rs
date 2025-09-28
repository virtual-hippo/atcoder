use ac_library::*;
use proconio::{fastout, input, marker::Chars};
use rustc_hash::FxHashMap;

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        s: Chars,
    }

    // dp[i][t] = i まで決めて直前 k - 1 文字が t である
    let mut dp = FxHashMap::default();
    dp.insert("".to_string(), ModInt998244353::new(1));

    for i in 0..n {
        let mut old = FxHashMap::default();
        std::mem::swap(&mut old, &mut dp);

        for (t, num) in old.iter() {
            for c in 'A'..='B' {
                if s[i] != '?' && s[i] != c {
                    continue;
                }

                let nt = format!("{}{}", t, c);
                if nt.len() == k && is_kaibun(&nt) {
                    continue;
                }

                let nt = if nt.len() == k { nt[1..].to_string() } else { nt };
                *dp.entry(nt).or_insert(ModInt998244353::new(0)) += *num;
            }
        }
    }

    let ans = dp.values().fold(ModInt998244353::new(0), |s, a| s + *a);
    println!("{}", ans);
}

fn is_kaibun(s: &str) -> bool {
    let s_len = s.len();
    for i in 0..s_len / 2 {
        if s.chars().nth(i) != s.chars().nth(s_len - 1 - i) {
            return false;
        }
    }
    true
}
