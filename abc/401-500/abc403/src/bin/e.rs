#![allow(unused_imports)]
use ac_library::*;
use itertools::*;
use proconio::{fastout, input, marker::Chars};
use rustc_hash::{FxHashMap, FxHashSet};
use std::collections::{HashMap, HashSet, VecDeque};
use superslice::Ext;

#[fastout]
fn main() {
    _solve1();
}

// 参考: https://atcoder.jp/contests/abc403/submissions/65294475
fn _solve0() {
    input! {
        q: usize,
    }

    let mut x_set = HashSet::new();
    let mut y_map: HashMap<rolling_hash::Mint, Vec<usize>> = HashMap::new();
    let mut y_alive = HashSet::new();

    let builder = rolling_hash::RollingHashBuilder::new();

    for idx in 0..q {
        input! {
            t: usize,
            s: String,
        }

        let n = s.len();
        let rh = builder.build(s.as_bytes());

        match t {
            1 => {
                let z = rh.get_hash(0, n);

                if let Some(f) = y_map.get(&z) {
                    for y in f.iter() {
                        if y_alive.contains(y) {
                            y_alive.remove(y);
                        }
                    }
                    y_map.remove(&z);
                }
                x_set.insert(z);
            },
            2 => {
                let is_not_contained = (1..n + 1).all(|r| !x_set.contains(&rh.get_hash(0, r)));

                if is_not_contained {
                    y_alive.insert(idx);
                    for r in 1..=n {
                        let z = rh.get_hash(0, r);
                        y_map.entry(z).or_insert_with(|| vec![]).push(idx);
                    }
                }
            },
            _ => unreachable!(),
        }
        println!("{}", y_alive.len());
    }
}

fn _solve1() {
    input! {
        q: usize,
    }

    let mut trie = trie_tree::TrieTree::new();
    let mut y = 0;

    for qi in 0..q {
        input! {
            t: usize,
            s: String,
        }

        match t {
            1 => {
                trie.add_x(&s, qi);
            },
            2 => {
                trie.add_y(&s, qi);
                y += 1;
            },
            _ => unreachable!(),
        }
        println!("{}", y - trie.z.len());
    }
}

pub mod rolling_hash {
    use rand::Rng;
    use std::cmp::PartialEq;
    use std::ops::{Add, Mul, Sub};

    ///
    /// Mint
    ///
    #[derive(Copy, Clone, Debug, Eq, Hash)]
    pub struct Mint {
        v0: u64,
        v1: u64,
    }
    impl Mint {
        const M0: u64 = 1_000_000_021;
        const M1: u64 = 1_000_000_033;

        pub fn new(v0: u64, v1: u64) -> Mint {
            Mint { v0: v0 % Self::M0, v1: v1 % Self::M1 }
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

    pub struct RollingHashBuilder(Mint);
    impl RollingHashBuilder {
        pub fn new() -> Self {
            let mut rng = rand::prelude::ThreadRng::default();
            let randam_value0 = rng.gen_range(2..8197);
            let randam_value1 = rng.gen_range(2..8197);
            let base = Mint::new(randam_value0, randam_value1);
            Self(base)
        }

        pub fn build(&self, s: &[u8]) -> RollingHash {
            RollingHash::new(s, self.0)
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
        fn new(s: &[u8], base: Mint) -> Self {
            let n = s.len();

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
            Self { hash, hash_rev, power }
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

/// トライ木
/// 参考: https://atcoder.jp/contests/abc403/submissions/65219576
pub mod trie_tree {
    use std::collections::{HashMap, HashSet};

    #[derive(Debug)]
    pub struct TrieNode {
        next: Vec<usize>,
        /// X に含まれるかのフラグ
        flag: bool,
        /// X の要素を接頭辞として持つものの集合
        z: HashSet<usize>,
    }

    impl TrieNode {
        pub fn new() -> Self {
            Self {
                next: vec![usize::MAX; 26],
                flag: false,
                z: HashSet::new(),
            }
        }
    }

    #[derive(Debug)]
    pub struct TrieTree {
        nodes: Vec<TrieNode>,
        /// X の要素を接頭辞として持つものの集合
        pub z: HashSet<usize>,
    }

    impl TrieTree {
        pub fn new() -> Self {
            Self {
                nodes: vec![TrieNode::new()],
                z: HashSet::new(),
            }
        }

        pub fn add_x(&mut self, s: &str, _i: usize) {
            let mut k = 0;
            for ch in s.chars() {
                let c = (ch as u8 - 'a' as u8) as usize;

                if self.nodes[k].next[c] == usize::MAX {
                    self.nodes[k].next[c] = self.nodes.len();
                    self.nodes.push(TrieNode::new());
                }

                k = self.nodes[k].next[c];
            }

            self.nodes[k].flag = true;
            for v in self.nodes[k].z.iter() {
                self.z.insert(*v);
            }
            self.nodes[k].z.clear();
        }

        pub fn add_y(&mut self, s: &str, i: usize) {
            let mut k = 0;
            for ch in s.chars() {
                let c = (ch as u8 - 'a' as u8) as usize;

                if self.nodes[k].next[c] == usize::MAX {
                    self.nodes[k].next[c] = self.nodes.len();
                    self.nodes.push(TrieNode::new());
                }

                self.nodes[k].z.insert(i);
                if self.nodes[k].flag {
                    self.z.insert(i);
                }

                k = self.nodes[k].next[c];
            }

            self.nodes[k].z.insert(i);
            if self.nodes[k].flag {
                self.z.insert(i);
            }
        }
    }
}
