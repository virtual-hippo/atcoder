use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        (v1, v2, v3): (usize, usize, usize),
    }
    let all = 7 * 7 * 7 * 3;
    if v1 + v2 * 2 + v3 * 3 == all {
        println!("Yes");
    } else {
        println!("No");
        return;
    }
    if v1 == all {
        if v2 != 0 || v3 != 0 {
            println!("No");
            return;
        }
    }
    if v3 == 0 {
        for i in 0..7 {
            for j in i..7 {
                for j in i..7 {
                    //
                }
            }
        }
    }
}
