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
    let mut ans = 0;

    let rh = rolling_hash::RollingHash::new(s.as_bytes());
    for i in 0..s.len() {
        for j in i + 1..s.len() + 1 {
            if rh.is_palindrome(i, j) {
                ans = ans.max(j - i);
            }
        }
    }

    println!("{}", ans);
}

pub mod rolling_hash {
    use rand::Rng;
    use std::cmp::PartialEq;
    use std::ops::{Add, Mul, Sub};

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

        /// S[l, r) が回文か判定する
        pub fn is_palindrome(&self, l: usize, r: usize) -> bool {
            self.get_hash(l, r) == self.get_rev_hash(l, r)
        }
    }
}
