use itertools::*;
use proconio::input;
use std::collections::BTreeMap;

pub fn solve() {
    input! {
        n: usize,
        m: usize,
        a: [usize; n],
        b: [usize; n],
    }

    let a = a.iter().copied().map(|v| v % m).collect_vec();
    let b = b.iter().copied().map(|v| v % m).collect_vec();

    let mut map = BTreeMap::new();
    for i in 0..n {
        *map.entry(a[i]).or_insert(0) += 1;
    }

    let mut ans = 0;
    for i in 0..n {
        let c = m - b[i];

        let (ai, cnt) = if let Some((&ai, &cnt)) = map.range(c..).next() {
            (ai, cnt)
        } else {
            let (&ai, &cnt) = map.iter().next().unwrap();
            (ai, cnt)
        };
        ans += (ai + b[i]) % m;
        if cnt == 1 {
            map.remove(&ai);
        } else {
            map.insert(ai, cnt - 1);
        }
    }

    println!("{}", ans);
}

fn solve1() {
    input! {
        n: usize,
        m: u64,
        a: [u64; n],
        b: [u64; n],
    }

    let aa = a.iter().copied().sorted_by_key(|&v| std::cmp::Reverse(v)).collect_vec();
    let bb = b.iter().copied().sorted().collect_vec();

    let mut j = 0;
    let mut c = 0;

    for i in 0..n {
        while j < n && aa[i] + bb[j] < m {
            j += 1;
        }

        if j >= n {
            break;
        }

        c += 1;
        j += 1;
    }

    let ans = a.iter().sum::<u64>() + b.iter().sum::<u64>() - c * m;
    println!("{}", ans);
}

fn main() {
    input! {
        t: usize,
    }

    (0..t).for_each(|_| solve1());
}
