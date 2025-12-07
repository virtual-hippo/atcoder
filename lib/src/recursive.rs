/// メモ化再帰
/// https://atcoder.jp/contests/abc275/tasks/abc275_d
pub mod memo_recursive {
    use std::collections::HashMap;

    fn recursive(x: u64, memo: &mut HashMap<u64, u64>) -> u64 {
        if x == 0 {
            1
        } else {
            if let Some(val) = memo.get(&x) {
                *val
            } else {
                let a = x / 2;
                let b = x / 3;
                let res = recursive(a, memo) + recursive(b, memo);
                memo.insert(x, res);
                res
            }
        }
    }
    pub fn run() {
        let mut memo: HashMap<u64, u64> = HashMap::new();
        let n = 100;
        recursive(n, &mut memo);
    }
}
