use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let mut pizza = vec![false; 360];
    pizza[0] = true;
    for i in 0..n {
        let mut new = vec![false; 360];
        for j in 0..360 {
            new[(j + a[i]) % 360] = pizza[j];
        }
        pizza = new;
        pizza[0] = true;
    }
    pizza.push(true);
    let kirekomi = pizza
        .iter()
        .enumerate()
        .filter(|(_, v)| **v)
        .map(|(i, _)| i)
        .collect::<Vec<_>>();
    let mut ans = 0;
    for i in 1..kirekomi.len() {
        ans = ans.max(kirekomi[i] - kirekomi[i - 1]);
    }

    println!("{}", ans);
}
