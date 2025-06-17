use proconio::input;

fn main() {
    input! {
        x: i64,
        y: i64,
        z: i64,
    }

    let (x, y, z) = if y < 0 { (-x, -y, -z) } else { (x, y, z) };

    if x < y {
        println!("{}", x.abs());
        return;
    }
    if y < z {
        println!("{}", -1);
        return;
    }

    if z < 0 {
        println!("{}", z.abs() * 2 + x);
        return;
    } else {
        println!("{}", x.abs());
        return;
    }
}
