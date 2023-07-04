use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [[usize; n]; m],
    }
    let mut table = vec![vec![false; n]; n];
    for i in 0..n {
        table[i][i] = true;
    }
    for i in 0..m {
        for j in 1..n {
            let current = (a[i][j]-1, a[i][j-1]-1);
            table[current.0][current.1] = true;
            table[current.1][current.0] = true;
        }
    }
    let mut cnt = 0;
    for i in 0..n {
        for j in 0..n {
            if table[i][j] == false {
                cnt += 1;
            }
        }
    }

    println!("{}", cnt/2);
}

