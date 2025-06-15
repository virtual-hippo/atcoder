use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a: u64,
        b: u64,
        c: u64,
    }

    let one = c.count_ones() as u64;

    if a + b < one || (a + b - one) % 2 != 0 {
        println!("-1");
        return;
    }

    let to1 = (a + b - one) / 2;

    let mut a = a - to1;
    let mut b = b - to1;
    let mut to1 = to1;

    let mut x = 0;
    let mut y = 0;

    for i in 0..61 {
        if (c >> i) & 1 == 1 {
            if a > 0 {
                a -= 1;
                x |= 1 << i;
                continue;
            }
            if b > 0 {
                b -= 1;
                y |= 1 << i;
                continue;
            }
            // (x,y) = (1,0) or (0,1)
        } else {
            if to1 > 0 {
                x |= 1 << i;
                y |= 1 << i;
                to1 -= 1;
                continue;
            } else {
                //
            }
        }
    }

    if a == 0 && b == 0 && to1 == 0 && x ^ y == c {
        println!("{} {}", x, y);
    } else {
        println!("-1");
    }
}
