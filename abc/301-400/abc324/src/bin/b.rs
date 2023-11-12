use proconio::input;

pub fn prime_factorize(input: usize) -> Vec<(usize, usize)> {
    let mut res = vec![];
    let mut target = input;
    let mut i = 2;
    while i * i <= target && i < 5 {
        if target % i != 0 {
            i += 1;
            continue;
        }
        let mut e = 0;
        while target % i == 0 {
            e += 1;
            target /= i;
        }
        res.push((i, e));

        i += 1;
    }
    if target != 1 {
        res.push((target, 1));
    }
    res
}

fn main() {
    input! {
        n: usize,
    }
    let primes = prime_factorize(n);
    for &(prime, _) in primes.iter() {
        if prime != 2 && prime != 3 {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
