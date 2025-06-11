use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        x: i64,
        y: i64,
        z: i64,
    }

    if 0 < x && x < y {
        println!("{}", x);
        return;
    }

    if y < 0 && 0 < x {
        println!("{}", x);
        return;
    }

    if 0 < y && y < x {
        if 0 < z && z < y {
            println!("{}", x);
            return;
        }
        if z < 0 {
            println!("{}", x - z - z);
            return;
        }
    }

    let x = -x;
    let y = -y;
    let z = -z;

    if 0 < x && x < y {
        println!("{}", x);
        return;
    }

    if y < 0 && 0 < x {
        println!("{}", x);
        return;
    }

    if 0 < y && y < x {
        if 0 < z && z < y {
            println!("{}", x);
            return;
        }
        if z < 0 {
            println!("{}", x - z - z);
            return;
        }
    }

    println!("-1");
}
