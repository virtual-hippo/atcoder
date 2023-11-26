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
        a: usize,
        b: usize,
        c: usize,
        d: usize,
    }
    let sosu = sieve_of_eratosthenes(201);
    for tka in a..b + 1 {
        let mut win_tka = true;
        for ao in c..d + 1 {
            if sosu[tka + ao] {
                win_tka = false;
            }
        }
        if win_tka {
            println!("Takahashi");
            return;
        }
    }
    println!("Aoki");
}
