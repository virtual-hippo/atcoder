use proconio::input;

fn main() {
    input! {
        n: usize,
    }
    let mut vec = Vec::with_capacity(n);
    for _ in 0..n {
        input! {
            s: String,
            t: String,
        }
        vec.push((s, t));
    }
    let ans = (0..n)
        .filter(|&i| {
            (0..n)
                .filter(|&j| i != j)
                .filter(|&j| vec[i].0 == vec[j].0 || vec[i].0 == vec[j].1)
                .count()
                > 0
        })
        .filter(|&i| {
            (0..n)
                .filter(|&j| i != j)
                .filter(|&j| vec[i].1 == vec[j].1 || vec[i].1 == vec[j].0)
                .count()
                > 0
        })
        .count()
        > 0;
    if ans {
        println!("No");
    } else {
        println!("Yes");
    }
}
