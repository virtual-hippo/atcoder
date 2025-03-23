// 解説AC

#![allow(unused_imports)]
use ac_library::*;
use itertools::*;
use proconio::{fastout, input, marker::Chars};
use rustc_hash::{FxHashMap, FxHashSet};
use std::collections::VecDeque;
use superslice::Ext;

#[fastout]
fn main() {
    input! {
        s: String,
    }
    // solve0::solve0(&s);
    solve1::solve1(&s);
    // solve2::solve2(&s);
}

#[warn(dead_code)]
mod rolling_hash {
    use rand::Rng;
    use std::cmp::PartialEq;
    use std::ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign};

    ///
    /// Mint
    ///
    #[derive(Copy, Clone)]
    pub struct Mint {
        v0: u64,
        v1: u64,
    }
    impl Mint {
        const M0: u64 = 1_000_000_021;
        const M1: u64 = 1_000_000_033;

        pub fn new(v0: u64, v1: u64) -> Mint {
            Mint {
                v0: v0 % Self::M0,
                v1: v1 % Self::M1,
            }
        }
    }

    impl Add for Mint {
        type Output = Mint;
        fn add(self, rhs: Self) -> Self::Output {
            Self::new(self.v0 + rhs.v0, self.v1 + rhs.v1)
        }
    }

    impl Sub for Mint {
        type Output = Mint;
        fn sub(self, rhs: Self) -> Self::Output {
            Self::new(Self::M0 + self.v0 - rhs.v0, Self::M1 + self.v1 - rhs.v1)
        }
    }

    impl Mul for Mint {
        type Output = Mint;
        fn mul(self, rhs: Self) -> Self::Output {
            Self::new(self.v0 * rhs.v0, self.v1 * rhs.v1)
        }
    }

    impl PartialEq for Mint {
        fn eq(&self, other: &Self) -> bool {
            self.v0 == other.v0 && self.v1 == other.v1
        }
    }

    ///
    /// RollingHash
    /// ハッシュが衝突しないために 2 つの m を保持する Mint を使う
    ///
    pub struct RollingHash {
        hash: Vec<Mint>,
        hash_rev: Vec<Mint>,
        power: Vec<Mint>,
    }

    impl RollingHash {
        pub fn new(s: &[u8]) -> Self {
            let n = s.len();
            let mut rng = rand::prelude::ThreadRng::default();
            let randam_value0 = rng.gen_range(2..8197);
            let randam_value1 = rng.gen_range(2..8197);
            let base = Mint::new(randam_value0, randam_value1);

            let mut hash = vec![Mint::new(0, 0); n + 1];
            let mut hash_rev = vec![Mint::new(0, 0); n + 1];
            let mut power = vec![Mint::new(1, 1); n + 1];

            for i in 0..n {
                let c = Mint::new(s[i] as u64, s[i] as u64);
                hash[i + 1] = hash[i] * base + c;

                let rev_c = Mint::new(s[n - 1 - i] as u64, s[n - 1 - i] as u64);
                hash_rev[i + 1] = hash_rev[i] * base + rev_c;

                power[i + 1] = power[i] * base;
            }
            Self {
                hash,
                hash_rev,
                power,
            }
        }

        /// S[l, r) のハッシュを求める
        pub fn get_hash(&self, l: usize, r: usize) -> Mint {
            self.hash[r] - self.power[r - l] * self.hash[l]
        }

        /// rev_S[l, r) のハッシュを求める
        pub fn get_rev_hash(&self, l: usize, r: usize) -> Mint {
            let n = self.hash_rev.len() - 1;
            let (l, r) = (n - r, n - l);
            self.hash_rev[r] - self.power[r - l] * self.hash_rev[l]
        }

        /// 回文判定を行う
        pub fn is_palindrome(&self, l: usize, r: usize) -> bool {
            self.get_hash(l, r) == self.get_rev_hash(l, r)
        }
    }
}

#[warn(dead_code)]
mod solve0 {
    use super::*;
    // ローリングハッシュ による解法
    pub(super) fn solve0(s: &String) {
        let n = s.len();
        let rh = rolling_hash::RollingHash::new(s.as_bytes());
        for l in 0..n {
            if rh.is_palindrome(l, n) {
                let ans = format!(
                    "{}{}{}",
                    &s[0..l],
                    &s[l..n],
                    &s[0..l].to_string().chars().rev().collect::<String>()
                );
                println!("{}", ans);
                return;
            }
        }
    }
}

mod solve1 {
    use super::*;

    // Z-algorithm (解説放送でオススメされてた)
    pub(super) fn solve1(s: &String) {
        let n = s.len();
        let t = {
            let mut ret = s.chars().rev().collect::<Vec<char>>();
            for ch in s.chars() {
                ret.push(ch);
            }
            ret.iter().collect::<String>()
        };
        let z = z_algorithm(t.as_str());

        for i in 0..n {
            if z[n + i] == n - i {
                print!("{}", s);
                println!("{}", &t[n - i..n]);
                return;
            }
        }
    }
}

#[warn(dead_code)]
mod solve2 {
    use super::*;

    // https://snuke.hatenablog.com/entry/2014/12/02/235837
    fn manacher(s: &Vec<char>) -> Vec<usize> {
        let mut r = vec![0; s.len()];
        let mut i = 0;
        let mut j = 0;

        while i < s.len() {
            while i >= j && i + j < s.len() && s[i - j] == s[i + j] {
                j += 1;
            }
            r[i] = j;
            let mut k = 1;
            while i >= k && k + r[i - k] < j {
                r[i + k] = r[i - k];
                k += 1;
            }
            i += k;
            j -= k;
        }
        r
    }

    // Manacher (公式解説)
    pub(super) fn solve2(s: &String) {
        let mut t = vec!['$'];
        for ch in s.chars() {
            t.push(ch);
            t.push('$');
        }
        let r = manacher(&t);

        let mut k = 0;
        {
            let mut i = s.len();
            while r[i] < s.len() - k {
                i += 1;
                k += 1;
            }
        }

        print!("{}", s);
        let s = s.chars().collect_vec();
        for i in 0..k {
            print!("{}", s[k - 1 - i]);
        }
        println!("");
    }
}
