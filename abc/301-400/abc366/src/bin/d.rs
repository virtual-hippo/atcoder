// 3 次元累積和
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
    }
    let mut a = vec![vec![vec![0; n]; n]; n];
    for i in 0..n {
        for j in 0..n {
            for k in 0..n {
                input! {
                    aa: usize,
                }
                a[i][j][k] = aa;
            }
        }
    }

    let mut s = vec![vec![vec![0; n + 1]; n + 1]; n + 1];
    for i in 0..n {
        for j in 0..n {
            for k in 0..n {
                s[i + 1][j + 1][k + 1] = a[i][j][k]
            }
        }
    }

    // x 軸方向に加算
    for i in 0..n {
        for j in 0..n + 1 {
            for k in 0..n + 1 {
                s[i + 1][j][k] += s[i][j][k]
            }
        }
    }

    // y 軸方向に加算
    for i in 0..n + 1 {
        for j in 0..n {
            for k in 0..n + 1 {
                s[i][j + 1][k] += s[i][j][k]
            }
        }
    }

    // z 軸方向に加算
    for i in 0..n + 1 {
        for j in 0..n + 1 {
            for k in 0..n {
                s[i][j][k + 1] += s[i][j][k]
            }
        }
    }

    input! {
        q: usize,
    }
    for _ in 0..q {
        input! {
            (lx,rx): (usize,usize),
            (ly,ry): (usize,usize),
            (lz,rz): (usize,usize),
        }
        let sum_z = |rx: usize, ry: usize, (lz, rz): (usize, usize)| -> usize {
            // (X, Y, (0 ~ rz)) - (X, Y, (0 ~ lz))
            s[rx][ry][rz] - s[rx][ry][lz]
        };
        let sum_yz = |rx: usize, (ly, ry): (usize, usize), (lz, rz): (usize, usize)| -> usize {
            // (X, (0 ~ ry), (lz, rz)) - (X, (0 ~ ly), (lz, rz))
            sum_z(rx, ry, (lz, rz)) - sum_z(rx, ly, (lz, rz))
        };
        let sum_xyz = |(lx, rx): (usize, usize),
                       (ly, ry): (usize, usize),
                       (lz, rz): (usize, usize)|
         -> usize {
            // ((0 ~ rx), (ly, ry), (lz, rz)) - ((0 ~ lx), (ly, ry), (lz, rz))
            sum_yz(rx, (ly, ry), (lz, rz)) - sum_yz(lx, (ly, ry), (lz, rz))
        };
        let ans = sum_xyz((lx - 1, rx), (ly - 1, ry), (lz - 1, rz));
        println!("{}", ans);
    }
}
