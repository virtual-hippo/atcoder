use proconio::input;

fn main() {
    input! {
        m: usize,
        d: usize,
    }

    let ans = match (m, d) {
        (1, 7) | (3, 3) | (5, 5) | (7, 7) | (9, 9) => true,
        _ => false,
    };
    if ans {
        println!("Yes");
    } else {
        println!("No");
    }
}
