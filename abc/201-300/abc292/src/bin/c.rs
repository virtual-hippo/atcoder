use proconio::input;

fn get_primes(n: usize) -> Vec<usize> {
    let mut i = 1;
    let mut ret = vec![];
    while i * i <= n {
        if i * i == n {
            ret.push(i);
        } else if n % i == 0 {
            ret.push(i);
            ret.push(n/i);
        }
        i += 1;
    }
    ret
}

fn main() {
    input! {
        n: usize,
    }
    let mut vec = vec![0; n];

    for i in 1..n {
        let primes = get_primes(i);
        vec[i] = primes.len();
    }
    
    let mut ans = 0_u64;
    for i in 1..n {
        ans += vec[i] as u64 * vec[n-i] as u64;
    }
    println!("{}", ans);
}

