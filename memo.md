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

## 文字列操作系
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

### str to num
```rust
let str_num = "111";

// 7
println!("{}", u64::from_str_radix(str_num, 2).unwrap());
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

### 回文チェック
```rust
fn is_kaibun(s: &Vec<char>) -> bool{
    let s_len = s.len();
    for i in 0..s_len/2 {
        if s[i] != s[s_len-1-i] {
            return false;
        }
    }
    true
}
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

### 一部を新しく作る
```rust
let s = vec!['a', 'b', 'c', 'd', 'e']; 
let new_s = s[0..(s.len()-1)/2].iter().map(|&ch| ch).collect();
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

## 探索系
### bit全探索
```rust
for bit in 0..(1<<n) {
    for i in 0..n {
        if bit & (1 << i) == 0 {
            //
        } else {
            //
        }
    }
}
```
## 数学
### 最大公約数
```rust
fn gcd(x: usize, y: usize) -> usize {
    let mut xy = (x, y);
    while xy.0 >= 1 && xy.1 >= 1 {
        if xy.0 < xy.1 {
            xy.1 %= xy.0;
        } else {
            xy.0 %= xy.1;
        }
    }
    if xy.0 >= 1 {
        xy.0
    } else {
        xy.1
    }
}
```

### 最小公倍数
```rust
fn lcm(x: usize, y: usize) -> usize {
    let d = gcd(x,y);
    x / d * y
}
```

## うまく分類できないの
### 区間スケジュール問題
```rust
// https://atcoder.jp/contests/abc131/tasks/abc131_d
input! {
    n: usize,
    mut ab: [(usize, usize); n],
}
ab.sort_by(|(_,a), (_,b)| a.cmp(b));
let mut current_time = 0;
for (a, b) in ab.iter() {
    current_time += *a;
    if current_time > *b {
        println!("No");
        return;
    }
}
println!("Yes");
```
