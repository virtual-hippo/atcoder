use proconio::input;

fn is_ok((r1, c1): (i64, i64), (r2, c2): (i64, i64)) -> bool {
    r1 + c1 == r2 + c2 || r1 - c1 == r2 - c2 || (r1 - r2).abs() + (c1 - c2).abs() <= 3
}

fn main() {
    input! {
        (r1,c1): (i64, i64),
        (r2,c2): (i64, i64),
    }
    if (r1, c1) == (r2, c2) {
        println!("{}", 0);
        return;
    }
    if is_ok((r1, c1), (r2, c2)) {
        println!("{}", 1);
        return;
    }

    if (r1 - r2).abs() + (c1 - c2).abs() <= 6 {
        println!("{}", 2);
        return;
    }

    if (r1 + c1) % 2 == (r2 + c2) % 2 {
        println!("{}", 2);
        return;
    }

    for d in 1..4 {
        if r1 + d + c1 == r2 + c2 || r1 + c1 - d == r2 + c2 {
            println!("{}", 2);
            return;
        }
        if r1 + d - c1 == r2 - c2 || r1 - (c1 + d) == r2 - c2 {
            println!("{}", 2);
            return;
        }
    }

    println!("{}", 3);
}
