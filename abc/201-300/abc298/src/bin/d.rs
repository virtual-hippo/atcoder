use proconio::input;
use std::collections::VecDeque;

fn main() {
    input! {
        q: usize,
    }
    const MOD: u64 = 998244353;

    let mut mods = vec![1; q+1];

    for i in 2..q+1 {
        mods[i] = 10 * mods[i-1] % MOD;
    }

    let mut s = VecDeque::with_capacity(q);
    s.push_back('1');
    let mut current = 1;


    for _ in 0..q {
        input! {
            query: usize,
        }
        if query == 1 {
            input! {
                x: char,
            }
            s.push_back(x);
            current *= 10;
            current += x as u64 - 48;
            current %= MOD;
        }
        if query == 2 {
            let front = s[0] as u64 - 48;
            let sub = front * mods[s.len()] % MOD;

            current += MOD;
            current -= sub;
            current %= MOD;
            s.pop_front();
        }
        if query == 3 {
            println!("{}", current % MOD);
        }
    }
}
