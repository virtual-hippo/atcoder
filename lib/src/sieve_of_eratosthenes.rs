use itertools::*;

///! エラトステネスの篩
///! https://algo-method.com/descriptions/64
pub fn sieve_of_eratosthenes(n: usize) -> Vec<bool> {
    // 素数がtrueとなったベクタ
    let mut init = vec![true; n + 1];
    init[0] = false;
    init[1] = false;

    (2..n + 1).fold(init, |mut acc, x| {
        if acc[x] {
            // x の倍数は素数ではないと判定する
            ((x * 2)..).step_by(x).take_while(|&y| y < n + 1).for_each(|y| {
                acc[y] = false;
            });
        }
        acc
    })
}

pub fn get_primes() -> Vec<usize> {
    let primes = sieve_of_eratosthenes(2_000_000)
        .iter()
        .enumerate()
        .filter(|&(_, &v)| v)
        .map(|(i, _)| i)
        .collect_vec();

    primes
}

// abc445-e (1 - N までの素因数を高速に求めることができる)
// https://www.youtube.com/watch?v=QZYvzw9Wi5E&t=2238s
pub struct Sieve {
    /// 最小の素因数
    f: Vec<usize>,
    /// 素数一覧
    pub primes: Vec<usize>,
}

impl Sieve {
    pub fn new(n: usize) -> Self {
        let mut f = vec![0; n + 1];
        let mut primes = vec![];

        f[0] = 1;
        f[1] = 1;

        for x in 2..=n {
            if f[x] != 0 {
                continue;
            }
            primes.push(x);
            f[x] = x;

            // x の倍数にマーキングする
            ((x * x)..).step_by(x).take_while(|&y| y < n + 1).for_each(|y| {
                if f[y] == 0 {
                    f[y] = x;
                }
            });
        }

        Self { f, primes }
    }

    pub fn is_prime(&self, x: usize) -> bool {
        1 < x && self.f[x] == x
    }

    pub fn factor_list(&self, mut x: usize) -> Vec<usize> {
        let mut factors = vec![];
        while x > 1 {
            factors.push(self.f[x]);
            x /= self.f[x];
        }
        factors
    }

    pub fn factor(&self, x: usize) -> Vec<(usize, usize)> {
        self.factor_list(x)
            .iter()
            .copied()
            .dedup_with_count()
            .map(|(count, prime)| (prime, count)) // (素因数, 指数) の順に揃える
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ─── 構築 ───────────────────────────────────────────────

    #[test]
    fn test_new_small() {
        let sieve = Sieve::new(10);
        assert_eq!(sieve.primes, vec![2, 3, 5, 7]);
    }

    #[test]
    fn test_new_boundary_zero_and_one() {
        let sieve = Sieve::new(1);
        assert!(sieve.primes.is_empty());
    }

    #[test]
    fn test_new_boundary_two() {
        let sieve = Sieve::new(2);
        assert_eq!(sieve.primes, vec![2]);
    }

    #[test]
    fn test_primes_up_to_30() {
        let sieve = Sieve::new(30);
        assert_eq!(sieve.primes, vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29]);
    }

    #[test]
    fn test_prime_count_up_to_100() {
        let sieve = Sieve::new(100);
        // 100以下の素数は25個
        assert_eq!(sieve.primes.len(), 25);
    }

    #[test]
    fn test_prime_count_up_to_1000() {
        let sieve = Sieve::new(1000);
        // 1000以下の素数は168個
        assert_eq!(sieve.primes.len(), 168);
    }

    // ─── 最小素因数テーブル f の検証 ─────────────────────────

    #[test]
    fn test_smallest_prime_factor_table() {
        let sieve = Sieve::new(20);
        // f[0]=1, f[1]=1 (番兵)
        assert_eq!(sieve.f[0], 1);
        assert_eq!(sieve.f[1], 1);
        // 素数は自分自身
        assert_eq!(sieve.f[2], 2);
        assert_eq!(sieve.f[3], 3);
        assert_eq!(sieve.f[5], 5);
        assert_eq!(sieve.f[7], 7);
        assert_eq!(sieve.f[11], 11);
        assert_eq!(sieve.f[13], 13);
        assert_eq!(sieve.f[17], 17);
        assert_eq!(sieve.f[19], 19);
        // 合成数は最小素因数
        assert_eq!(sieve.f[4], 2); // 4 = 2^2
        assert_eq!(sieve.f[6], 2); // 6 = 2*3
        assert_eq!(sieve.f[8], 2); // 8 = 2^3
        assert_eq!(sieve.f[9], 3); // 9 = 3^2
        assert_eq!(sieve.f[10], 2); // 10 = 2*5
        assert_eq!(sieve.f[12], 2); // 12 = 2^2*3  ← 元コードだと 3 になるバグ
        assert_eq!(sieve.f[14], 2); // 14 = 2*7
        assert_eq!(sieve.f[15], 3); // 15 = 3*5
        assert_eq!(sieve.f[16], 2); // 16 = 2^4
        assert_eq!(sieve.f[18], 2); // 18 = 2*3^2
        assert_eq!(sieve.f[20], 2); // 20 = 2^2*5
    }

    // ─── is_prime ───────────────────────────────────────────

    #[test]
    fn test_is_prime_basic() {
        let sieve = Sieve::new(100);
        assert!(sieve.is_prime(2));
        assert!(sieve.is_prime(3));
        assert!(sieve.is_prime(5));
        assert!(sieve.is_prime(97));

        assert!(!sieve.is_prime(0));
        assert!(!sieve.is_prime(1));
        assert!(!sieve.is_prime(4));
        assert!(!sieve.is_prime(6));
        assert!(!sieve.is_prime(100));
    }

    #[test]
    fn test_is_prime_agrees_with_primes_list() {
        let n = 500;
        let sieve = Sieve::new(n);
        let prime_set: std::collections::HashSet<usize> = sieve.primes.iter().copied().collect();
        for x in 2..=n {
            assert_eq!(
                sieve.is_prime(x),
                prime_set.contains(&x),
                "is_prime({x}) と primes リストが不一致"
            );
        }
    }

    // ─── factor_list ────────────────────────────────────────

    #[test]
    fn test_factor_list_prime() {
        let sieve = Sieve::new(100);
        assert_eq!(sieve.factor_list(2), vec![2]);
        assert_eq!(sieve.factor_list(3), vec![3]);
        assert_eq!(sieve.factor_list(97), vec![97]);
    }

    #[test]
    fn test_factor_list_prime_power() {
        let sieve = Sieve::new(100);
        assert_eq!(sieve.factor_list(4), vec![2, 2]);
        assert_eq!(sieve.factor_list(8), vec![2, 2, 2]);
        assert_eq!(sieve.factor_list(27), vec![3, 3, 3]);
        assert_eq!(sieve.factor_list(64), vec![2, 2, 2, 2, 2, 2]);
    }

    #[test]
    fn test_factor_list_composite() {
        let sieve = Sieve::new(100);
        assert_eq!(sieve.factor_list(6), vec![2, 3]);
        assert_eq!(sieve.factor_list(12), vec![2, 2, 3]);
        assert_eq!(sieve.factor_list(30), vec![2, 3, 5]);
        assert_eq!(sieve.factor_list(60), vec![2, 2, 3, 5]);
        assert_eq!(sieve.factor_list(72), vec![2, 2, 2, 3, 3]);
    }

    #[test]
    fn test_factor_list_one() {
        let sieve = Sieve::new(10);
        assert_eq!(sieve.factor_list(1), vec![]);
    }

    #[test]
    fn test_factor_list_product_equals_original() {
        let n = 500;
        let sieve = Sieve::new(n);
        for x in 2..=n {
            let product: usize = sieve.factor_list(x).iter().product();
            assert_eq!(product, x, "factor_list({x}) の積が元の値と不一致");
        }
    }

    // ─── factor ─────────────────────────────────────────────

    #[test]
    fn test_factor_prime() {
        let sieve = Sieve::new(100);
        assert_eq!(sieve.factor(2), vec![(2, 1)]);
        assert_eq!(sieve.factor(97), vec![(97, 1)]);
    }

    #[test]
    fn test_factor_prime_power() {
        let sieve = Sieve::new(100);
        assert_eq!(sieve.factor(4), vec![(2, 2)]);
        assert_eq!(sieve.factor(8), vec![(2, 3)]);
        assert_eq!(sieve.factor(27), vec![(3, 3)]);
    }

    #[test]
    fn test_factor_composite() {
        let sieve = Sieve::new(100);
        assert_eq!(sieve.factor(6), vec![(2, 1), (3, 1)]);
        assert_eq!(sieve.factor(12), vec![(2, 2), (3, 1)]);
        assert_eq!(sieve.factor(30), vec![(2, 1), (3, 1), (5, 1)]);
        assert_eq!(sieve.factor(60), vec![(2, 2), (3, 1), (5, 1)]);
        assert_eq!(sieve.factor(72), vec![(2, 3), (3, 2)]);
        assert_eq!(sieve.factor(100), vec![(2, 2), (5, 2)]);
    }

    #[test]
    fn test_factor_one() {
        let sieve = Sieve::new(10);
        assert_eq!(sieve.factor(1), vec![]);
    }

    #[test]
    fn test_factor_reconstruction() {
        let n = 500;
        let sieve = Sieve::new(n);
        for x in 2..=n {
            let product: usize = sieve.factor(x).iter().map(|&(p, e)| p.pow(e as u32)).product();
            assert_eq!(product, x, "factor({x}) から元の値を復元できない");
        }
    }

    #[test]
    fn test_factor_sorted_ascending() {
        let n = 500;
        let sieve = Sieve::new(n);
        for x in 2..=n {
            let factors = sieve.factor(x);
            for w in factors.windows(2) {
                assert!(w[0].0 < w[1].0, "factor({x}) の素因数が昇順でない: {:?}", factors);
            }
        }
    }

    // ─── 大きめの値 ─────────────────────────────────────────

    #[test]
    fn test_large_sieve() {
        let sieve = Sieve::new(1_000_000);
        // 10^6 以下の素数は 78498 個
        assert_eq!(sieve.primes.len(), 78498);
        // 大きい素数の判定
        assert!(sieve.is_prime(999983));
        assert!(!sieve.is_prime(999999)); // 999999 = 3 * 333333
    }

    #[test]
    fn test_factor_large_composite() {
        let sieve = Sieve::new(1_000_000);
        // 720720 = 2^4 * 3^2 * 5 * 7 * 11 * 13
        assert_eq!(sieve.factor(720720), vec![(2, 4), (3, 2), (5, 1), (7, 1), (11, 1), (13, 1)]);
    }

    // ─── 境界値 ─────────────────────────────────────────────

    #[test]
    fn test_n_equals_boundary() {
        // n ちょうどの値が正しく扱われるか
        let sieve = Sieve::new(7);
        assert_eq!(sieve.primes, vec![2, 3, 5, 7]);
        assert!(sieve.is_prime(7));
        assert_eq!(sieve.factor(6), vec![(2, 1), (3, 1)]);
    }
}
