use proconio::input;

fn main() {
    input! {
        (n,m,d): (usize, usize, u64),
        mut a: [u64; n],
        mut b: [u64; m],
    }
    a.sort();
    b.sort();
    loop {
        if a.len() == 0 || b.len() == 0 {
            break;
        }
        let a_max = a[a.len()-1];
        let b_max = b[b.len()-1];

        if a_max < b_max{
            if b_max - a_max <= d {
                println!("{}", b_max + a_max);
                return
            } else {
                b.pop();
            }
        } else if a_max > b_max{
            if a_max - b_max <= d {
                println!("{}", b_max + a_max);
                return
            } else {
                a.pop();
            }
        } else {
            println!("{}", b_max + a_max);
            return
        }
    }
    println!("-1");
}
