use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        mut a: [usize;n],
        b: [usize; m],
    }
    let mut pointer = 0;
    let mut c = vec![];
    for i in 0..n {
        c.push(a[i])
    }
    for i in 0..m {
        c.push(b[i])
    }
    a.sort();
    c.sort();
    let mut cnt = 0;
    for v in c.iter() {
        if pointer == n {
            break;
        }
        if *v == a[pointer] {
            cnt += 1;
            pointer += 1;
        } else {
            cnt = 0;
        }
        if cnt == 2 {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
