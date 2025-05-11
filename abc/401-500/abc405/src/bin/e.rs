//
// モジュラ逆元
// combination
// フェルマーの小定理
//

#![allow(unused_imports)]
use ac_library::*;
use itertools::*;
use proconio::{fastout, input, marker::Chars};
use rustc_hash::{FxHashMap, FxHashSet};
use std::collections::{HashMap, HashSet, VecDeque};
use superslice::Ext;

#[fastout]
fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
        d: usize,
    }
    let comb = comb::Comb::<ModInt998244353>::new(a + b + c + d);

    // A の右端よりも左にある B の個数を全探索する
    let ans = (0..b + 1)
        .map(|i| comb.comb(a - 1 + i, i) * comb.comb(b - i + d + c, c))
        .sum::<ModInt998244353>();

    println!("{}", ans);
}

// 以下からパクった
// https://github.com/atcoder/live_library/blob/master/comb.cpp
pub mod comb {
    use ac_library::modint::ModIntBase;
    use std::ops::Index;

    // モジュラ逆元
    struct ModInv<Mint: ModIntBase> {
        d: Vec<Mint>,
    }

    impl<Mint: ModIntBase> ModInv<Mint> {
        fn new(n: usize) -> Self {
            let mut d = vec![Mint::new(0), Mint::new(1)];
            for i in 2..n {
                let v = (-d[(Mint::modulus() as usize) % i]) * Mint::new((Mint::modulus() as usize) / i);
                d.push(v);
            }
            ModInv { d }
        }
    }

    impl<Mint: ModIntBase> Index<usize> for ModInv<Mint> {
        type Output = Mint;

        fn index(&self, index: usize) -> &Self::Output {
            &self.d[index]
        }
    }

    // 階乗
    struct ModFact<Mint: ModIntBase> {
        d: Vec<Mint>,
    }

    impl<Mint: ModIntBase> ModFact<Mint> {
        fn new(n: usize) -> Self {
            let mut d = vec![Mint::new(1), Mint::new(1)];
            for i in 2..n {
                let v = d[i - 1] * Mint::new(i);
                d.push(v);
            }
            ModFact { d }
        }
    }

    impl<Mint: ModIntBase> Index<usize> for ModFact<Mint> {
        type Output = Mint;

        fn index(&self, index: usize) -> &Self::Output {
            &self.d[index]
        }
    }

    // 階乗の逆元
    struct ModFactInv<Mint: ModIntBase> {
        d: Vec<Mint>,
    }

    impl<Mint: ModIntBase> ModFactInv<Mint> {
        fn new(n: usize) -> Self {
            let invs = ModInv::new(n);
            let mut d = vec![Mint::new(1), Mint::new(1)];
            for i in 2..n {
                let v = d[i - 1] * invs[i];
                d.push(v);
            }
            ModFactInv { d }
        }
    }

    impl<Mint: ModIntBase> Index<usize> for ModFactInv<Mint> {
        type Output = Mint;

        fn index(&self, index: usize) -> &Self::Output {
            &self.d[index]
        }
    }

    pub struct Comb<Mint: ModIntBase> {
        n: usize,
        mod_fact: ModFact<Mint>,
        mod_fact_inv: ModFactInv<Mint>,
    }
    impl<Mint: ModIntBase> Comb<Mint> {
        pub fn new(n: usize) -> Self {
            let mod_fact = ModFact::new(n + 1);
            let mod_fact_inv = ModFactInv::new(n + 1);
            Comb { n, mod_fact, mod_fact_inv }
        }

        pub fn comb(&self, n: usize, k: usize) -> Mint {
            if n < k {
                return Mint::new(0);
            }

            if n > self.n {
                panic!("n is too large");
            }

            self.mod_fact[n] * self.mod_fact_inv[k] * self.mod_fact_inv[n - k]
        }
    }
}
