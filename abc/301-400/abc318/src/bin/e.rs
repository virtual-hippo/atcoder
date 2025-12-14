use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [Usize1; n],
    }

    let mut cnt = vec![0; n];
    let mut sum = vec![0; n];

    let mut ans = 0;
    for k in 0..n {
        if k > 0 {
            ans += (k - 1) * cnt[a[k]] - sum[a[k]];
        }
        cnt[a[k]] += 1;
        sum[a[k]] += k;
    }
    ans -= cnt
        .iter()
        .copied()
        .filter(|&v| v > 2)
        .map(|v| (v * (v - 1) * (v - 2)) / 6)
        .sum::<usize>();

    println!("{}", ans);
}

// https://drken1215.hatenablog.com/entry/2023/09/03/192823
pub fn solve0() {
    input! {
        n: usize,
        a: [Usize1; n],
    }

    let b = {
        let mut b = vec![vec![]; n];

        for i in 0..n {
            b[a[i]].push(i);
        }

        b
    };

    let mut ans = 0_u64;

    for i in 0..n {
        let mut d = vec![];

        if b[i].len() == 0 {
            continue;
        }

        for j in 0..b[i].len() - 1 {
            d.push(b[i][j + 1] - b[i][j] - 1);
        }

        let len = d.len();

        for j in 0..d.len() {
            ans += (d[j] * (len - j) * (j + 1)) as u64;
        }
    }

    println!("{}", ans);
}

pub fn solve1() {
    input! {
        n: usize,
        a: [Usize1; n],
    }

    let b = {
        let mut b = vec![vec![]; n];

        for i in 0..n {
            b[a[i]].push(i);
        }

        b
    };

    let m: u64 = b
        .iter()
        .map(|v| v.len())
        .filter(|&v| v > 2)
        .map(|v| (v * (v - 1) * (v - 2)) as u64 / (3 * 2))
        .sum();

    let mut ans = 0_u64;

    for i in 0..n {
        if b[i].len() < 1 {
            continue;
        }

        let c = &b[i];
        let len = c.len();

        let mut s = vec![0; len + 1];
        for j in 0..len {
            s[j + 1] = s[j] + c[j] as u64;
        }

        for kk in 1..len {
            let k = c[kk];
            ans += (kk * (k - 1)) as u64;
            ans -= s[kk] - s[0];
        }
    }

    ans -= m;

    println!("{}", ans);
}
