use std::collections::HashMap;

/// メモ化再帰
/// https://atcoder.jp/contests/abc275/tasks/abc275_d
mod memo_recursive {
    type Integer = u64;

    fn recursive(x: Integer, memo: &mut HashMap<Integer, Integer>) -> Integer {
        if x == 0 {
            1
        } else {
            if let Some(val) = memo.get(&x) {
                *val
            } else {    
                let a= x/2;
                let b = x/3;
                let res = memo_recursive(a, memo) + memo_recursive(b, memo);
                memo.insert(x, res);
                res
            }
        }
    }
    fn run() {
        let mut memo:HashMap<Integer, Integer> = HashMap::new();
        recursive(n, &mut memo);
    }
}