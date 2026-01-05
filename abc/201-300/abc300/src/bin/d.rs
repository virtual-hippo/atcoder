use proconio::input;

///! エラトステネスの篩
///! https://algo-method.com/descriptions/64
pub fn sieve_of_eratosthenes(n: usize) -> Vec<bool> {
    // 素数がtrueとなったベクタ
    let mut is_prime = vec![true; n + 1];
    is_prime[0] = false;
    is_prime[1] = false;

    for i in 2..n + 1 {
        if is_prime[i] == false {
            continue;
        }
        let mut j = i * 2;
        while j < n + 1 {
            is_prime[j] = false;
            j += i;
        }
    }
    is_prime
}

fn main() {
    input! {
        n: usize,
    }

    let primes = sieve_of_eratosthenes(1_000_100);

    let ans = (2_usize..)
        .take_while(|&a| a.pow(5) <= n)
        .filter(|&a| primes[a])
        .map(|a| {
            ((a + 1)..)
                .take_while(|&b| a.pow(2) * b * (b + 1).pow(2) <= n)
                .filter(|&b| primes[b])
                .map(|b| {
                    ((b + 1)..)
                        .take_while(|&c| a.pow(2) * b * c.pow(2) <= n)
                        .filter(|&c| primes[c])
                        .count()
                })
                .sum::<usize>()
        })
        .sum::<usize>();
    println!("{}", ans);
}
