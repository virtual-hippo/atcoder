use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
    }
    let mut ans = vec![vec!["T".to_string(); n]; n];
    let mut c = 1;

    let mut i = 0;
    let mut j = 0;

    let mut i_l = 0;
    let mut j_l = 0;
    let mut i_r = n;
    let mut j_r = n;
    let mut dir = 0;
    while !(i == n / 2 && j == n / 2) {
        if dir == 0 {
            for jj in j_l..j_r {
                ans[i][jj] = c.to_string();
                c += 1;
            }
            j = j_r - 1;
            i_l += 1;
        } else if dir == 1 {
            for ii in i_l..i_r {
                ans[ii][j] = c.to_string();
                c += 1;
            }
            i = i_r - 1;
            j_r -= 1;
        } else if dir == 2 {
            for jj in (j_l..j_r).rev() {
                ans[i][jj] = c.to_string();
                c += 1;
            }
            j = j_l;
            i_r -= 1;
        } else if dir == 3 {
            for ii in (i_l..i_r).rev() {
                ans[ii][j] = c.to_string();
                c += 1;
            }
            i = i_l;
            j_l += 1;
        }
        dir += 1;
        dir %= 4;
    }
    for i in 0..n {
        for j in 0..n {
            if i == n / 2 && j == n / 2 {
                print!("T ");
            } else {
                print!("{} ", ans[i][j]);
            }
        }
        println!("");
    }
}
