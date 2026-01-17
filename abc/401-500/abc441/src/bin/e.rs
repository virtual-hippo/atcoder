use ac_library::*;
use proconio::{fastout, input};

// 転倒数
#[fastout]
fn main() {
    solve3();
}

pub fn solve1() {
    input! {
        n: usize,
        s: String,
    }

    let d = s.chars().fold(vec![n], |mut acc, ch| {
        let v = match ch {
            'A' => 1,
            'B' => -1,
            'C' => 0,
            _ => unreachable!(),
        };
        acc.push((acc[acc.len() - 1] as i64 + v) as usize);
        acc
    });

    let ans = d
        .iter()
        .fold((0_u64, FenwickTree::new(2 * n + 1, 0_u64)), |(ans, mut bit), &d| {
            let x = bit.sum(..d);
            bit.add(d, 1);
            (ans + x, bit)
        })
        .0;
    println!("{}", ans);
}

// O(N)
pub fn solve2() {
    input! {
        n: usize,
        s: String,
    }

    let d: Vec<usize> = s.chars().fold(vec![n], |mut acc, ch| {
        let v = match ch {
            'A' => 1,
            'B' => -1,
            'C' => 0,
            _ => unreachable!(),
        };
        acc.push((acc[acc.len() - 1] as i64 + v) as usize);
        acc
    });

    let initial_state = (vec![0_u64; 2 * n + 2], 0_u64, 0_u64);

    let (_, acc_ans, last_s_sum) = (0..n).fold(initial_state, |(mut count, ans, s_sum), i| {
        let new_ans = ans + s_sum;
        count[d[i]] += 1;

        let new_s_sum = match d[i + 1] as i64 - d[i] as i64 {
            1 => s_sum + count[d[i]],      // d[i] が「より小さい」範囲に入る
            -1 => s_sum - count[d[i + 1]], // d[i+1] が「より小さい」範囲から外れる
            _ => s_sum,                    // 変化なし
        };

        (count, new_ans, new_s_sum)
    });

    // i = n の分を加算
    let ans = acc_ans + last_s_sum;

    println!("{}", ans);
}

// O(N)
pub fn solve3() {
    input! {
        n: usize,
        s: String,
    }

    let init = (vec![0_u64; 2 * n + 1], n, 0_u64, 0_u64);
    let ans = s
        .chars()
        .fold(init, |(mut cnt, x, sum, ans), ch| {
            cnt[x] += 1;
            let (new_sum, new_x) = match ch {
                // 現在までの分を加える
                'A' => (sum + cnt[x], x + 1),
                // これまでの分を減算する
                'B' => (sum - cnt[x - 1], x - 1),
                'C' => (sum, x),
                _ => unreachable!(),
            };
            (cnt, new_x, new_sum, ans + new_sum)
        })
        .3;

    println!("{}", ans);
}
