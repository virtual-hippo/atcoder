use std::collections::HashMap;

use proconio::{fastout, input};

const INF: i64 = i64::MAX - 10;

fn rec(a: &Vec<Vec<i64>>, state: &[[i64; 3]; 3], memo: &mut HashMap<[[i64; 3]; 3], i64>) -> i64 {
    if let Some(&ans) = memo.get(state) {
        return ans;
    }

    // 行
    for i in 0..3 {
        if state[i].iter().all(|&x| x == 1) {
            return INF;
        }
        if state[i].iter().all(|&x| x == 2) {
            return -INF;
        }
    }
    // 列
    for j in 0..3 {
        if (0..3).all(|i| state[i][j] == 1) {
            return INF;
        }
        if (0..3).all(|i| state[i][j] == 2) {
            return -INF;
        }
    }
    // 斜め
    {
        if (0..3).all(|i| state[i][i] == 1) {
            return INF;
        }
        if (0..3).all(|i| state[i][i] == 2) {
            return -INF;
        }
        if (0..3).all(|i| state[2 - i][i] == 1) {
            return INF;
        }
        if (0..3).all(|i| state[2 - i][i] == 2) {
            return -INF;
        }
    }

    let mut ret = -INF;
    let mut cnt = 0;
    for i in 0..3 {
        for j in 0..3 {
            if state[i][j] != 0 {
                cnt += 1;
                continue;
            }
            let mut state = state.clone();
            state[i][j] = 1;
            for x in 0..3 {
                for y in 0..3 {
                    state[x][y] = match state[x][y] {
                        0 => 0,
                        1 => 2,
                        2 => 1,
                        _ => unreachable!(),
                    }
                }
            }
            let ans = if let Some(&ans) = memo.get(&state) {
                ans
            } else {
                let ans = -rec(a, &state, memo);
                memo.insert(state, ans);
                ans
            };
            ret = ret.max(ans);
        }
    }

    let ret = if cnt < 9 {
        ret
    } else {
        let ret = if let Some(&ans) = memo.get(state) {
            ans
        } else {
            let mut ans = 0;
            for i in 0..3 {
                for j in 0..3 {
                    match state[i][j] {
                        1 => ans += a[i][j],
                        2 => ans -= a[i][j],
                        _ => unreachable!(),
                    };
                }
            }
            memo.insert(*state, ans);
            ans
        };
        ret
    };
    ret
}

#[fastout]
fn main() {
    input! {
        a: [[i64;3];3],
    }
    let mut memo = HashMap::new();
    let mut state = [[0; 3]; 3];
    let ret = rec(&a, &mut state, &mut memo);
    if ret < 0 {
        println!("Aoki");
    } else {
        println!("Takahashi");
    }
}
