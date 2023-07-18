use proconio::input;

fn main() {
    input! {
        k: usize,
    }
    let primes = prime_factorize(k);
    let mut ans = 0;
    for &(val, exp) in primes.iter() {
        let mut cnt = 0;
        let mut i = val;
        while cnt < exp {
            let mut curr = i;
            while curr % val == 0 {
                curr /= val;
                cnt += 1;
            }
            i += 1;
        }
        ans = ans.max(i-1);
    }
    println!("{}", ans);
}

pub fn prime_factorize(input: usize) -> Vec<(usize, usize)> {
    let mut res = vec![];
    let mut target = input;
    let mut i = 2;
    while i * i <= target {
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

