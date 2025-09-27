use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
    }

    let m = 3_usize.pow(n as u32);
    let mut ans = vec![vec!['_'; m]; m];
    rec(n, &mut ans, 0, 0);

    for i in 0..m {
        for j in 0..m {
            print!("{}", ans[i][j]);
        }
        println!();
    }
}

fn rec(k: usize, ans: &mut Vec<Vec<char>>, ii: usize, jj: usize) {
    if k == 0 {
        ans[ii][jj] = '#';
        return;
    }

    for i in 3 * ii..(3 * ii + 3) {
        for j in 3 * jj..(3 * jj + 3) {
            if i == 3 * ii + 1 && j == 3 * jj + 1 {
                print_white(k, ans, i, j);
            } else {
                rec(k - 1, ans, i, j);
            }
        }
    }
}

fn print_white(k: usize, ans: &mut Vec<Vec<char>>, ii: usize, jj: usize) {
    let n = 3_usize.pow(k as u32 - 1);

    let ii = ii * n;
    let jj = jj * n;

    for i in ii..(ii + n) {
        for j in jj..(jj + n) {
            ans[i][j] = '.';
        }
    }
}
