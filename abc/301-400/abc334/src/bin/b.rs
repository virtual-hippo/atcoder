use proconio::{fastout, input};

fn get_a_num(a: i128, m: i128, target: i128) -> i128 {
    let diff = (target - a).abs();
    diff / m + 1
}

#[fastout]
fn main() {
    input! {
        a: i128,
        m: i128,
        l: i128,
        r: i128,
    }
    let a_to_l = get_a_num(a, m, l);
    let a_to_r = get_a_num(a, m, r);
    if a < l {
        if (l - a).abs() % m == 0 {
            println!("{}", a_to_r - a_to_l + 1);
        } else {
            println!("{}", a_to_r - a_to_l);
        }
    } else if l <= a && a <= r {
        println!("{}", a_to_r + a_to_l - 1);
    } else {
        if (r - a).abs() % m == 0 {
            println!("{}", a_to_l - a_to_r + 1);
        } else {
            println!("{}", a_to_l - a_to_r);
        }
    }
}
