use proconio::{fastout, input};
use rustc_hash::FxHashMap;

// "k 要素のvec" を "k 要素の p+1 進数" に変換
fn vec_to_index(v: &[usize], p: usize) -> usize {
    let mut idx = 0;
    let mut base = 1;
    for &x in v {
        idx += x * base;
        base *= p + 1;
    }
    idx
}

//  "k 要素の p+1 進数"  "k 要素のvec" に変換
fn index_to_vec(mut idx: usize, k: usize, p: usize) -> Vec<usize> {
    let mut v = vec![0; k];
    for i in 0..k {
        v[i] = idx % (p + 1);
        idx /= p + 1;
    }
    v
}

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        p: usize,
    }

    let mut c = vec![0; n];
    let mut a = vec![];

    for i in 0..n {
        input! {
            cc: usize,
            aa: [usize; k],
        }
        c[i] = cc;
        a.push(aa);
    }

    let dp_size = (p + 1).pow(k as u32);
    let mut dp = vec![usize::MAX; dp_size];

    dp[vec_to_index(&vec![0; k], p)] = 0;

    for i in 1..n + 1 {
        let mut now_dp = dp.clone();
        for idx in 0..dp_size {
            if dp[idx] == usize::MAX {
                continue;
            }

            let cost = dp[idx] + c[i - 1];
            let key = index_to_vec(idx, k, p);
            let mut new_key = key.clone();

            for j in 0..k {
                new_key[j] = (new_key[j] + a[i - 1][j]).min(p);
            }

            let new_idx = vec_to_index(&new_key, p);
            if cost < now_dp[new_idx] {
                now_dp[new_idx] = cost;
            }
        }

        dp = now_dp;
    }

    let ans = (0..dp_size)
        .filter_map(|idx| {
            if dp[idx] == usize::MAX {
                return None;
            }

            let key = index_to_vec(idx, k, p);
            if key.iter().all(|&v| v >= p) {
                Some(dp[idx])
            } else {
                None
            }
        })
        .min()
        .map(|v| format!("{}", v))
        .unwrap_or("-1".to_string());

    println!("{}", ans);
}

pub solve() {
        input! {
        n: usize,
        k: usize,
        p: usize,
    }

    let mut c = vec![0; n];
    let mut a = vec![];

    for i in 0..n {
        input! {
            cc: usize,
            aa: [usize; k],
        }
        c[i] = cc;
        a.push(aa);
    }

    let mut dp = FxHashMap::default();

    dp.insert(vec![0; k], 0);

    for i in 1..n + 1 {
        let mut now_dp = dp.clone();
        for (key, p_c) in dp.iter() {
            let cost = p_c + c[i - 1];
            let mut key = key.clone();

            for j in 0..k {
                key[j] = (key[j] + a[i - 1][j]).min(p);
            }

            if let Some(&old_c) = now_dp.get(&key) {
                if cost < old_c {
                    now_dp.insert(key, cost);
                }
            } else {
                now_dp.insert(key, cost);
            }
        }

        dp = now_dp;
    }

    let ans = dp
        .iter()
        .filter(|&(k, _)| k.iter().all(|&v| v >= p))
        .map(|(_, v)| *v)
        .min()
        .map(|v| format!("{}", v))
        .unwrap_or("-1".to_string());

    println!("{}", ans);
}