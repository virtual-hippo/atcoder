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
