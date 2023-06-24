use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    }
    if k == 1 {
        println!("Yes");
        return;
    }
    let mut swapped = vec![vec![]; k];
    for i in 0..n {
        swapped[i%k].push(a[i]);
    }
    for i in 0..k {
        swapped[i].sort_by(|a,b| b.cmp(a));
    }
    let mut b = vec![0; n];
    for i in 0..n {
        b[i] = swapped[i%k].pop().unwrap();
    }
    for i in 1..n {
        if b[i] < b[i-1] {
            println!("No");
            return;
        }
    }
    println!("Yes");
}

