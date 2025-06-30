use ac_library::*;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: u64,
        m: u64,
    }
    let n = n + 1;

    let mut ans = ModInt998244353::new(0);
    for i in 0..60 {
        let x = (m >> i) & 1;
        if x == 0 {
            continue;
        }

        //
        // Copilot
        //
        // 高速化: n個の数値でjビット目が1になる個数を数える
        let cnt = (n >> (i + 1)) << i; // サイクルに含まれる 1 の数
        let rem = n & ((1 << (i + 1)) - 1); // サイクル外の余り部分の個数
        let cnt = cnt + rem.saturating_sub(1 << i); // サイクル外の余り部分の個数の内 0 i bit 目が 0 である個数を取り除く
        ans += cnt;

        //
        // YouTube 解説放送
        //
        // let p = 1 << (i + 1);
        // let r = n % p;
        // ans += (n - r) / 2;

        // if r >= (1 << i) {
        //     ans += r - (1 << i);
        // }
    }

    println!("{}", ans);
}
