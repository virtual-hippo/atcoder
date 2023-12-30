use proconio::{fastout, input};

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

#[fastout]
fn main() {
    input! {
        n: usize,
    }
    let mut ans = 0;
    let sosu = sieve_of_eratosthenes(1000000)
        .into_iter()
        .enumerate()
        .filter(|(_, v)| *v)
        .map(|(i, _)| i)
        .collect::<Vec<usize>>();
    for a_i in 0..sosu.len() - 2 {
        let a2 = sosu[a_i] * sosu[a_i];
        if a2 > n {
            break;
        }
        for b_i in a_i + 1..sosu.len() - 1 {
            let b = sosu[b_i];
            if a2 * b > n {
                break;
            }
            for c_i in b_i + 1..sosu.len() {
                let c2 = sosu[c_i] * sosu[c_i];
                if a2 * b <= n / c2 {
                    ans += 1;
                } else {
                    break;
                }
            }
        }
    }
    println!("{}", ans);
}
