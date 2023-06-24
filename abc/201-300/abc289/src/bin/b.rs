use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; m],
    }
    let mut b = vec![false; n+1];
    for i in 0..m {
        b[a[i]] = true;
    }
    let mut current = vec![];
    for i in 1..n+1 {
        current.push(i);
        if b[i] == false {
            current.sort_by(|a,b| b.cmp(a));
            for &v in current.iter() {
                print!("{} ", v);
            }
            current.clear()
        }
    }
}

