use proconio::input;

fn main() {
    input! {
        n: u64,
    }
    if n == 1 {
        println!("0");
        return;
    }

    let mut keta = 1;
    let mut sosu = 1;
    let mut goal = n - 1;
    loop {
        let diff = 9 * 10_u64.pow((keta - 1) / 2);
        if sosu + diff <= n {
            sosu += diff;
            goal -= diff;
            keta += 1;
            continue;
        }
        goal += diff / 9 - 1;
        let mut s = goal.to_string();
        let rs: String = s.chars().rev().collect();
        if keta % 2 == 1 {
            s.pop();
        }
        let ans = format!("{}{}", s, rs);
        println!("{}", ans);
        return;
    }
}
