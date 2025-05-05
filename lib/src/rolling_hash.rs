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

    /// RollingHash 作成用ファクトリ
    /// 同じファクトリ, 同じ文字列から作成された RollingHash は同じハッシュ値を持つ
    /// 異なるファクトリから作成された RollingHash は同じ文字列でも異なるハッシュ値を持つ
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

#[cfg(test)]
mod tests {
    use super::rolling_hash::*;

    #[test]
    fn it_works0() {
        let rolling_hash_builder = RollingHashBuilder::new();
        let text = "there".to_string();
        let rh = rolling_hash_builder.build(text.as_bytes());

        // "there" is palindrome
        assert_eq!(rh.is_palindrome(0, text.len()), false);

        // "t" is palindrome
        assert_eq!(rh.is_palindrome(0, 1), true);

        // "ere" is palindrome
        assert_eq!(rh.is_palindrome(2, text.len()), true);
    }

    #[test]
    fn it_works1() {
        let rolling_hash_builder = RollingHashBuilder::new();

        let text0 = "there".to_string();
        let rh0 = rolling_hash_builder.build(text0.as_bytes());

        let text1 = "th".to_string();
        let rh1 = rolling_hash_builder.build(text1.as_bytes());

        let hash0 = rh0.get_hash(0, text1.len());
        let hash1 = rh1.get_hash(0, text1.len());

        assert_eq!(hash0 == hash1, true);
    }
}
