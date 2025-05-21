use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
    }

    let mut table = vec![vec![false; n]; n];

    for _ in 0..m {
        input! {
            k: usize,
            x: [Usize1; k],
        }
        for i in 0..k - 1 {
            for j in i + 1..k {
                table[x[i]][x[j]] = true;
                table[x[j]][x[i]] = true;
            }
        }
    }

    let mut ans = true;
    for i in 0..n - 1 {
        for j in i + 1..n {
            if !table[i][j] {
                ans = false;
            }
        }
    }

    if ans {
        println!("Yes");
    } else {
        println!("No");
    }
}
