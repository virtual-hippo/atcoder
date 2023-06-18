# Rustで競プロする際のメモ

## 良く使うライブラリ
```rust
use proconio::input;
use std::collections::HashSet;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::collections::BinaryHeap;
use proconio::marker::Chars;
use itertools::Itertools; // p.iter().permutations(n)
use regex::Regex; // ←あまり使わん
```

## cast 系
### char to num
```rust
let c: char = '5';
let num: i32 = c as i32 - 48;
```

## 文字列出力系
### 大文字変換
```rust
let mut s = String::from("Grüße, Jürgen ❤");
s.make_ascii_uppercase();
assert_eq!("GRüßE, JüRGEN ❤", s);
```

### 小数点
```rust
 // 少数第7まで表示
println!("{:.7}", n);
```

### N進数
```rust
// 16進数0埋め, 文字列長2
println!("{:>02X}", n); 

// 8進数にしたときに7が含まれるか
for i in 1..n+1 {
    let d = format!("{}", i);
    let o = format!("{:>03o}", i);
    if d.contains("7") || o.contains("7") {
        //
    } else {
        cnt +=1;
    }
}
```

### アルファベットを順に出力
```rust
// https://www.k-cube.co.jp/wakaba/server/ascii_code.html
// ABCDEFGHIJKLMNOPQRSTUVWXYZ
let large_a = 65_u8;
(0..26).for_each(|i| print!("{}", (large_a + i) as char));

// abcdefghijklmnopqrstuvwxyz
let small_a = 97_u8;
(0..26).for_each(|i| print!("{}", (small_a + i) as char));
```

## HashMap
### entry API
```rust
 *map.entry(a[i]).or_insert(0) += 1;
 map.entry(s[j]).or_insert_with(|| vec![]).push(j);
```

## Vec
### sort
```rust
// 昇順
vec.sort();

// 降順
vec.sort_by(|a, b| b.cmp(a));

// ソート前の位置を保持したままソート
let mut vec_with_ind = vec.iter().enumerate().map(|(i,x)| (i, *x)).collect::<Vec<(usize,usize)>>();
vec_with_ind.sort_by(|a, b| a.1.cmp(&b.1));
```

### min, max
```rust
let min = v.iter().min().unwrap();
let max = v.iter().max().unwrap();

// fold()を使う場合
let min = x_vec.iter().fold(dummy_max, |min, x| std::cmp::min(min, *x));
let max = x_vec.iter().fold(dummy_min, |max, x| std::cmp::max(max, *x));
```

### sum
```rust
let sum = v.iter().sum::<usize>();

// fold()を使う場合
let sum = v.iter().fold(0, |sum, x| sum + *x);
```

## DP
### 部分和DP
```rust
// a: Vec<usize>
// s: aの合計
let mut dp = vec![vec![false; s+1]; n+1];
dp[0][0] = true;
for i in 1..n+1 {
    for j in 0..s+1 {
        if dp[i-1][j] {
            dp[i][j] = true;
        }
        if j < a[i-1] {
            // 何もしない
        } else if dp[i-1][j-a[i-1]] {
            dp[i][j] = true;
        }
    }
}
```

## 座標系
### 距離の2乗計算(2次元)
```rust
fn calc_d(pos1: (i64, i64), pos2: (i64, i64)) -> i64 {
    (pos2.0 - pos1.0).pow(2) + (pos2.1 - pos1.1).pow(2) 
}
```

### 距離の2乗計算(N次元)
```rust
fn calc_d(pos1: &Vec<i64>, pos2: &Vec<i64>) -> i64 {
    (0..pos1.len()).fold(0, |sum, x| sum + (pos2[x]-pos1[x]).pow(2))
}
```

