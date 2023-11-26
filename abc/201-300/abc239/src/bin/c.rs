use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        (x1, y1): (i64, i64),
        (x2, y2): (i64, i64),
    }
    let d = [
        (1, 2),
        (2, 1),
        (-1, 2),
        (2, -1),
        (1, -2),
        (-2, 1),
        (-1, -2),
        (-2, -1),
    ];
    for &(dx1, dy1) in d.iter() {
        for &(dx2, dy2) in d.iter() {
            if (x1 + dx1, y1 + dy1) == (x2 + dx2, y2 + dy2) {
                println!("Yes");
                return;
            }
        }
    }
    println!("No");
}
