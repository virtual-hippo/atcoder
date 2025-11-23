use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        mut x: usize,
        mut y: usize,
        z: usize,
    }

    while y * z <= x {
        if y * z == x {
            println!("Yes");
            return;
        }

        x += 1;
        y += 1;
    }
    println!("No");
}
