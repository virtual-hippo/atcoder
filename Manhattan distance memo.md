# マンハッタン距離の最大値 O(N) 解法

## 原理

`|x_i - x_j| + |y_i - y_j|` の絶対値を外すと、符号パターンが 2^次元 通り生じる。
同じ符号パターンで変換した値の `max - min` を全パターンで取れば答え。

| j   | 変換  |
| --- | ----- |
| 0   | +x +y |
| 1   | -x +y |
| 2   | +x -y |
| 3   | -x -y |

## コード (Rust)

```rust
fn get_val(j: usize, x: i64, y: i64) -> i64 {
    let mut val = 0;
    if j & 1 == 0 { val += x; } else { val -= x; }
    if j & 2 == 0 { val += y; } else { val -= y; }
    val
}

fn max_manhattan_distance(points: &[(i64, i64)]) -> i64 {
    let mut ans = 0;
    for j in 0..4 {
        let (mut ma, mut mi) = (i64::MIN, i64::MAX);
        for &(x, y) in points {
            let v = get_val(j, x, y);
            ma = ma.max(v);
            mi = mi.min(v);
        }
        ans = ans.max(ma - mi);
    }
    ans
}
```

## 3 次元版

```rust
fn get_val_3d(j: usize, x: i64, y: i64, z: i64) -> i64 {
    let mut val = 0;
    if j & 1 == 0 { val += x; } else { val -= x; }
    if j & 2 == 0 { val += y; } else { val -= y; }
    if j & 4 == 0 { val += z; } else { val -= z; }
    val
}

// j in 0..8 でループ
```

## 計算量

- 愚直: O(N²)
- この手法: O(2^D × N)　※D: 次元数
