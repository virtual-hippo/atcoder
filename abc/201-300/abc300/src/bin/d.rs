use proconio::input;

fn sieve_of_eratosthenes(n: usize) -> Vec<bool> {
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
    let koho: Vec<usize> = sieve_of_eratosthenes(1_000_000)
        .iter()
        .enumerate()
        .filter(|&(_, &v)| v)
        .map(|(i, _)| i)
        .collect();
    let mut cnt = 0;
    for i in 0..koho.len()-2 {
        if koho[i] * koho[i] * koho[i] * koho[i] * koho[i] > n {
            break;
        }
        for j in i+1..koho.len()-1 {
            if koho[i] * koho[i] * koho[j] * koho[j] * koho[j] > n {
                break;
            }
            for k in j+1..koho.len() {
                if koho[i] * koho[i] * koho[j] * koho[k] * koho[k] > n {
                    break;
                }
                if koho[i] * koho[i] * koho[j] * koho[k] * koho[k] <= n {
                    cnt += 1;
                }
            }
        }
    }
    println!("{}", cnt);
}
