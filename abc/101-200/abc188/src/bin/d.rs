use std::collections::BTreeMap;
use proconio::input;

// バチャ中に書いたのを解説を参考に士ながら修正したもの
fn main() {
    input! {
        n: usize,
        c: usize,
    }
    let mut b_map = BTreeMap::new();
    for _ in 0..n {
        input! {
            a: usize,
            b: usize,
            cc: i64,
        }
        *b_map.entry(a).or_insert(0) += cc;
        *b_map.entry(b+1).or_insert(0) -= cc;
    }


    let mut prev = (0, 0);
    let mut ans = 0;
    let mut fee = 0_i64;
    for (k, v) in b_map.iter() {
        ans += fee.min(c as i64) * ((*k - prev.0) as i64);
        prev = (*k, *v);
        fee += *v;
    }
    
    println!("{}", ans);
}

// 解説見て書いたやつ
// use proconio::input;

// fn main() {
//     input! {
//         n: usize,
//         c: i64,
//     }
//     let mut event = Vec::with_capacity(2 * n);
//     for _ in 0..n {
//         input! {
//             a: usize,
//             b: usize,
//             cc: i64,
//         }
//         event.push((a-1, cc));
//         event.push((b, -cc));
//     }
//     event.sort();

//     let mut ans = 0;
//     let mut t = 0;
//     let mut fee = 0;

//     for &(x, y) in event.iter() {
//         if x != t {
//             ans += c.min(fee) * ((x-t) as i64);
//             t = x;
//         }
//         fee += y;
//     }

//     println!("{}", ans);
// }


