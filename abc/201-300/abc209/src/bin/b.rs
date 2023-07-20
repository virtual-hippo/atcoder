use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
        a: [usize; n],
    }
    let sum = a
        .iter()
        .enumerate()
        .map(|(i, &v)| if i % 2 == 1 { v - 1 } else { v })
        .sum::<usize>();
    if x < sum {
        println!("No");
    } else {
        println!("Yes");
    }
}
