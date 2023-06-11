use proconio::input;

fn main() {
    input! {
        (n,m): (usize, usize),
        b: [[usize; m]; n],
    }
    for i in 0..n {
        for j in 0..m {
            if 0 < j && b[i][j-1] + 1 != b[i][j] {
                println!("No");
                return;
            }
            if 0 < i && b[i-1][j] + 7 != b[i][j] {
                println!("No");
                return;
            }
            if b[i][j] % 7 == 0 && j != m - 1 {
                println!("No");
                return;
            }
            if b[i][j] % 7 == 1 && j != 0 {
                println!("No");
                return;
            }
        }
    }
    println!("Yes");
}

